use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::Rng;
use rusttype::{point, Font, Scale};
use std::time::Instant;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const GRAVITY: f32 = 0.2;
const AIR_FRICTION: f32 = 0.99;

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: u32,
}

impl Particle {
    fn update(&mut self, gravity_enabled: bool, collision_enabled: bool) {
        if gravity_enabled {
            self.vy += GRAVITY;
        }
        self.vx *= AIR_FRICTION;
        self.vy *= AIR_FRICTION;
        self.x += self.vx;
        self.y += self.vy;
        if collision_enabled {
            if self.x <= 0.0 || self.x >= WIDTH as f32 {
                self.vx = -self.vx;
            }
            if self.y <= 0.0 || self.y >= HEIGHT as f32 {
                self.vy = -self.vy;
            }
            self.x = self.x.clamp(0.0, WIDTH as f32);
            self.y = self.y.clamp(0.0, HEIGHT as f32);
        }
    }
}

fn draw_text(buffer: &mut Vec<u32>, text: &str, x: usize, y: usize, font: &Font) {
    let scale = Scale::uniform(24.0);
    let v_metrics = font.v_metrics(scale);
    let offset = point(x as f32, y as f32 + v_metrics.ascent);

    for glyph in font.layout(text, scale, offset) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, v| {
                let px = (bb.min.x + gx as i32) as usize;
                let py = (bb.min.y + gy as i32) as usize;
                if px < WIDTH && py < HEIGHT {
                    let intensity = (v * 255.0) as u32;
                    let color = (intensity << 16) | (intensity << 8) | intensity;
                    buffer[py * WIDTH + px] = color;
                }
            });
        }
    }
}

fn main() {
    let font_data = include_bytes!("/Users/vreenox/Desktop/ciambella/ciambella_e_particelle/rust/Prova_Simulatore_particelle/assets/Fira_Code,Roboto/Roboto/static/Roboto_Condensed-SemiBold.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Failed to load font");

    let mut window = Window::new(
        "Particle Simulation",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("Window creation error: {}", e));

    let mut rng = rand::thread_rng();
    let mut particle_size = 10;
    let mut particles: Vec<Particle> = (0..100)
        .map(|_| Particle {
            x: rng.gen_range(0.0..WIDTH as f32),
            y: rng.gen_range(0.0..HEIGHT as f32),
            vx: rng.gen_range(-2.0..2.0),
            vy: rng.gen_range(-2.0..2.0),
            color: rng.gen_range(0x0000FF..=0xFFFFFF),
        })
        .collect();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut gravity_enabled = false;
    let mut collision_enabled = false;
    let mut last_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let elapsed = last_time.elapsed().as_secs_f32();
        last_time = Instant::now();
        let fps = 1.0 / elapsed;

        buffer.fill(0x000000);
        let mouse_pos = window.get_mouse_pos(MouseMode::Clamp).unwrap_or((0.0, 0.0));
        let mouse_pressed = window.get_mouse_down(MouseButton::Right);

        if window.is_key_pressed(Key::G, minifb::KeyRepeat::No) {
            gravity_enabled = !gravity_enabled;
        }
        if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            collision_enabled = !collision_enabled;
        }
        if window.is_key_pressed(Key::Up, minifb::KeyRepeat::Yes) {
            for _ in 0..1000 {
                particles.push(Particle {
                    x: rng.gen_range(0.0..WIDTH as f32),
                    y: rng.gen_range(0.0..HEIGHT as f32),
                    vx: rng.gen_range(-2.0..2.0),
                    vy: rng.gen_range(-2.0..2.0),
                    color: rng.gen_range(0x0000FF..=0xFFFFFF),
                });
            }
        }
        if window.is_key_pressed(Key::Down, minifb::KeyRepeat::Yes) {
            let remove_count = particles.len().min(1000);
            particles.truncate(particles.len() - remove_count);
        }
        if window.is_key_pressed(Key::Right, minifb::KeyRepeat::Yes) {
            particle_size += 1;
        }
        if window.is_key_pressed(Key::Left, minifb::KeyRepeat::Yes) {
            if particle_size > 1 {
                particle_size -= 1;
            }
        }

        for p in particles.iter_mut() {
            if mouse_pressed {
                let dx = mouse_pos.0 - p.x;
                let dy = mouse_pos.1 - p.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > 1.0 {
                    p.vx += dx / dist * 0.5;
                    p.vy += dy / dist * 0.5;
                }
            }
            p.update(gravity_enabled, collision_enabled);
        }

        for p in &particles {
            let px = p.x as usize;
            let py = p.y as usize;
            for dy in 0..particle_size {
                for dx in 0..particle_size {
                    let x = px + dx;
                    let y = py + dy;
                    if x < WIDTH && y < HEIGHT {
                        buffer[y * WIDTH + x] = p.color;
                    }
                }
            }
        }

        draw_text(&mut buffer, &format!("FPS: {:.2}", fps), 10, 10, &font);
        draw_text(
            &mut buffer,
            &format!("Particles: {}", particles.len()),
            10,
            30,
            &font,
        );

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
