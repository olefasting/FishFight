use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use glow::Context;
use glutin::event_loop::EventLoop;
use glutin::window::{Fullscreen, Window, WindowBuilder};
use glutin::window::{Window as GlutinWindow, WindowId};
use glutin::ContextBuilder;

use crate::config::Config;
use crate::event::Event;
use crate::math::Size;
use crate::result::Result;
use crate::video::Display;
use crate::window::{WindowConfig, WindowMode};

static mut CONTEXT_WRAPPER: Option<glutin::ContextWrapper<glutin::PossiblyCurrent, Window>> = None;

pub fn context_wrapper() -> &'static glutin::ContextWrapper<glutin::PossiblyCurrent, Window> {
    unsafe {
        CONTEXT_WRAPPER
            .as_ref()
            .unwrap_or_else(|| panic!("ERROR: Attempted to get window but none has been created!"))
    }
}

pub fn window() -> &'static Window {
    context_wrapper().window()
}

pub fn window_size() -> Size<f32> {
    let size = window().inner_size();

    Size {
        width: size.width as f32,
        height: size.height as f32,
    }
}

pub fn create_window<E: 'static + Debug>(
    title: &str,
    event_loop: &EventLoop<Event<E>>,
    config: &Config,
) -> Result<&'static glutin::ContextWrapper<glutin::PossiblyCurrent, Window>> {
    let mut window_builder = WindowBuilder::new().with_title(title);

    /*
    let _display = match display.into() {
        Some(display) => Some(display),
        None => event_loop.primary_monitor().map(|handle| handle.into()),
    };
    */

    window_builder = match config.window.mode {
        WindowMode::Windowed { size } => {
            let size = glutin::dpi::Size::Physical(size.into());

            window_builder
                .with_fullscreen(None)
                .with_inner_size(size)
                .with_resizable(true)
        }
        WindowMode::Borderless => {
            let fullscreen = Fullscreen::Borderless(None);

            window_builder.with_fullscreen(Some(fullscreen))
        }
        WindowMode::Fullscreen {
            resolution,
            bit_depth,
            refresh_rate,
        } => {
            //let video_mode = video_mode.clone().unwrap().into();

            //let fullscreen = Fullscreen::Exclusive(video_mode);

            let fullscreen = Fullscreen::Borderless(None);

            window_builder.with_fullscreen(Some(fullscreen))
        }
    };

    unsafe {
        let wrapper = ContextBuilder::new()
            .with_depth_buffer(0)
            .with_srgb(true)
            .with_stencil_buffer(0)
            .with_vsync(config.video.is_vsync_enabled)
            .with_multisampling(config.video.msaa_samples.unwrap_or(0))
            .build_windowed(window_builder, event_loop)?
            .make_current()?;

        CONTEXT_WRAPPER = Some(wrapper);
    };

    Ok(context_wrapper())
}

pub(crate) fn apply_window_config(config: &WindowConfig) {
    match config.mode {
        WindowMode::Windowed { size } => {
            let size = glutin::dpi::Size::Physical(size.into());

            let window = window();

            window.set_fullscreen(None);
            window.set_inner_size(size);
            window.set_resizable(true);
        }
        WindowMode::Borderless => {
            let fullscreen = Fullscreen::Borderless(None);

            let window = window();

            window.set_fullscreen(Some(fullscreen));
            window.set_resizable(false);
        }
        WindowMode::Fullscreen {
            resolution,
            bit_depth,
            refresh_rate,
        } => {
            //let video_mode = video_mode.clone().unwrap().into();

            //let fullscreen = Fullscreen::Exclusive(video_mode);

            let fullscreen = Fullscreen::Borderless(None);

            let window = window();

            window.set_fullscreen(Some(fullscreen));
            window.set_resizable(false);
        }
    }
}

pub struct WindowIcon {}
