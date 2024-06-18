/// The size of one pixel on CHIP-8 screen in the current display.
const PIXEL_SCALE: (Upixel, Upixel) = (20, 20);
/// the emulator had a window size of 64x32 pixels
pub const SCREEN_SIZE: (Upixel, Upixel) = (64, 32);

const WIDTH: Upixel = SCREEN_SIZE.0 * PIXEL_SCALE.0;
const HEIGHT: Upixel = SCREEN_SIZE.1 * PIXEL_SCALE.1;

const ON_PIXEL_COLOR: [u8; 4] = [0xe8, 0xf2, 0x55, 0xff];
const OFF_PIXEL_COLOR: [u8; 4] = [0xb5, 0x83, 0x16, 0xff];

pub fn main_thread(
    graphics_mem: Arc<RwLock<GraphicsMemory>>,
    barrier: Arc<Barrier>,
    inp_sender: Sender<(crate::input::Key, ElementState)>,
) {
    // safety: unwrap, as for any failures, we want to panic

    let event_loop = EventLoop::new().unwrap(); // talk with the OS to create a window
    event_loop.set_control_flow(ControlFlow::Poll); // maybe use waituntil(60hz/sth), but docs say to use poll

    let mut app = App::new(graphics_mem, barrier, inp_sender);

    event_loop.run_app(&mut app).unwrap();

    // anything done here doesnt execute until the event loop is closed
    // i.e., the app is closed.
}

struct App {
    // window needs to be stored, as dropping it means closing the window
    window: Option<Window>,
    graphics_mem: Arc<RwLock<GraphicsMemory>>,
    pixels: Option<Pixels>,
    barrier: Arc<Barrier>,
    inp_sender: Sender<(crate::input::Key, ElementState)>,
}

/// The complete memory assosciated to graphics
pub struct GraphicsMemory(pub [bool; GraphicsMemory::TOTAL_PIXELS]);

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("Window has been resumed/ initialized, probably initialized for the first time");

        let window = event_loop
            .create_window(
                WindowAttributes::default()
                    .with_title("CHIP-8 Emulator")
                    .with_min_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
                    .with_resizable(true),
            )
            .expect("Failed to create window");
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);

        self.pixels = Some(Pixels::new(SCREEN_SIZE.0, SCREEN_SIZE.1, surface_texture).unwrap());
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        use KeyCode::*;
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                if let Some(pixels) = self.pixels.as_mut() {
                    if let Err(e) = pixels.resize_surface(physical_size.width, physical_size.height)
                    {
                        // probably means the window is minimized
                        tracing::error!("Failed to resize surface: {}", e);
                        self.pixels = None;
                    }
                } else {
                    tracing::info!("Empty pixels, trying to re-initialize");
                    let surface_texture =
                        SurfaceTexture::new(WIDTH, HEIGHT, self.window.as_ref().unwrap());
                    self.pixels =
                        Some(Pixels::new(SCREEN_SIZE.0, SCREEN_SIZE.1, surface_texture).unwrap());
                }
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state,
                        repeat: false,
                        ..
                    },
                ..
            } => match key {
                KeyCode::Escape => tracing::info!("Escape key {:?}", state),
                Numpad0 | Numpad1 | Numpad2 | Numpad3 | Numpad4 | Numpad5 | Numpad6 | Numpad7
                | Numpad8 | Numpad9 | KeyA | KeyB | KeyC | KeyD | KeyE | KeyF => {
                    if let Some(key) = crate::input::Key::from_key_code(key) {
                        self.inp_sender.send((key, state)).unwrap();
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: winit::event::StartCause) {
        use winit::event::StartCause::*;
        match cause {
            // todo: emulate accurate timing/refresh rate
            Poll => {
                // todo: handle this error!
                // i.e., what to do when window is minimized?
                // (better to pause the cpu execution too...)
                let _ = self.render_mem();
            }

            Init => {
                self.barrier.wait();
            }
            _ => {}
        }
    }
}

impl App {
    fn new(
        graphics_mem: Arc<RwLock<GraphicsMemory>>,
        barrier: Arc<Barrier>,
        inp_sender: Sender<(crate::input::Key, ElementState)>,
    ) -> Self {
        Self {
            window: None,
            pixels: None,
            graphics_mem,
            barrier,
            inp_sender,
        }
    }

    fn render_mem(&mut self) -> Result<(), Box<dyn std::error::Error + '_>> {
        let pixels = self.pixels.as_mut().ok_or("Pixels not initialized")?;
        let frame = pixels.frame_mut();
        for (display_pixel, memory_value) in
            std::iter::zip(frame.chunks_exact_mut(4), self.graphics_mem.read()?.iter())
        {
            #[rustfmt::skip]
            let data = if *memory_value { ON_PIXEL_COLOR } else { OFF_PIXEL_COLOR };
            display_pixel.copy_from_slice(&data);
        }

        pixels.render()?;
        std::thread::sleep(std::time::Duration::from_micros(1_000_000 / 60)); // 60Hz
        Ok(())
    }
}

