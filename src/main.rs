#![allow(dead_code, unused)]

use anyhow::{anyhow, Result};
use log::*;

use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::window as vk_window;
use vulkanalia::prelude::v1_0::*;

#[derive(Clone, Debug)]
struct App {
    entry: Entry,
    instance: Instance,
}

impl App {
    unsafe fn new(window: &Window) -> Result<Self> {
        let loader = LibloadingLoader::new(LIBRARY)?;
        let entry = Entry::new(loader).map_err(|b| anyhow!("{}", b))?;

        //let instance = create_instance(window, &entry)?;
        let application_info = vk::ApplicationInfo::builder()
            .application_name(b"uwu\0")
            .application_version(vk::make_version(0, 1, 0))
            .engine_name(b"no engine\0")
            .engine_version(vk::make_version(0, 1, 0))
            .api_version(vk::make_version(0, 1, 0));
        let extensions = vk_window::get_required_instance_extensions(window)
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();
        let instance_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_extension_names(&extensions);
        let instance = entry.create_instance(&instance_info, None)?;

        Ok(Self {entry, instance})
    }

    unsafe fn render(&mut self, window: &Window) -> Result<()> {
        Ok(())
    }

    unsafe fn destroy(&mut self) {
        self.instance.destroy_instance(None);
    }
}


#[derive(Clone, Debug, Default)]
struct AppData{}


// as far as i can tell this function is just unnecessary clean code, so i'm
// gonna just bake this function into App::new until the tutorial demonstrates
// a good reason for making this its own function
/*unsafe fn create_instance(window: &Window, entry: &Entry) -> Result<Instance> {
    let application_info = vk::ApplicationInfo::builder()
        .application_name(b"uwu\0")
        .application_version(vk::make_version(0, 1, 0))
        .engine_name(b"no engine\0")
        .engine_version(vk::make_version(0, 1, 0))
        .api_version(vk::make_version(0, 1, 0));
    let extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e| e.as_ptr())
        .collect::<Vec<_>>();
    let info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&extensions);
    
    Ok(entry.create_instance(&info, None)?)
}*/

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
        Err(error) => panic!("Failed creating window: {:?}", error)
    };

    
    // app
    let mut app = unsafe { App::new(&window).expect("uwu") };

    let mut destroying = false;
    eloop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::MainEventsCleared if !destroying =>
                unsafe { app.render(&window) }.unwrap(),

            Event::WindowEvent {event: WindowEvent::CloseRequested, ..} => {
                destroying = true;
                *control_flow = ControlFlow::Exit;
                unsafe { app.destroy(); }
            },

            Event::DeviceEvent {..} => {},

            Event::RedrawEventsCleared => {},
            Event::NewEvents(_poll) => {},

            _other_event => println!("{:?}", _other_event)
        }

    });

    return Ok(());
}
