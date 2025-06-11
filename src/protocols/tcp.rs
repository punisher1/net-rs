use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{channel, Receiver, Sender},
        RwLock,
    },
};

use crate::protocols::common::{ConnectionInfo, Message, MessageDirection, MessageType, ProtocolHandler};

/// TCP 服务器处理器
pub struct TcpServerHandler {
    /// 本地地址
    local_addr: SocketAddr,
    /// 连接的客户端
    clients: Arc<RwLock<HashMap<String, TcpClientInfo>>>,
    /// 控制通道 (用于停止服务器)
    control_tx: Option<Sender<()>>,
    /// UI到服务器发送通道
    ui_to_server_tx: Option<Sender<Message>>,
    /// UI到服务器接收通道
    ui_to_server_rx: Option<Receiver<Message>>,
    /// 服务器到UI发送通道
    server_to_ui_tx: Option<Sender<Message>>,
    /// 运行状态
    running: bool,
}

/// TCP 客户端信息
struct TcpClientInfo {
    /// 远程地址
    addr: SocketAddr,
    /// 发送通道
    tx: Sender<Bytes>,
}

impl TcpServerHandler {
    /// 创建新的TCP服务器处理器
    pub fn new(local_addr: SocketAddr) -> Self {
        Self {
            local_addr,
            clients: Arc::new(RwLock::new(HashMap::new())),
            control_tx: None,
            ui_to_server_tx: None,
            ui_to_server_rx: None,
            server_to_ui_tx: None,
            running: false,
        }
    }
}