impl GraphicsMemory {
    const TOTAL_PIXELS: usize =
        (crate::graphics::SCREEN_SIZE.0 * crate::graphics::SCREEN_SIZE.1) as usize;

    pub fn new() -> Self {
        tracing::info!("Initializing graphics memory");
        let mut data = [false; Self::TOTAL_PIXELS];
        // a simple design of a 5x5 square
        data[0] = true;
        data[1] = true;
        data[2] = true;
        data[3] = true;
        data[4] = true;
        data[5] = true;
        data[10] = true;
        data[15] = true;
        data[20] = true;
        data[21] = true;
        data[22] = true;
        data[23] = true;
        data[24] = true;

        data[63] = true;
        data[63 + 64] = true;
        data[63 + 128] = true;
        data[63 + 192] = true;
        data[63 + 256] = true;
        data[63 + 320] = true;
        data[63 + 384] = true;
        Self(data)
    }

    /// Make a iterator over the pixels as registered in the graphics memory
    pub fn iter<'it>(&self) -> crate::memory::MemoryIterator<bool> {
        crate::memory::MemoryIterator {
            index: 0,
            data_slice: &self.0,
            max_index: Self::TOTAL_PIXELS,
        }
    }

    pub fn clear_screen(&mut self) {
        self.0 = [false; Self::TOTAL_PIXELS];
    }

    #[rustfmt::skip]
    pub fn display_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        const MAX_X: usize = crate::graphics::SCREEN_SIZE.0 as usize;
        const MAX_Y: usize = crate::graphics::SCREEN_SIZE.1 as usize;
        let x = x as usize % MAX_X;
        let y = y as usize % MAX_Y;

        let mut collision = false;
        for (y_off, sprite_byte) in sprite.iter().enumerate() {
            let y = y + y_off;
            if y >= MAX_Y { Self::report_out_of_screen(x, y); continue; }
            for x_off in 0..8 {
                let x = x + x_off;
                if x >= MAX_X { Self::report_out_of_screen(x, y); continue; }
                let pixel = &mut self.0[y*MAX_X + x];
                let sprite_pixel = (sprite_byte >> (7 - x_off)) & 0x1 == 1;
                collision |= *pixel && sprite_pixel;
                *pixel ^= sprite_pixel;
            }
        }
        collision
    }

    fn report_out_of_screen(x: usize, y: usize) {
        tracing::warn!("Sprite out of screen, x: {}, y: {}, Clipping it!", x, y);
    }

    #[cfg(debug_assertions)]
    #[allow(unused)]
    /// For sake of testing, negate all the pixels
    pub fn negate(&mut self) {
        for pixel in self.0.iter_mut() {
            *pixel = !*pixel;
        }
    }
}

/// an arbitary variable type holder, that I want to change
/// incase any API to dependecy libraries changes
type Upixel = u32;

use std::sync::{mpsc::Sender, Arc, Barrier, RwLock};

use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

use pixels::{Pixels, SurfaceTexture};

use tracing::info;
