#![allow(dead_code, unused)]

use anyhow::{anyhow, Result};
use log::*;
use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_void;

use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::prelude::v1_0::*;
use vulkanalia::window as vk_window;
use vulkanalia::Version;
use vulkanalia::vk::ExtDebugUtilsExtension;

mod app_struct;
use app_struct::*;


fn main() -> Result<()> {
    pretty_env_logger::init();

    // window
    let eloop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("uwu")
        .with_inner_size(LogicalSize::new(1024, 768))
        .build(&eloop);

    let window = match window {
        Ok(window) => window,
        Err(error) => panic!("Failed creating window: {:?}", error),
    };

    // app
    let mut app = unsafe { App::new(&window).expect("uwu") };

    let mut destroying = false;
    eloop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared if !destroying => unsafe { app.render(&window) }.unwrap(),

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe {
                    app.destroy();
                }
            }

            Event::DeviceEvent { .. } => {}

            Event::RedrawEventsCleared => {}
            Event::NewEvents(_poll) => {}

            _other_event => println!("{:?}", _other_event),
        }
    });

    return Ok(());
}
