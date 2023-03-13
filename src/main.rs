use sdl2::{
    event::Event,
    image::LoadSurface,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::BlendMode,
    surface::Surface,
};
use std::{f32::consts::PI, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;

    let window = video_subsystem
        .window("Taiko", 1280, 720)
        .position_centered()
        .build()?;

    let freud = Surface::from_file("./assets/sigmundfreud3.png")?;
    let mut void_setup = Surface::new(400, 400, PixelFormatEnum::RGBA8888)?;

    let mut event_pump = sdl.event_pump()?;
    let bg_colour = Color::RGB(80, 80, 200);
    let now = Instant::now();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        let mut window_surface = window.surface(&event_pump)?;

        window_surface.fill_rect(None, bg_colour)?;

        void_setup.set_blend_mode(BlendMode::None)?;
        void_setup.fill_rect(None, Color::RGBA(0, 0, 0, 0))?;
        void_setup.set_blend_mode(BlendMode::Blend)?;

        let x = 50 + ((now.elapsed().as_secs_f32() * PI).cos() * 100.0) as i32;
        let y = 50 + ((now.elapsed().as_secs_f32() * PI).sin() * 100.0) as i32;

        freud
            .blit(None, &mut void_setup, Some(Rect::new(x, y, 260, 300)))?;

        void_setup
            .blit(None, &mut window_surface, Some(Rect::new(100, 100, 1, 1)))?;

        window_surface.finish()?;

        //std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
