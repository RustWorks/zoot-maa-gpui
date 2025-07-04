use global::constants::APP_NAME;
use gpui::{
    div, prelude::FluentBuilder, px, App, InteractiveElement, IntoElement, ParentElement, Pixels, RenderOnce,
    Styled, Window, WindowControlArea,
};
use gpui_component::{
    h_flex, red_400, ActiveTheme, Colorize, Icon, IconName, InteractiveElementExt, Sizable,
};

pub const TITLE_BAR_HEIGHT: Pixels = px(40.);

#[cfg(target_os = "macos")]
const TITLE_BAR_LEFT_PADDING: Pixels = px(80.);
#[cfg(not(target_os = "macos"))]
const TITLE_BAR_LEFT_PADDING: Pixels = px(12.);

#[derive(IntoElement)]
pub struct AppTitleBar {}

impl AppTitleBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for AppTitleBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let is_linux = cfg!(target_os = "linux");
        let is_macos = cfg!(target_os = "macos");

        div().flex_shrink_0().bg(cx.theme().tab_bar).child(
            div()
                .id("title-bar")
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .pl(TITLE_BAR_LEFT_PADDING)
                .pr(px(12.))
                .window_control_area(WindowControlArea::Drag)
                .when(is_linux, |this| {
                    this.on_double_click(|_, window, _| window.zoom_window())
                })
                .child(format!("{}", APP_NAME))
                .child(WindowControls {})
                .map(|this| {
                    if is_macos && window.is_fullscreen() {
                        this.h_0().invisible()
                    } else {
                        this.h(TITLE_BAR_HEIGHT).visible()
                    }
                }),
        )
    }
}

#[derive(IntoElement, Clone, PartialEq)]
enum ControlIcon {
    Minimize,
    Restore,
    Maximize,
    Close,
}

impl ControlIcon {
    fn minimize() -> Self {
        Self::Minimize
    }

    fn restore() -> Self {
        Self::Restore
    }

    fn maximize() -> Self {
        Self::Maximize
    }

    fn close() -> Self {
        Self::Close
    }

    fn id(&self) -> &'static str {
        match self {
            Self::Minimize => "minimize",
            Self::Restore => "restore",
            Self::Maximize => "maximize",
            Self::Close { .. } => "close",
        }
    }

    fn icon(&self) -> IconName {
        match self {
            Self::Minimize => IconName::WindowMinimize,
            Self::Restore => IconName::WindowRestore,
            Self::Maximize => IconName::WindowMaximize,
            Self::Close { .. } => IconName::WindowClose,
        }
    }
}

impl RenderOnce for ControlIcon {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id())
            .flex_shrink_0()
            .cursor_default()
            .flex()
            .items_center()
            .justify_center()
            .occlude()
            .size_8()
            .rounded(cx.theme().radius)
            .map(|this| match self {
                ControlIcon::Close { .. } => this.window_control_area(WindowControlArea::Close),
                ControlIcon::Maximize | ControlIcon::Restore => {
                    this.window_control_area(WindowControlArea::Max)
                },
                ControlIcon::Minimize => this.window_control_area(WindowControlArea::Min),
            })
            .child(Icon::new(self.icon()).small())
            .hover(|this| match self {
                ControlIcon::Close => this.bg(red_400()),
                _ => {
                    let color = if cx.theme().mode.is_dark() {
                        cx.theme().secondary.lighten(0.1).opacity(0.8)
                    } else {
                        cx.theme().secondary.darken(0.1).opacity(0.8)
                    };
                    this.bg(color)
                },
            })
    }
}

#[derive(IntoElement)]
struct WindowControls {}

impl RenderOnce for WindowControls {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        if cfg!(target_os = "macos") {
            return div().id("window-controls");
        }

        h_flex()
            .id("window-controls")
            .items_center()
            .flex_shrink_0()
            .h_full()
            .child(
                h_flex()
                    .justify_center()
                    .content_stretch()
                    .h_full()
                    .gap_1()
                    .child(ControlIcon::minimize())
                    .child(if window.is_maximized() {
                        ControlIcon::restore()
                    } else {
                        ControlIcon::maximize()
                    })
                    .child(ControlIcon::close()),
            )
    }
}
