
/// 布局类型
pub enum LayoutType {
    /// 水平分割 (左右布局)
    HorizontalSplit,
    /// 垂直分割 (上下布局)
    VerticalSplit,
}

/// 应用布局管理器
pub struct AppLayout {
    /// 布局类型
    pub layout_type: LayoutType,
}

impl AppLayout {
    pub fn new(layout_type: LayoutType) -> Self {
        Self { layout_type }
    }
}