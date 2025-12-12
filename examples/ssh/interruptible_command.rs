use ssh2::Session;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 建立 TCP 连接和 SSH 会话
    let tcp = TcpStream::connect("192.168.0.100:22")?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    // 2. 认证 (这里以密码为例)
    session.userauth_password("pi", "Simple@1993")?;
    println!("SSH 连接建立成功...");
    println!("按 Ctrl+C 中断命令执行...");

    // 3. 创建一个原子标志用于中断控制
    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_clone = should_stop.clone();

    // 设置 Ctrl+C 信号处理
    ctrlc::set_handler(move || {
        println!("\n收到中断信号，正在停止命令...");
        should_stop_clone.store(true, Ordering::SeqCst);
    })?;

    // 4. 打开一个通道并执行命令
    let mut channel = session.channel_session()?;
    channel.exec("ping 127.0.0.1")?;

    // 5. 创建一个 BufReader 来包装 channel
    let mut reader = BufReader::new(&mut channel);
    let mut line = String::new();

    // 6. 循环读取输出，直到收到中断信号或命令结束
    loop {
        // 检查是否需要停止
        if should_stop.load(Ordering::SeqCst) {
            // 向SSH通道发送中断信号（ASCII 3，对应Ctrl+C）
            println!("向远程发送中断信号...");
            channel.write(&[3])?;
            channel.flush()?;
            
            // 等待片刻让命令有时间处理中断
            thread::sleep(Duration::from_millis(500));
            
            // 关闭通道
            channel.close()?;
            break;
        }

        // 使用非阻塞方式读取输出（轮询）
        // 尝试读取一行
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // 没有数据且通道关闭
                    break;
                }
                // 输出读取到的内容
                print!("{}", line);
                std::io::stdout().flush()?;
                line.clear();
            }
            Err(_) => {
                // 如果读取失败（可能是因为没有可用数据），短暂睡眠后继续
                thread::sleep(Duration::from_millis(100));
                continue;
            }
        }
    }

    // 7. 等待命令执行完成并获取退出状态
    channel.wait_close()?;
    println!("\n命令执行完毕，退出状态: {}", channel.exit_status()?);

    Ok(())
}