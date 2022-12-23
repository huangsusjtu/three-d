use crate::*;
use std::ops::Deref;

include!("winit_window/settings.rs");

pub struct Window {
    context: HeadlessContext,
    size: (u32, u32),
}

impl Window {
    pub fn new(
        window_settings: WindowSettings,
        surface_settings: SurfaceSettings,
    ) -> Result<Self, HeadlessError> {
        Ok(Self {
            context: HeadlessContext::new()?,
            size: window_settings.max_size.unwrap_or(window_settings.min_size),
        })
    }

    pub fn render_loop(self, mut callback: impl 'static + FnMut(FrameInput) -> FrameOutput) {
        let exit_time = if let Ok(v) = std::env::var("THREE_D_EXIT") {
            v.parse::<f64>().unwrap()
        } else {
            3000.0
        };

        let mut color_texture = Texture2D::new_empty::<[u8; 4]>(
            &self.context,
            self.size.0,
            self.size.1,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
        let mut depth_texture = DepthTexture2D::new::<f32>(
            &self.context,
            self.size.0,
            self.size.1,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        let mut last_time = std::time::Instant::now();
        let mut accumulated_time = 0.0;
        let mut first_frame = true;
        while exit_time > accumulated_time {
            let now = std::time::Instant::now();
            let duration = now.duration_since(last_time);
            if duration.as_millis() > 30 {
                last_time = now;
                let elapsed_time =
                    duration.as_secs() as f64 * 1000.0 + duration.subsec_nanos() as f64 * 1e-6;
                accumulated_time += elapsed_time;
                callback(FrameInput {
                    events: Vec::new(),
                    elapsed_time,
                    accumulated_time,
                    viewport: self.viewport(),
                    device_pixel_ratio: 1.0,
                    window_width: self.size.0,
                    window_height: self.size.1,
                    first_frame,
                    context: self.context.deref().clone(),
                    render_target: std::rc::Rc::new(RenderTarget::new(
                        color_texture.as_color_target(None),
                        depth_texture.as_depth_target(),
                    )),
                });
                first_frame = false;
            }
        }

        if let Ok(ref v) = std::env::var("THREE_D_SCREENSHOT") {
            let pixels = RenderTarget::new(
                color_texture.as_color_target(None),
                depth_texture.as_depth_target(),
            )
            .read_color::<[u8; 4]>();
            use three_d_asset::io::Serialize;
            CpuTexture {
                data: TextureData::RgbaU8(pixels),
                width: self.size.0,
                height: self.size.1,
                ..Default::default()
            }
            .serialize(v)
            .unwrap()
            .save()
            .unwrap();
        }
    }

    ///
    /// Return the current logical size of the window.
    ///
    pub fn size(&self) -> (u32, u32) {
        self.size
    }

    ///
    /// Returns the current viewport of the window in physical pixels (the size of the screen [RenderTarget] which is returned from [FrameInput::screen]).
    ///
    pub fn viewport(&self) -> Viewport {
        Viewport::new_at_origo(self.size.0, self.size.1)
    }

    ///
    /// Returns the graphics context for this window.
    ///
    pub fn gl(&self) -> Context {
        self.context.deref().clone()
    }
}
use crate::control::*;
use crate::core::*;

///
/// Input from the window to the rendering (and whatever else needs it) each frame.
///
#[derive(Clone)]
pub struct FrameInput<'a> {
    /// A list of [events](crate::Event) which has occurred since last frame.
    pub events: Vec<Event>,

    /// Milliseconds since last frame.
    pub elapsed_time: f64,

    /// Milliseconds accumulated time since start.
    pub accumulated_time: f64,

    /// Viewport of the window in physical pixels (the size of the screen [RenderTarget] which is returned from [FrameInput::screen]).
    pub viewport: Viewport,

    /// Width of the window in logical pixels.
    pub window_width: u32,

    /// Height of the window in logical pixels.
    pub window_height: u32,

    /// Number of physical pixels for each logical pixel.
    pub device_pixel_ratio: f64,

    /// Whether or not this is the first frame.
    pub first_frame: bool,

    /// The graphics context for the window.
    pub context: Context,

    pub render_target: std::rc::Rc<RenderTarget<'a>>,
}

impl<'a> FrameInput<'a> {
    pub fn screen(&'a self) -> &'a RenderTarget {
        self.render_target.as_ref()
    }
}

///
/// Output from the rendering to the window each frame.
///
#[derive(Clone, Debug)]
pub struct FrameOutput {
    ///
    /// If this is true:
    /// - On desktop, the window is closed and the renderloop is stopped.
    /// - On web, the render loop is stopped, the event handlers are removed and the `Window` dropped. Note that the canvas is not removed.
    ///
    pub exit: bool,

    ///
    /// Swaps the back and front buffer if this is true.
    /// Set this to true if something have been rendered this frame and you want to display it.
    /// Set it to false if nothing have been rendered this frame, for example if nothing has changed,
    /// and you want to reuse the image from an old frame.
    /// Currently ignored on web, since it does not use double buffering.
    ///
    pub swap_buffers: bool,

    ///
    /// Whether to stop the render loop until next event.
    ///
    pub wait_next_event: bool,
}

impl Default for FrameOutput {
    fn default() -> Self {
        Self {
            exit: false,
            swap_buffers: true,
            wait_next_event: false,
        }
    }
}
