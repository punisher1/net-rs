use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::App;

use super::layout::LayoutType;

pub fn draw(frame: &mut Frame, app: &mut App) {
    // 首先将屏幕分为上、中、下三个部分
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // 顶部状态栏
            Constraint::Min(1),    // 中间内容区域
            Constraint::Length(1), // 底部状态栏
        ])
        .split(frame.area());

    // 绘制顶部状态栏 (统计信息)
    app.status_bar.draw_top_bar(frame, vertical_chunks[0], app);

    // 根据布局类型绘制中间的发送和接收区
    match app.layout.layout_type {
        LayoutType::HorizontalSplit => draw_horizontal(frame, app, vertical_chunks[1]),
        LayoutType::VerticalSplit => draw_vertical(frame, app, vertical_chunks[1]),
    };

    // 绘制底部状态栏 (快捷键提示)
    app.status_bar.draw_bottom_bar(frame, vertical_chunks[2]);

    // 如果有输入对话框, 绘制在最顶层
    if let Some(dialog) = &app.input_dialog {
        dialog.draw(frame);
    }
}

/// 水平布局 (左右分割)
fn draw_horizontal(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 2), // 左侧发送区
            Constraint::Ratio(1, 2), // 右侧接收区
        ])
        .split(area);

    // 绘制发送区
    app.send_view.draw(frame, chunks[0]);

    // 绘制接收区
    app.receive_view.draw(frame, chunks[1]);
}

/// 垂直布局 (上下分割)
fn draw_vertical(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 2), // 上部发送区
            Constraint::Ratio(1, 2), // 下部接收区
        ])
        .split(area);

    // 绘制发送区
    app.send_view.draw(frame, chunks[0]);

    // 绘制接收区
    app.receive_view.draw(frame, chunks[1]);
}
