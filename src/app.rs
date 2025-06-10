use std::time::Instant;

use anyhow::{Ok, Result};
use crossterm::event::{KeyCode, KeyModifiers};

use crate::cli::args::{AppMode, Args, ProtocolType};
use crate::ui::layout::{AppLayout, LayoutType};
use crate::ui::widgets::{input_dialog::InputDialog, message_view::MessageView, status_bar::StatusBar};

/// 应用程序状态
pub enum InputMode {
    Normal,
    Editing,
}

/// 数据显示格式
pub enum DisplayFormat {
    String,
    Hex,
}

/// 应用程序统计数据
pub struct Stats {
    pub sent_bytes: usize,
    pub received_bytes: usize,
    pub connected: bool,
    pub last_activity: Instant,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            sent_bytes: 0,
            received_bytes: 0,
            connected: false,
            last_activity: Instant::now(),
        }
    }
}

/// 主应用状态
pub struct App {
    /// 应用退出标志
    pub should_quit: bool,
    /// 输入模式
    input_mode: InputMode,
    /// 布局
    pub layout: AppLayout,
    /// 发送区状态
    pub send_view: MessageView,
    /// 接收区状态
    pub receive_view: MessageView,
    /// 状态栏
    pub status_bar: StatusBar,
    /// 输入对话框
    pub input_dialog: Option<InputDialog>,
    /// 统计数据
    pub stats: Stats,
    pub args: Args,
}

impl App {
    pub fn new(args: Args) -> Result<Self> {
        // 根据参数确定布局方式
        let layout_type = if args.vertical_layout {
            LayoutType::VerticalSplit
        } else {
            LayoutType::HorizontalSplit
        };

        // 设置发送和接收视图的标题
        let (send_title, recv_title) = match args.protocol {
            ProtocolType::Tcp => match args.mode {
                AppMode::Server => ("TCP Server Send", "TCP Server Receive"),
                AppMode::Client => ("TCP Client Send", "TCP Client Receive"),
            },
            ProtocolType::Udp => match args.mode {
                AppMode::Server => ("UDP Server Send", "UDP Server Receive"),
                AppMode::Client => ("UDP Client Send", "UDP Client Receive"),
            },
            ProtocolType::WebSocket => match args.mode {
                AppMode::Server => ("WebSocket Server Send", "WebSocket Server Receive"),
                AppMode::Client => ("WebSocket Client Send", "WebSocket Client Receive"),
            },
            ProtocolType::Http => match args.mode {
                AppMode::Server => ("HTTP Server Send", "HTTP Server Receive"),
                AppMode::Client => ("HTTP Client Send", "HTTP Client Receive"),
            },
            ProtocolType::Http2 => match args.mode {
                AppMode::Server => ("HTTP/2 Server Send", "HTTP/2 Server Receive"),
                AppMode::Client => ("HTTP/2 Client Send", "HTTP/2 Client Receive"),
            },
            ProtocolType::Http3 => match args.mode {
                AppMode::Server => ("HTTP/3 Server Send", "HTTP/3 Server Receive"),
                AppMode::Client => ("HTTP/3 Client Send", "HTTP/3 Client Receive"),
            },
        };

        Ok(Self {
            should_quit: false,
            input_mode: InputMode::Normal,
            layout: AppLayout::new(layout_type),
            send_view: MessageView::new(send_title),
            receive_view: MessageView::new(recv_title),
            status_bar: StatusBar::default(),
            input_dialog: None,
            stats: Stats::default(),
            args,
        })
    }

    /// 处理按键事件
    pub fn handle_key_event(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        match self.input_mode {
            InputMode::Normal => self.handle_normal_mode_key(key, modifiers),
            InputMode::Editing => self.handle_editing_mode_key(key, modifiers),
        }
    }

    /// 处理正常模式键盘输入
    fn handle_normal_mode_key(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        match (key, modifiers) {
            // 退出应用
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }

            // 输入模式 (I)
            (KeyCode::Char('i'), KeyModifiers::NONE) => {
                self.input_mode = InputMode::Editing;
                self.input_dialog = Some(InputDialog::new());
            }
            _ => {}
        }
        Ok(())
    }

    /// 处理编辑模式键盘输入
    fn handle_editing_mode_key(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        if let Some(dialog) = &mut self.input_dialog {
            match key {
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                    self.input_dialog = None;
                }
                KeyCode::Enter => {
                    // 获取输入内容并发送
                    if let Some(input) = dialog.submit() {
                        // 处理输入的内容，实际发送逻辑将由具体协议实现
                        self.send_message(input);
                    }
                    self.input_mode = InputMode::Normal;
                    self.input_dialog = None;
                }
                KeyCode::Char(c) => {
                    dialog.input.push(c);
                }
                KeyCode::Backspace => {
                    dialog.input.pop();
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// 模拟发送消息的方法，实际实现将连接到网络协议
    fn send_message(&mut self, message: String) {
        // 更新统计数据
        self.stats.sent_bytes += message.len();
        self.stats.last_activity = Instant::now();

        // 添加消息到发送视图
        self.send_view
            .add_message(format!("[{}] {}", chrono::Local::now().format("%H:%M:%S"), message));
    }

    /// 添加接收到的消息
    pub fn add_received_message(&mut self, message: String, from: Option<String>) {
        // 更新统计数据
        self.stats.received_bytes += message.len();
        self.stats.last_activity = Instant::now();

        // 添加消息到接收视图
        let prefix = if let Some(addr) = from {
            format!("[{}] [{}]", chrono::Local::now().format("%H:%M:%S"), addr)
        } else {
            format!("[{}]", chrono::Local::now().format("%H:%M:%S"))
        };

        self.receive_view.add_message(format!("{} {}", prefix, message));
    }

    /// 更新连接状态
    pub fn set_connected(&mut self, connected: bool) {
        self.stats.connected = connected;
    }
}
