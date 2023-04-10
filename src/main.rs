use futures::executor::block_on;
use wgpu::{Adapter, Backends, Device, DeviceDescriptor, Instance, PowerPreference, RequestAdapterOptions, Surface};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("WGPU App")
        .build(&event_loop)
        .unwrap();

    // Initialize the WGPU backend
    let instance = Instance::new(Backends::all());
    let surface = unsafe { instance.create_surface(&window) };

    let adapter = block_on(request_adapter(&instance, &surface));
    let (device, queue) = block_on(request_device(adapter));

    // Your rendering code goes here
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

async fn request_adapter(instance: &Instance, surface: &Surface) -> Adapter {
    instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(surface),
        })
        .await
        .expect("Failed to find a suitable adapter")
}

async fn request_device(adapter: Adapter) -> (Device, wgpu::Queue) {
    adapter
        .request_device(
            &DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .expect("Failed to create device")
}