#[async_trait]
impl ProtocolHandler for TcpServerHandler {
    async fn start(&mut self) -> Result<()> {
        // 创建消息通道
        let (ui_to_server_tx, ui_to_server_rx) = channel::<Message>(100);
        let (control_tx, mut control_rx) = channel::<()>(1);

        self.ui_to_server_tx = Some(ui_to_server_tx);
        self.ui_to_server_rx = Some(ui_to_server_rx);
        self.control_tx = Some(control_tx);
        self.running = true;

        // 绑定监听地址
        let listener = TcpListener::bind(self.local_addr).await?;

        let clients = Arc::clone(&self.clients);
        let server_to_ui_tx = self.server_to_ui_tx.clone();

        // 启动服务器监听任务
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // 处理新的客户端连接
                    result = listener.accept() => {
                        match result {
                            Ok((stream, addr)) => {
                                // 为每个客户端创建处理任务
                                let client_id = addr.to_string();
                                let (client_tx, mut client_rx) = channel::<Bytes>(100);

                                // 保存客户端信息
                                {
                                    let mut clients_lock = clients.write().await;
                                    clients_lock.insert(client_id.clone(), TcpClientInfo {
                                        addr,
                                        tx: client_tx,
                                    });
                                }

                                // 通知UI有新连接
                                if let Some(ref server_to_ui_sender) = server_to_ui_tx {
                                    let _ = server_to_ui_sender.send(Message {
                                        content: MessageType::ClientConnected,
                                        direction: MessageDirection::Received,
                                        timestamp: chrono::Local::now(),
                                        connection_info: Some(ConnectionInfo {
                                           remote_addr: addr,
                                           connection_id: client_id.clone(),
                                        }),
                                    }).await;
                                }

                                // 分离读写流
                                let (mut read_half, mut write_half) = stream.into_split();
                                let clients_for_read = Arc::clone(&clients);
                                let server_to_ui_tx_for_read = server_to_ui_tx.clone();

                                // 处理客户端读取任务
                                let read_client_id = client_id.clone();
                                tokio::spawn(async move {
                                    let mut buffer = vec![0u8; 4096];
                                    loop {
                                        match read_half.read(&mut buffer).await {
                                            Ok(0) => {
                                                // 从客户端列表中移除
                                                {
                                                    let mut clients_lock = clients_for_read.write().await;
                                                    clients_lock.remove(&read_client_id);
                                                }

                                                // 通知UI连接断开
                                                if let Some(ref server_to_ui_sender) = server_to_ui_tx_for_read {
                                                    let _ = server_to_ui_sender.send(Message {
                                                        direction: MessageDirection::Received,
                                                        content: MessageType::ClientDisconnected,
                                                        timestamp: chrono::Local::now(),
                                                        connection_info: Some(ConnectionInfo {
                                                            remote_addr: addr,
                                                            connection_id: read_client_id.clone(),
                                                        }),
                                                    }).await;
                                                }
                                                break;
                                            }
                                            Ok(n) => {
                                                // 接收到数据
                                                let data = &buffer[..n];
                                                let message_content = String::from_utf8_lossy(data).to_string();

                                                // 发送到UI
                                                if let Some(ref server_to_ui_sender) = server_to_ui_tx_for_read {
                                                    let _ = server_to_ui_sender.send(Message {
                                                        direction: MessageDirection::Received,
                                                        content: MessageType::Text(message_content.clone()),
                                                        timestamp: chrono::Local::now(),
                                                        connection_info: Some(ConnectionInfo {
                                                            remote_addr: addr,
                                                            connection_id: read_client_id.clone(),
                                                        }),
                                                    }).await;
                                                }
                                            }
                                            Err(e) => {
                                                println!("读取客户端 {} 数据时出错: {}", addr, e);
                                                break;
                                            }
                                        }
                                    }

                                    drop(read_half);
                                });

                                // 处理客户端写入任务
                                tokio::spawn(async move {
                                    while let Some(data) = client_rx.recv().await {
                                        if let Err(e) = write_half.write_all(&data).await {
                                            println!("向客户端 {} 发送数据时出错: {}", addr, e);
                                            break;
                                        }
                                    }

                                    drop(write_half);
                                });
                            }
                            Err(e) => {
                                println!("接受客户端连接时出错: {}", e);
                            }
                        }
                    }

                    // 处理停止信号 - 明确检查是否收到了信号
                    result = control_rx.recv() => {
                        match result {
                            Some(_) => {
                                println!("TCP 服务器收到停止信号，正在停止...");
                                break;
                            }
                            None => {
                                // 发送方已关闭，这通常意味着服务器应该停止
                                println!("TCP 服务器控制通道已关闭，正在停止...");
                                break;
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        if self.running {
            // 发送停止信号
            if let Some(ref control_tx) = self.control_tx {
                let _ = control_tx.send(()).await;
            }
            self.running = false;
            // 清理资源
            self.control_tx = None;
        }
        Ok(())
    }

    async fn send_message(&mut self, message: MessageType, target: Option<String>) -> Result<()> {
        todo!("Implement TCP server send message")
    }

    fn get_ui_to_server_sender(&self) -> Option<Sender<Message>> {
        self.ui_to_server_tx.clone()
    }

    fn set_server_to_ui_sender(&mut self, sender: Sender<Message>) {
        self.server_to_ui_tx = Some(sender);
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn get_connections(&self) -> Vec<ConnectionInfo> {
        todo!("Implement get_connections for TCP server")
    }

    fn protocol_name(&self) -> &'static str {
        "TCP Server"
    }
}

/// TCP 客户端处理器
pub struct TcpClientHandler {
    /// 本地地址
    local_addr: SocketAddr,
    /// 远程服务器地址
    remote_addr: SocketAddr,
    /// TCP 流
    stream: Option<TcpStream>,
    /// 控制通道 (用于停止客户端)
    control_tx: Option<Sender<()>>,
    /// 消息接收通道
    ui_to_server_rx: Option<Receiver<Message>>,
    /// 消息发送通道
    ui_to_server_tx: Option<Sender<Message>>,
    /// UI消息发送通道
    server_to_ui_tx: Option<Sender<Message>>,
    /// 运行状态
    running: bool,
}

impl TcpClientHandler {
    /// 创建新的TCP客户端处理器
    pub fn new(local_addr: SocketAddr, remote_addr: SocketAddr) -> Self {
        Self {
            local_addr,
            remote_addr,
            stream: None,
            control_tx: None,
            ui_to_server_rx: None,
            ui_to_server_tx: None,
            server_to_ui_tx: None,
            running: false,
        }
    }
}

#[async_trait]
impl ProtocolHandler for TcpClientHandler {
    async fn start(&mut self) -> Result<()> {
        todo!("Implement TCP client start")
    }

    async fn stop(&mut self) -> Result<()> {
        todo!("Implement TCP client stop")
    }

    async fn send_message(&mut self, message: MessageType, target: Option<String>) -> Result<()> {
        todo!("Implement TCP client send message")
    }

    fn get_ui_to_server_sender(&self) -> Option<Sender<Message>> {
        self.ui_to_server_tx.clone()
    }

    fn set_server_to_ui_sender(&mut self, sender: Sender<Message>) {
        self.server_to_ui_tx = Some(sender);
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn get_connections(&self) -> Vec<ConnectionInfo> {
        todo!("Implement get_connections for TCP client")
    }

    fn protocol_name(&self) -> &'static str {
        "TCP Client"
    }
}
