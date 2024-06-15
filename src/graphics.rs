/// The size of one pixel on CHIP-8 screen in the current display.
const PIXEL_SCALE: (Upixel, Upixel) = (1, 1);
/// the emulator had a window size of 64x32 pixels
pub const SCREEN_SIZE: (Upixel, Upixel) = (64, 32);

const WIDTH: Upixel = SCREEN_SIZE.0 * PIXEL_SCALE.0;
const HEIGHT: Upixel = SCREEN_SIZE.1 * PIXEL_SCALE.1;

pub fn main_thread(graphics_mem: Arc<RwLock<GraphicsMemory>>) {
    // safety: unwrap, as for any failures, we want to panic

    let event_loop = EventLoop::new().unwrap(); // talk with the OS to create a window
    event_loop.set_control_flow(ControlFlow::Poll); // maybe use waituntil(60hz/sth), but docs say to use poll

    let mut app = App::new(graphics_mem);

    event_loop.run_app(&mut app).unwrap();

    // anything done here doesnt execute until the event loop is closed
    // i.e., the app is closed.
}

struct App {
    window: Option<Window>,
    graphics_mem: Arc<RwLock<GraphicsMemory>>,
    pixels: Option<Pixels>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Window has been resumed");

        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("CHIP-8 Emulator")
                    .with_min_inner_size(PhysicalSize::new(HEIGHT, WIDTH))
                    .with_resizable(true),
            )
            .expect("Failed to create window");
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);

        self.pixels = Some(Pixels::new(SCREEN_SIZE.0, SCREEN_SIZE.1, surface_texture).unwrap());
        self.window = Some(window);
        self.draw().unwrap();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            _ => {}
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        use winit::event::StartCause::*;
        match cause {
            // todo: emulate accurate timing/refresh rate
            Poll => self.draw().unwrap(),
            _ => {}
        }
    }
}

impl App {
    fn new(graphics_mem: Arc<RwLock<GraphicsMemory>>) -> Self {
        Self {
            window: None,
            pixels: None,
            graphics_mem,
        }
    }

    fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let pixels = self.pixels.as_mut().ok_or("Pixels not initialized")?;
        let frame = pixels.frame_mut();
        for (display_pixel, memory_value) in frame
            .chunks_exact_mut(4)
            .zip(self.graphics_mem.read().unwrap().iter())
        {
            //todo:improvement customize colours & A
            let data = if *memory_value { 0xff } else { 0x00 };
            display_pixel[0] = data; // R
            display_pixel[1] = data; // G
            display_pixel[2] = data; // B
            display_pixel[3] = 0xff; // A
        }

        pixels.render()?;
        Ok(())
    }
}

/// an arbitary variable type holder, that I want to change
/// incase any API to dependecy libraries changes
type Upixel = u32;

use crate::memory::GraphicsMemory;
use std::sync::{Arc, RwLock};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use pixels::{Pixels, SurfaceTexture};

use tracing::info;
