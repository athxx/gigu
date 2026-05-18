#![allow(dead_code)]

pub use makepad_widgets;

use makepad_widgets::makepad_platform::video::{VideoInputsEvent, VideoPixelFormat};
use makepad_widgets::*;
use platform::platform::location::{LocationService, LocationSnapshot};

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    let HeroCard = RoundedView{
        width: Fill
        height: Fit
        flow: Down
        spacing: 10
        padding: Inset{top: 18. bottom: 18. left: 18. right: 18.}
        draw_bg.color: #xf3efe6
        draw_bg.border_radius: 18.
    }

    let InfoCard = RoundedView{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8
        padding: Inset{top: 18. bottom: 18. left: 18. right: 18.}
        draw_bg.color: #xd7e7f4
        draw_bg.border_radius: 18.
    }

    let AccentCard = RoundedView{
        width: Fill
        height: Fit
        flow: Down
        spacing: 8
        padding: Inset{top: 18. bottom: 18. left: 18. right: 18.}
        draw_bg.color: #xe7dac5
        draw_bg.border_radius: 18.
    }

    let NavShell = RoundedView{
        width: Fill
        height: Fit
        flow: Right
        spacing: 10
        padding: Inset{top: 10. bottom: 10. left: 10. right: 10.}
        draw_bg.color: #x122130
        draw_bg.border_radius: 24.
    }

    let NavButton = Button{
        width: Fill
        height: 44
    }

    let ToolButton = Button{
        width: Fill
        height: 42
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "gigu Demo"
                window.inner_size: vec2(430, 900)
                pass.clear_color: vec4(0.05, 0.09, 0.15, 1.0)
                body +: {
                    width: Fill
                    height: Fill
                    flow: Down
                    spacing: 16
                    padding: Inset{top: 24. bottom: 18. left: 18. right: 18.}

                    page_flip := PageFlip{
                        width: Fill
                        height: Fill
                        flow: Down
                        active_page: @page_home

                        page_home := View{
                            width: Fill
                            height: Fill
                            flow: Down
                            spacing: 16
                            padding: Inset{top: 12. bottom: 4. left: 6. right: 6.}

                            HeroCard{
                                Label{
                                    text: "gigu Dev Shell"
                                    draw_text.color: #x152033
                                    draw_text.text_style: theme.font_bold{font_size: 18.}
                                }
                                Label{
                                    text: "This demo now includes camera, iOS location, and debug surfaces for day-to-day development."
                                    draw_text.color: #x31455f
                                    draw_text.text_style.font_size: 11.
                                }
                            }

                            InfoCard{
                                Label{
                                    text: "Workflow"
                                    draw_text.color: #x102132
                                    draw_text.text_style: theme.font_bold{font_size: 13.}
                                }
                                Label{
                                    text: "1. Edit code  2. ./run-ios.sh  3. watch this terminal  4. ./scripts/ios-log.sh in another terminal."
                                    draw_text.color: #x28435d
                                    draw_text.text_style.font_size: 10.
                                }
                                home_cta_btn := ToolButton{
                                    text: "Emit Dev Log"
                                }
                            }

                            AccentCard{
                                Label{
                                    text: "What Changed"
                                    draw_text.color: #x2c2318
                                    draw_text.text_style: theme.font_bold{font_size: 13.}
                                }
                                Label{
                                    text: "Camera came from the official Makepad example. Location is wired through an iOS CoreLocation bridge with desktop-safe fallback."
                                    draw_text.color: #x5e4c39
                                    draw_text.text_style.font_size: 10.
                                }
                            }
                        }

                        page_camera := View{
                            width: Fill
                            height: Fill
                            flow: Down
                            spacing: 16
                            padding: Inset{top: 12. bottom: 4. left: 6. right: 6.}

                            HeroCard{
                                Label{
                                    text: "Camera"
                                    draw_text.color: #x152033
                                    draw_text.text_style: theme.font_bold{font_size: 18.}
                                }
                                Label{
                                    text: "Texture and native preview modes are wired in. On iOS Simulator you may see no camera device, which is expected."
                                    draw_text.color: #x31455f
                                    draw_text.text_style.font_size: 11.
                                }
                            }

                            InfoCard{
                                mode_row := View{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 8

                                    camera_off_btn := ToolButton{text: "Off"}
                                    camera_texture_btn := ToolButton{text: "Texture"}
                                    camera_native_btn := ToolButton{text: "Native"}
                                }

                                camera_mode_label := Label{
                                    text: "Mode: off"
                                    draw_text.color: #x102132
                                    draw_text.text_style.font_size: 10.
                                }
                                camera_permission_label := Label{
                                    text: "Permission: waiting"
                                    draw_text.color: #x102132
                                    draw_text.text_style.font_size: 10.
                                }
                                camera_rotation_label := Label{
                                    text: "YUV rotation: 0 (0°)"
                                    draw_text.color: #x28435d
                                    draw_text.text_style.font_size: 10.
                                }
                                camera_status_label := Label{
                                    text: "Camera is off"
                                    draw_text.color: #x28435d
                                    draw_text.text_style.font_size: 10.
                                }
                            }

                            AccentCard{
                                width: Fill
                                height: Fill

                                camera_placeholder := View{
                                    width: Fill
                                    height: Fill
                                    align: Center
                                    camera_hint := Label{
                                        text: "Camera preview is off"
                                        draw_text.color: #x6a6258
                                        draw_text.text_style.font_size: 11.
                                    }
                                }

                                camera_native_host := View{
                                    width: Fill
                                    height: Fill
                                    visible: false
                                    camera_video_native := Video{
                                        width: Fill
                                        height: Fill
                                        autoplay: false
                                        show_controls: false
                                    }
                                }

                                camera_texture_host := View{
                                    width: Fill
                                    height: Fill
                                    visible: false
                                    camera_video_texture := Video{
                                        width: Fill
                                        height: Fill
                                        autoplay: false
                                        show_controls: false
                                    }
                                }
                            }
                        }

                        page_location := View{
                            width: Fill
                            height: Fill
                            flow: Down
                            spacing: 16
                            padding: Inset{top: 12. bottom: 4. left: 6. right: 6.}

                            HeroCard{
                                Label{
                                    text: "Location"
                                    draw_text.color: #x152033
                                    draw_text.text_style: theme.font_bold{font_size: 18.}
                                }
                                Label{
                                    text: "This page uses a direct iOS bridge. In Simulator, set a fake route from Features > Location to see coordinates move."
                                    draw_text.color: #x31455f
                                    draw_text.text_style.font_size: 11.
                                }
                            }

                            InfoCard{
                                location_actions := View{
                                    width: Fill
                                    height: Fit
                                    flow: Right
                                    spacing: 8

                                    location_start_btn := ToolButton{text: "Start"}
                                    location_refresh_btn := ToolButton{text: "Refresh"}
                                    location_stop_btn := ToolButton{text: "Stop"}
                                }

                                location_permission_label := Label{
                                    text: "Permission: idle"
                                    draw_text.color: #x102132
                                    draw_text.text_style.font_size: 10.
                                }
                                location_coords_label := Label{
                                    text: "Coordinates: unavailable"
                                    draw_text.color: #x102132
                                    draw_text.text_style.font_size: 10.
                                }
                                location_meta_label := Label{
                                    text: "Details: waiting"
                                    draw_text.color: #x28435d
                                    draw_text.text_style.font_size: 10.
                                }
                                location_hint_label := Label{
                                    text: "Hint: tap Start on iOS to request location access."
                                    draw_text.color: #x28435d
                                    draw_text.text_style.font_size: 10.
                                }
                            }

                            AccentCard{
                                Label{
                                    text: "Notes"
                                    draw_text.color: #x2c2318
                                    draw_text.text_style: theme.font_bold{font_size: 13.}
                                }
                                Label{
                                    text: "Desktop builds stay runnable and show a friendly fallback. iOS builds request permission and poll CoreLocation."
                                    draw_text.color: #x5e4c39
                                    draw_text.text_style.font_size: 10.
                                }
                            }
                        }

                        page_debug := View{
                            width: Fill
                            height: Fill
                            flow: Down
                            spacing: 16
                            padding: Inset{top: 12. bottom: 4. left: 6. right: 6.}

                            HeroCard{
                                Label{
                                    text: "Debug"
                                    draw_text.color: #x152033
                                    draw_text.text_style: theme.font_bold{font_size: 18.}
                                }
                                Label{
                                    text: "Use this page to verify app-side logging while another terminal tails simulator logs."
                                    draw_text.color: #x31455f
                                    draw_text.text_style.font_size: 11.
                                }
                            }

                            InfoCard{
                                debug_log_info_btn := ToolButton{text: "Log Info"}
                                debug_log_warn_btn := ToolButton{text: "Log Error"}
                                Label{
                                    text: "./run-ios.sh"
                                    draw_text.color: #x102132
                                    draw_text.text_style: theme.font_bold{font_size: 12.}
                                }
                                Label{
                                    text: "./scripts/ios-log.sh"
                                    draw_text.color: #x28435d
                                    draw_text.text_style.font_size: 10.
                                }
                            }

                            AccentCard{
                                Label{
                                    text: "Expected Behavior"
                                    draw_text.color: #x2c2318
                                    draw_text.text_style: theme.font_bold{font_size: 13.}
                                }
                                Label{
                                    text: "The first terminal shows build and launch output. The second terminal streams iOS logs, including log! and error! messages from this app."
                                    draw_text.color: #x5e4c39
                                    draw_text.text_style.font_size: 10.
                                }
                            }
                        }
                    }

                    status := Label{
                        text: "Current page: Home"
                        draw_text.color: #xd8e6f2
                        draw_text.text_style.font_size: 11.
                    }

                    NavShell{
                        nav_home := NavButton{text: "Home"}
                        nav_camera := NavButton{text: "Camera"}
                        nav_location := NavButton{text: "Locate"}
                        nav_debug := NavButton{text: "Debug"}
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    desired_mode: CameraHomeMode,
    #[rust]
    pending_mode_switch: bool,
    #[rust]
    camera_permission: Option<makepad_widgets::makepad_platform::permission::PermissionStatus>,
    #[rust]
    camera_choice: Option<CameraChoice>,
    #[rust]
    last_yuv_rotation_steps: f32,
    #[rust]
    location_service: LocationService,
    #[rust]
    location_poll_next_frame: NextFrame,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
enum CameraHomeMode {
    #[default]
    NoCamera,
    Texture,
    NativePreview,
}

impl CameraHomeMode {
    fn label(self) -> &'static str {
        match self {
            Self::NoCamera => "off",
            Self::Texture => "texture",
            Self::NativePreview => "nativepreview",
        }
    }

    fn to_preview_mode(self) -> VideoCameraPreviewMode {
        match self {
            Self::Texture => VideoCameraPreviewMode::Texture,
            Self::NativePreview => VideoCameraPreviewMode::Native,
            Self::NoCamera => VideoCameraPreviewMode::Texture,
        }
    }
}

#[derive(Clone)]
struct CameraChoice {
    input_id: makepad_widgets::makepad_platform::video::VideoInputId,
    format_id: makepad_widgets::makepad_platform::video::VideoFormatId,
    name: String,
    width: usize,
    height: usize,
    pixel_format: makepad_widgets::makepad_platform::video::VideoPixelFormat,
    frame_rate: Option<f64>,
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(cx, ids!(nav_home)).clicked(actions) {
            self.switch_page(cx, live_id!(page_home), "Home");
        }

        if self.ui.button(cx, ids!(nav_camera)).clicked(actions) {
            self.switch_page(cx, live_id!(page_camera), "Camera");
        }

        if self.ui.button(cx, ids!(nav_location)).clicked(actions) {
            self.switch_page(cx, live_id!(page_location), "Location");
            let snapshot = self.location_service.poll();
            self.update_location_ui(cx, &snapshot);
        }

        if self.ui.button(cx, ids!(nav_debug)).clicked(actions) {
            self.switch_page(cx, live_id!(page_debug), "Debug");
        }

        if self.ui.button(cx, ids!(home_cta_btn)).clicked(actions) {
            log!("home: manual dev log emitted");
            self.set_status(cx, "Current page: Home");
        }

        if self.ui.button(cx, ids!(camera_off_btn)).clicked(actions) {
            self.choose_mode(cx, CameraHomeMode::NoCamera);
        }
        if self
            .ui
            .button(cx, ids!(camera_texture_btn))
            .clicked(actions)
        {
            self.choose_mode(cx, CameraHomeMode::Texture);
        }
        if self.ui.button(cx, ids!(camera_native_btn)).clicked(actions) {
            self.choose_mode(cx, CameraHomeMode::NativePreview);
        }

        if self
            .ui
            .button(cx, ids!(location_start_btn))
            .clicked(actions)
        {
            let snapshot = self.location_service.start();
            log!("location: start requested");
            self.update_location_ui(cx, &snapshot);
            if self.location_service.is_tracking() {
                self.location_poll_next_frame = cx.new_next_frame();
            }
        }

        if self
            .ui
            .button(cx, ids!(location_refresh_btn))
            .clicked(actions)
        {
            let snapshot = self.location_service.refresh();
            log!("location: refresh requested");
            self.update_location_ui(cx, &snapshot);
            if self.location_service.is_tracking() {
                self.location_poll_next_frame = cx.new_next_frame();
            }
        }

        if self.ui.button(cx, ids!(location_stop_btn)).clicked(actions) {
            let snapshot = self.location_service.stop();
            log!("location: stop requested");
            self.update_location_ui(cx, &snapshot);
        }

        if self
            .ui
            .button(cx, ids!(debug_log_info_btn))
            .clicked(actions)
        {
            log!("debug: info log button tapped");
        }

        if self
            .ui
            .button(cx, ids!(debug_log_warn_btn))
            .clicked(actions)
        {
            error!("debug: error log button tapped");
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());

        if self.location_poll_next_frame.is_event(event).is_some()
            && self.location_service.is_tracking()
        {
            let snapshot = self.location_service.poll();
            self.update_location_ui(cx, &snapshot);
            self.location_poll_next_frame = cx.new_next_frame();
        }

        match event {
            Event::Startup => {
                cx.request_permission(
                    makepad_widgets::makepad_platform::permission::Permission::Camera,
                );
                cx.video_input(0, |_buf| {});
                self.update_mode_label(cx);
                self.update_rotation_label(cx);
                self.set_preview_mode_visible(cx, None);
                self.set_camera_status(cx, "Camera is off");
                self.set_camera_permission_label(cx, "Permission: waiting for response");
                self.sync_nav_labels(cx, "Home");
                let snapshot = self.location_service.poll();
                self.update_location_ui(cx, &snapshot);
                log!("app: startup complete");
            }
            Event::VideoInputs(ev) => {
                let Some(_) = ev.descs.first() else {
                    self.camera_choice = None;
                    self.set_preview_mode_visible(cx, None);
                    self.set_camera_status(cx, "No camera found. This is normal on iOS Simulator.");
                    return;
                };

                self.camera_choice = Self::pick_camera_choice(ev);
                if self.camera_choice.is_none() {
                    self.set_camera_status(cx, "No suitable YUV camera format found");
                }

                self.drive_mode(cx);
            }
            Event::PermissionResult(result) => {
                use makepad_widgets::makepad_platform::permission::{Permission, PermissionStatus};

                if result.permission == Permission::Camera {
                    self.camera_permission = Some(result.status);
                    let label = match result.status {
                        PermissionStatus::Granted => "Permission: granted",
                        PermissionStatus::DeniedCanRetry => "Permission: denied, can retry",
                        PermissionStatus::DeniedPermanent => "Permission: denied in settings",
                        PermissionStatus::NotDetermined => "Permission: not determined",
                    };
                    self.set_camera_permission_label(cx, label);

                    match result.status {
                        PermissionStatus::Granted => {
                            if self.desired_mode == CameraHomeMode::NoCamera {
                                self.set_camera_status(cx, "Camera is off");
                            } else {
                                self.set_camera_status(cx, "Camera permission granted");
                            }
                            self.drive_mode(cx);
                        }
                        PermissionStatus::DeniedPermanent => {
                            self.set_preview_mode_visible(cx, None);
                            self.set_camera_status(cx, "Camera permission denied");
                        }
                        _ => {
                            self.set_camera_status(
                                cx,
                                &format!("Camera permission: {:?}", result.status),
                            );
                        }
                    }
                }
            }
            Event::VideoPlaybackPrepared(ev) => {
                if self.desired_mode == CameraHomeMode::NativePreview {
                    self.set_preview_mode_visible(cx, Some(CameraHomeMode::NativePreview));
                }
                self.set_camera_status(
                    cx,
                    &format!(
                        "Running {} mode at {}x{}",
                        self.desired_mode.label(),
                        ev.video_width,
                        ev.video_height
                    ),
                );
                self.drive_mode(cx);
            }
            Event::VideoTextureUpdated(ev) => {
                self.last_yuv_rotation_steps = ev.yuv.rotation_steps;
                self.update_rotation_label(cx);
                if self.desired_mode == CameraHomeMode::Texture {
                    self.set_preview_mode_visible(cx, Some(CameraHomeMode::Texture));
                }
            }
            Event::VideoPlaybackResourcesReleased(_) => {
                self.drive_mode(cx);
            }
            Event::VideoDecodingError(ev) => {
                self.set_preview_mode_visible(cx, None);
                self.set_camera_status(cx, &format!("Camera error: {}", ev.error));
            }
            _ => {}
        }
    }
}

impl App {
    fn set_status(&self, cx: &mut Cx, text: &str) {
        self.ui.label(cx, ids!(status)).set_text(cx, text);
    }

    fn switch_page(&self, cx: &mut Cx, page_id: LiveId, title: &str) {
        self.ui
            .page_flip(cx, ids!(page_flip))
            .set_active_page(cx, page_id);
        self.set_status(cx, &format!("Current page: {title}"));
        self.sync_nav_labels(cx, title);
    }

    fn sync_nav_labels(&self, cx: &mut Cx, active: &str) {
        self.ui
            .button(cx, ids!(nav_home))
            .set_text(cx, if active == "Home" { "• Home" } else { "Home" });
        self.ui.button(cx, ids!(nav_camera)).set_text(
            cx,
            if active == "Camera" {
                "• Camera"
            } else {
                "Camera"
            },
        );
        self.ui.button(cx, ids!(nav_location)).set_text(
            cx,
            if active == "Location" {
                "• Locate"
            } else {
                "Locate"
            },
        );
        self.ui.button(cx, ids!(nav_debug)).set_text(
            cx,
            if active == "Debug" {
                "• Debug"
            } else {
                "Debug"
            },
        );
    }

    fn set_camera_status(&self, cx: &mut Cx, text: &str) {
        self.ui
            .label(cx, ids!(camera_status_label))
            .set_text(cx, text);
    }

    fn set_camera_permission_label(&self, cx: &mut Cx, text: &str) {
        self.ui
            .label(cx, ids!(camera_permission_label))
            .set_text(cx, text);
    }

    fn set_preview_mode_visible(&self, cx: &mut Cx, mode: Option<CameraHomeMode>) {
        self.ui
            .view(cx, ids!(camera_native_host))
            .set_visible(cx, mode == Some(CameraHomeMode::NativePreview));
        self.ui
            .view(cx, ids!(camera_texture_host))
            .set_visible(cx, mode == Some(CameraHomeMode::Texture));
        self.ui
            .view(cx, ids!(camera_placeholder))
            .set_visible(cx, mode.is_none());
    }

    fn update_mode_label(&self, cx: &mut Cx) {
        self.ui
            .label(cx, ids!(camera_mode_label))
            .set_text(cx, &format!("Mode: {}", self.desired_mode.label()));
    }

    fn update_rotation_label(&self, cx: &mut Cx) {
        let steps = self.last_yuv_rotation_steps.round().clamp(0.0, 3.0) as i32;
        let degrees = steps * 90;
        self.ui
            .label(cx, ids!(camera_rotation_label))
            .set_text(cx, &format!("YUV rotation: {} ({}°)", steps, degrees));
    }

    fn pick_camera_choice(ev: &VideoInputsEvent) -> Option<CameraChoice> {
        let desc = ev.descs.first()?;

        fn pixel_rank(pixel_format: VideoPixelFormat) -> usize {
            match pixel_format {
                VideoPixelFormat::NV12 => 3,
                VideoPixelFormat::YUY2 => 2,
                VideoPixelFormat::YUV420 => 1,
                _ => 0,
            }
        }

        fn is_supported(pixel_format: VideoPixelFormat) -> bool {
            matches!(
                pixel_format,
                VideoPixelFormat::NV12 | VideoPixelFormat::YUY2 | VideoPixelFormat::YUV420
            )
        }

        fn better(
            a: &makepad_widgets::makepad_platform::video::VideoFormat,
            b: &makepad_widgets::makepad_platform::video::VideoFormat,
        ) -> bool {
            let a_rank = pixel_rank(a.pixel_format);
            let b_rank = pixel_rank(b.pixel_format);
            if a_rank != b_rank {
                return a_rank > b_rank;
            }
            let a_pixels = a.width * a.height;
            let b_pixels = b.width * b.height;
            if a_pixels != b_pixels {
                return a_pixels > b_pixels;
            }
            let a_fps = a.frame_rate.unwrap_or(0.0);
            let b_fps = b.frame_rate.unwrap_or(0.0);
            a_fps > b_fps
        }

        let mut best: Option<makepad_widgets::makepad_platform::video::VideoFormat> = None;

        for fmt in &desc.formats {
            if fmt.pixel_format != VideoPixelFormat::NV12 {
                continue;
            }
            if fmt.width > 1920 || fmt.height > 1080 {
                continue;
            }
            if best.as_ref().is_none_or(|b| better(fmt, b)) {
                best = Some(*fmt);
            }
        }

        if best.is_none() {
            for fmt in &desc.formats {
                if fmt.pixel_format != VideoPixelFormat::NV12 {
                    continue;
                }
                if best.as_ref().is_none_or(|b| better(fmt, b)) {
                    best = Some(*fmt);
                }
            }
        }

        if best.is_none() {
            for fmt in &desc.formats {
                if !is_supported(fmt.pixel_format) {
                    continue;
                }
                if best.as_ref().is_none_or(|b| better(fmt, b)) {
                    best = Some(*fmt);
                }
            }
        }

        let format = best?;
        Some(CameraChoice {
            input_id: desc.input_id,
            format_id: format.format_id,
            name: desc.name.clone(),
            width: format.width,
            height: format.height,
            pixel_format: format.pixel_format,
            frame_rate: format.frame_rate,
        })
    }

    fn choose_mode(&mut self, cx: &mut Cx, mode: CameraHomeMode) {
        if self.desired_mode != mode {
            self.desired_mode = mode;
            self.pending_mode_switch = true;
        }
        self.update_mode_label(cx);
        self.drive_mode(cx);
    }

    fn drive_mode(&mut self, cx: &mut Cx) {
        if !self.pending_mode_switch {
            return;
        }

        let native_video = self.ui.video(cx, &[live_id!(camera_video_native)]);
        let texture_video = self.ui.video(cx, &[live_id!(camera_video_texture)]);

        match self.desired_mode {
            CameraHomeMode::NoCamera => {
                self.set_preview_mode_visible(cx, None);

                let native_idle = native_video.is_unprepared();
                let texture_idle = texture_video.is_unprepared();
                if native_idle && texture_idle {
                    self.pending_mode_switch = false;
                    self.set_camera_status(cx, "Camera is off");
                    return;
                }

                if !native_idle && !native_video.is_cleaning_up() {
                    native_video.stop_and_cleanup_resources(cx);
                }
                if !texture_idle && !texture_video.is_cleaning_up() {
                    texture_video.stop_and_cleanup_resources(cx);
                }

                self.set_camera_status(cx, "Stopping camera...");
            }
            CameraHomeMode::Texture | CameraHomeMode::NativePreview => {
                self.set_preview_mode_visible(cx, None);

                if !matches!(
                    self.camera_permission,
                    Some(makepad_widgets::makepad_platform::permission::PermissionStatus::Granted)
                ) {
                    self.set_camera_status(cx, "Waiting for camera permission...");
                    return;
                }

                let Some(choice) = self.camera_choice.clone() else {
                    self.set_camera_status(cx, "Waiting for camera device...");
                    return;
                };

                let (target_video, other_video) = match self.desired_mode {
                    CameraHomeMode::Texture => (texture_video, native_video),
                    CameraHomeMode::NativePreview => (native_video, texture_video),
                    CameraHomeMode::NoCamera => return,
                };

                if !other_video.is_unprepared() {
                    if !other_video.is_cleaning_up() {
                        self.set_camera_status(cx, "Cleaning up previous mode...");
                        other_video.stop_and_cleanup_resources(cx);
                    }
                    return;
                }

                if !target_video.is_unprepared() {
                    if !target_video.is_cleaning_up() {
                        self.pending_mode_switch = false;
                        if self.desired_mode == CameraHomeMode::NativePreview {
                            self.set_preview_mode_visible(cx, Some(CameraHomeMode::NativePreview));
                        }
                    }
                    return;
                }

                target_video.set_camera_preview_mode(cx, self.desired_mode.to_preview_mode());
                target_video.set_source_camera(cx, choice.input_id, choice.format_id);
                target_video.begin_playback(cx);
                if self.desired_mode == CameraHomeMode::NativePreview {
                    self.set_preview_mode_visible(cx, Some(CameraHomeMode::NativePreview));
                }
                self.pending_mode_switch = false;
                self.set_camera_status(
                    cx,
                    &format!(
                        "Starting {} mode on {} ({}x{} {:?} fps={:?})",
                        self.desired_mode.label(),
                        choice.name,
                        choice.width,
                        choice.height,
                        choice.pixel_format,
                        choice.frame_rate
                    ),
                );
            }
        }
    }

    fn update_location_ui(&self, cx: &mut Cx, snapshot: &LocationSnapshot) {
        self.ui
            .label(cx, ids!(location_permission_label))
            .set_text(cx, &snapshot.permission);
        self.ui
            .label(cx, ids!(location_coords_label))
            .set_text(cx, &snapshot.coordinates);
        self.ui
            .label(cx, ids!(location_meta_label))
            .set_text(cx, &snapshot.details);
        self.ui
            .label(cx, ids!(location_hint_label))
            .set_text(cx, &snapshot.hint);
    }
}
