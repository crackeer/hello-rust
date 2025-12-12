use ssh2::Session;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

/// SSH信号类型枚举
#[derive(Debug)]
enum SshSignal {
    Interrupt,    // Ctrl+C (ASCII 3)
    Quit,         // Ctrl+\ (ASCII 28)
    Suspend,      // Ctrl+Z (ASCII 26)
    Break,        // Ctrl+Break (ASCII 21)
}

impl SshSignal {
    /// 将信号转换为对应的ASCII值
    fn to_ascii(&self) -> u8 {
        match self {
            SshSignal::Interrupt => 3,
            SshSignal::Quit => 28,
            SshSignal::Suspend => 26,
            SshSignal::Break => 21,
        }
    }
    
    /// 向SSH通道发送信号
    fn send_to_channel(&self, channel: &mut ssh2::Channel) -> Result<(), Box<dyn std::error::Error>> {
        println!("发送信号: {:?}", self);
        channel.write(&[self.to_ascii()])?;
        channel.flush()?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 建立 TCP 连接和 SSH 会话
    let tcp = TcpStream::connect("192.168.0.100:22")?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    // 2. 认证 (这里以密码为例)
    session.userauth_password("pi", "Simple@1993")?;
    println!("SSH 连接建立成功...");

    // 3. 创建一个原子标志用于中断控制
    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_clone = should_stop.clone();

    // 设置 Ctrl+C 信号处理
    ctrlc::set_handler(move || {
        println!("\n收到本地中断信号");
        should_stop_clone.store(true, Ordering::SeqCst);
    })?;

    // 4. 打开一个通道并执行长时间运行的命令
    let channel = session.channel_session()?;
    println!("执行命令: ping 127.0.0.1");
    
    // 将channel放入Arc<Mutex>中以安全地在线程间共享
    let shared_channel = Arc::new(Mutex::new(channel));
    
    // 在互斥锁保护下执行命令
    { 
        let mut channel = shared_channel.lock().unwrap();
        channel.exec("ping 127.0.0.1")?;
    }

    // 5. 启动一个线程来演示自动中断方式
    let should_stop_clone2 = should_stop.clone();
    let shared_channel_clone = shared_channel.clone();
    
    thread::spawn(move || {
        // 运行5秒后自动发送中断信号
        thread::sleep(Duration::from_secs(5));
        
        if !should_stop_clone2.load(Ordering::SeqCst) {
            println!("\n5秒时间到，自动发送中断信号...");
            
            // 获得通道的锁并发送信号
            if let Ok(mut channel) = shared_channel_clone.lock() {
                let _ = SshSignal::Interrupt.send_to_channel(&mut channel);
                
                // 等待命令处理中断
                thread::sleep(Duration::from_millis(1000));
                
                // 发送退出信号（如果需要）
                let _ = SshSignal::Quit.send_to_channel(&mut channel);
            }
            
            should_stop_clone2.store(true, Ordering::SeqCst);
        }
    });

    // 6. 创建一个读取线程来处理输出
    let should_stop_clone3 = should_stop.clone();
    let shared_channel_clone2 = shared_channel.clone();
    
    thread::spawn(move || {
        let mut line = String::new();
        
        while !should_stop_clone3.load(Ordering::SeqCst) {
            // 尝试获取通道锁
            if let Ok(mut channel) = shared_channel_clone2.try_lock() {
                // 创建BufReader读取输出
                let mut reader = BufReader::new(&mut *channel);
                
                // 尝试读取一行
                match reader.read_line(&mut line) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            // 没有数据且通道关闭
                            break;
                        }
                        // 输出读取到的内容
                        print!("{}", line);
                        std::io::stdout().flush().unwrap();
                        line.clear();
                    }
                    Err(_) => {
                        // 如果读取失败（可能是因为没有可用数据），短暂睡眠后继续
                        thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                }
            } else {
                // 无法获取锁，短暂睡眠后重试
                thread::sleep(Duration::from_millis(100));
            }
        }
    });

    // 7. 主线程等待，直到收到中断信号
    while !should_stop.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(100));
    }

    // 8. 等待命令执行完成并获取退出状态
    if let Ok(mut channel) = shared_channel.lock() {
        channel.wait_close().unwrap();
        println!("\n命令执行完毕，退出状态: {}", channel.exit_status().unwrap());
    }

    Ok(())
}