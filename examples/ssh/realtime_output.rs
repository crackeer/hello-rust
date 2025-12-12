use ssh2::Session;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
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

    // 3. 打开一个通道并执行命令
    let mut channel = session.channel_session()?;
    channel.request_pty(
        "xterm",
        None,
        None
    )?;
    // 创建一个原子标志，用于通知主线程退出
    let interrupted = Arc::new(AtomicBool::new(false));
    let interrupted_clone = interrupted.clone();
    ctrlc::set_handler(move || {
        println!("\n收到本地中断信号");
        interrupted_clone.store(true, Ordering::SeqCst);
    })?;
    // ⚠️ 注意：这里不使用 channel.exec("your_command") 然后 read_to_end
    // 而是直接操作 channel 的读取流
    channel.exec("top")?; // 例如 "ping 127.0.0.1" 或 "tail -f /var/log/syslog"

    let mut buffer = [0u8; 1024];
    loop {
        if interrupted.load(Ordering::SeqCst) {
            println!("收到中断信号，发送信号准备退出...");
            // 可选：发送中断字符到远程（但需在主线程操作 channel！）
            let _ = channel.write_all(&[3]); // 模拟 Ctrl+C
            let _ = channel.send_eof();
            interrupted.store(false, Ordering::Relaxed); // 避免重复发送
        }

        // 直接从 channel 读，不通过 BufReader
        match channel.read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                std::io::stdout().write_all(&buffer[..n])?;
                std::io::stdout().flush()?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(50));
                continue;
            }
            Err(e) => {
                eprintln!("读取错误: {}", e);
                break;
            }
        }

        if channel.eof() {
            break;
        }
    }

    // 6. 等待命令执行完成并获取退出状态
    channel.wait_close()?;

    println!("\n命令执行完毕，退出状态: {}", channel.exit_status()?);

    Ok(())
}
