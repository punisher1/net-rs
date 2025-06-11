use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Local};
use h2::server;
use std::net::SocketAddr;
use tokio::sync::mpsc::{Receiver, Sender};

use crate::protocols::tcp::TcpServerHandler;

/// 传输消息类型
#[derive(Debug, Clone)]
pub enum MessageType {
    Text(String),
    Binary(Bytes),
    Hex(String),
}

/// 消息方向
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageDirection {
    /// 收到的消息
    Received,
    /// 发送的消息
    Sent,
}

/// 连接信息
#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    /// 远程地址
    pub remote_addr: SocketAddr,
    /// 连接 ID (用于区分不同客户端)
    pub connection_id: String,
}

/// 消息
#[derive(Debug, Clone)]
pub struct Message {
    /// 消息内容
    pub content: MessageType,
    /// 消息方向
    pub direction: MessageDirection,
    /// 时间戳
    pub timestamp: DateTime<Local>,
    /// 连接信息
    pub connection_info: Option<ConnectionInfo>,
}

impl Message {
    /// 创建新的接收消息
    pub fn new_received(content: MessageType, connection_info: Option<ConnectionInfo>) -> Self {
        Self {
            content,
            direction: MessageDirection::Received,
            timestamp: Local::now(),
            connection_info,
        }
    }

    /// 创建新的发送消息
    pub fn new_sent(content: MessageType, connection_info: Option<ConnectionInfo>) -> Self {
        Self {
            content,
            direction: MessageDirection::Sent,
            timestamp: Local::now(),
            connection_info,
        }
    }
}

/// 通讯协议处理接口
#[async_trait]
pub trait ProtocolHandler {
    /// 启动协议处理器
    async fn start(&mut self) -> Result<()>;

    /// 停止协议处理器
    async fn stop(&mut self) -> Result<()>;

    /// 发送消息
    async fn send_message(&mut self, message: MessageType, target: Option<String>) -> Result<()>;

    /// 获取UI向服务端的发送通道
    fn get_ui_to_server_sender(&self) -> Option<Sender<Message>>;

    /// 设置服务端向UI的发送通道
    fn set_server_to_ui_sender(&mut self, sender: Sender<Message>);

    /// 处理程序是否正在运行
    fn is_running(&self) -> bool;

    /// 获取当前连接信息
    fn get_connections(&self) -> Vec<ConnectionInfo>;

    /// 获取协议名称
    fn protocol_name(&self) -> &'static str;
}

/// 创建协议处理器工厂函数
pub async fn create_protocol_handler(
    protocol: &str,
    is_server: bool,
    server_to_ui_tx: Option<Sender<Message>>,
    local_addr: SocketAddr,
    remote_addr: Option<SocketAddr>,
) -> Result<Box<dyn ProtocolHandler + Send + Sync>> {
    match (protocol.to_lowercase().as_str(), is_server) {
        ("tcp", true) => {
            let mut handler = TcpServerHandler::new(local_addr);
            handler.set_server_to_ui_sender(server_to_ui_tx.unwrap());
            handler.start().await?;
            Ok(Box::new(handler))
        }
        ("tcp", false) => {
            todo!("Create TCP client handler")
        }
        ("udp", true) => {
            todo!("Create UDP server handler")
        }
        ("udp", false) => {
            todo!("Create UDP client handler")
        }
        ("websocket", true) => {
            todo!("Create WebSocket server handler")
        }
        ("websocket", false) => {
            todo!("Create WebSocket client handler")
        }
        ("http", true) => {
            todo!("Create HTTP server handler")
        }
        ("http", false) => {
            todo!("Create HTTP client handler")
        }
        ("http2", true) => {
            todo!("Create HTTP/2 server handler")
        }
        ("http2", false) => {
            todo!("Create HTTP/2 client handler")
        }
        ("http3", true) => {
            todo!("Create HTTP/3 server handler")
        }
        ("http3", false) => {
            todo!("Create HTTP/3 client handler")
        }
        _ => anyhow::bail!("Unsupported protocol: {}", protocol),
    }
}
