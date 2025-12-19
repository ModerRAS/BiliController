use serde::{Deserialize, Serialize};

/// 窗口控制相关类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowControlRequest {
    pub action: WindowAction,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WindowAction {
    Maximize,
    Fullscreen,
    WindowedFullscreen,
}

/// 导航控制相关类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NavigationRequest {
    pub direction: NavigationDirection,
    pub count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NavigationDirection {
    Up,
    Down,
    Left,
    Right,
    Enter,
}

/// 播放器控制相关类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerControlRequest {
    pub action: PlayerAction,
    pub value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PlayerAction {
    PlayPause,
    Seek,
    SetSpeed,
    SetVolume,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerState {
    pub is_playing: bool,
    pub current_time: f64,
    pub duration: f64,
    pub speed: f64,
    pub volume: u32,
}

/// 显示控制相关类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisplayControlRequest {
    pub action: DisplayAction,
    pub enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DisplayAction {
    ToggleDanmaku,
    ToggleSubtitle,
    SetResolution,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisplayState {
    pub danmaku_enabled: bool,
    pub subtitle_enabled: bool,
    pub resolution: String,
}

/// 状态查询相关类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusResponse {
    pub player_state: PlayerState,
    pub display_state: DisplayState,
    pub window_state: WindowState,
    pub video_info: VideoInfo,
    pub browser_state: Option<BrowserState>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowState {
    pub is_fullscreen: bool,
    pub is_maximized: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoInfo {
    pub title: String,
    pub author: String,
    pub duration: f64,
    pub video_id: String,
}

/// 浏览器状态
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrowserState {
    pub is_connected: bool,
    pub current_url: Option<String>,
    pub window_handle: Option<String>,
    pub selection_mode: bool,
    pub active_element: Option<usize>,
}

/// MCP JSON-RPC 请求格式
#[derive(Debug, Serialize, Deserialize)]
pub struct MCPRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<serde_json::Value>,
    pub id: u64,
}

/// MCP JSON-RPC 响应格式
#[derive(Debug, Serialize, Deserialize)]
pub struct MCPResponse {
    pub jsonrpc: String,
    pub result: Option<serde_json::Value>,
    pub error: Option<MCPError>,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// 通用响应格式
#[derive(Debug, Serialize, Deserialize)]
pub struct GenericResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

/// 浏览器控制相关类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrowserControlRequest {
    pub action: BrowserAction,
    pub url: Option<String>,
    pub selector: Option<String>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BrowserAction {
    Connect,
    Disconnect,
    OpenUrl,
    GetUrl,
    Refresh,
    GoBack,
    GoForward,
    SendKey,
    SendText,
    ClickElement,
    FindElements,
    EnterSelectionMode,
    ExitSelectionMode,
    NavigateSelection,
    GetSelectionInfo,
    ExecuteScript,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyRequest {
    pub key: String,
    pub count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextRequest {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementRequest {
    pub selector: String,
    pub index: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectionRequest {
    pub action: SelectionAction,
    pub count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SelectionAction {
    Next,
    Previous,
    First,
    Last,
    Select,
    Cancel,
    GetInfo,
}

/// 元素信息结构体（与浏览器模块保持一致）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ElementInfo {
    pub index: usize,
    pub tag_name: String,
    pub rect: ElementRect,
    pub text: String,
    pub classes: String,
    pub id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ElementRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectionInfo {
    pub active_index: Option<usize>,
    pub total_elements: usize,
    pub elements: Vec<ElementInfo>,
}

/// 浏览器连接请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrowserConnectRequest {
    pub webdriver_url: Option<String>,
    pub timeout: Option<u64>,
}

/// 键盘事件请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyboardEventRequest {
    pub key: String,
    pub modifiers: Option<Vec<String>>,
}

/// 脚本执行请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScriptRequest {
    pub script: String,
}

/// 元素列表响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementListResponse {
    pub success: bool,
    pub message: String,
    pub elements: Vec<ElementInfo>,
}

/// 选择状态响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectionStateResponse {
    pub success: bool,
    pub message: String,
    pub selection_info: Option<SelectionInfo>,
}