use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs},
    Frame,
};

/// Tab页管理状态
pub struct TabsState {
    /// 所有Tab标题
    pub titles: Vec<String>,
    /// 当前索引
    pub index: usize,
    /// Tab所包含的内容
    pub contents: Vec<Vec<String>>,
}

impl TabsState {
    pub fn new(titles: Vec<String>) -> Self {
        let num_tabs = titles.len();
        let contents = vec![Vec::new(); num_tabs];

        Self {
            titles,
            index: 0,
            contents,
        }
    }

    /// 添加新的Tab
    pub fn add_tab(&mut self, title: String) {
        self.titles.push(title);
        self.contents.push(Vec::new());
    }

    /// 移除Tab
    pub fn remove_tab(&mut self, index: usize) {
        if index < self.titles.len() {
            self.titles.remove(index);
            self.contents.remove(index);

            // 调整当前索引
            if !self.titles.is_empty() {
                self.index = self.index.min(self.titles.len() - 1);
            } else {
                self.index = 0;
            }
        }
    }

    /// 根据标签页Title移除标签页
    pub fn remove_tab_by_title(&mut self, title: &str) {
        if let Some(index) = self.titles.iter().position(|t| t == title) {
            self.remove_tab(index);
        }
    }

    /// 向指定Tab添加消息
    pub fn add_message(&mut self, tab_index: usize, message: String) {
        if tab_index < self.contents.len() {
            self.contents[tab_index].push(message);
        }
    }

    /// 切换下一个Tab
    pub fn next(&mut self) {
        if !self.titles.is_empty() {
            self.index = (self.index + 1) % self.titles.len();
        }
    }

    /// 切换前一个Tab
    pub fn previous(&mut self) {
        if !self.titles.is_empty() {
            if self.index > 0 {
                self.index -= 1;
            } else {
                self.index = self.titles.len() - 1;
            }
        }
    }

    /// 绘制Tab栏
    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        if self.titles.is_empty() {
            return;
        }

        // 分割区域为Tab栏和内容区域
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Tab栏高度
                Constraint::Min(0),    // 内容区域
            ])
            .split(area);

        // 创建Tab栏
        let tab_titles: Vec<Line> = self.titles.iter().map(|t| Line::from(vec![Span::raw(t)])).collect();

        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL))
            .select(self.index)
            .style(Style::default())
            .highlight_style(Style::default().fg(Color::LightCyan));

        frame.render_widget(tabs, chunks[0]);

        // 绘制当前选中Tab的内容
        if self.index < self.contents.len() {
            // 这里可以扩展为绘制具体内容
            // 具体实现取决于Tab内容的数据类型和显示方式
        }
    }
}
