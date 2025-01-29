use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 1280; // Larghezza della finestra
const HEIGHT: usize = 720; // Altezza della finestra
const PARTICLE_COUNT: usize = 100; // Numero di particelle
const PARTICLE_SIZE: usize = 10; // Grandezza delle particelle
const GRAVITY: f32 = 0.2; // Intensità della gravità
const AIR_FRICTION: f32 = 0.99; // Coefficiente di attrito con l'aria (0.99 per rallentare gradualmente)

#[derive(Clone, Copy)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: u32,
}

impl Particle {
    fn new(x: f32, y: f32, vx: f32, vy: f32, color: u32) -> Self {
        Particle {
            x,
            y,
            vx,
            vy,
            color,
        }
    }

    fn update(&mut self, gravity_enabled: bool) {
        // Applica la gravità se abilitata
        if gravity_enabled {
            self.vy += GRAVITY;
        }

        // Applica l'attrito con l'aria
        self.vx *= AIR_FRICTION;
        self.vy *= AIR_FRICTION;

        // Aggiorna la posizione
        self.x += self.vx;
        self.y += self.vy;

        // Rimbalza sui bordi della finestra
        if self.x <= 0.0 || self.x >= WIDTH as f32 {
            self.vx = -self.vx;
        }
        if self.y <= 0.0 || self.y >= HEIGHT as f32 {
            self.vy = -self.vy;
        }

        // Mantieni le particelle entro i limiti della finestra
        self.x = self.x.clamp(0.0, WIDTH as f32);
        self.y = self.y.clamp(0.0, HEIGHT as f32);
    }

    fn distance_to(&self, other: &Particle) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    fn apply_collision(&mut self, other: &mut Particle) {
        let dist = self.distance_to(other);

        // Collisione: se la distanza tra le particelle è inferiore alla somma delle loro dimensioni
        if dist < PARTICLE_SIZE as f32 {
            // Semplice scambio delle velocità
            let temp_vx = self.vx;
            let temp_vy = self.vy;

            self.vx = other.vx;
            self.vy = other.vy;

            other.vx = temp_vx;
            other.vy = temp_vy;
        }
    }

    fn move_towards(&mut self, target_x: f32, target_y: f32) {
        let dx = target_x - self.x;
        let dy = target_y - self.y;
        let distance = (dx.powi(2) + dy.powi(2)).sqrt();

        if distance > 1.0 {
            self.vx += dx / distance * 0.5; // Aggiungi una piccola forza verso il target
            self.vy += dy / distance * 0.5;
        }
    }
}

fn draw_particle(buffer: &mut Vec<u32>, particle: &Particle) {
    let px = particle.x as usize;
    let py = particle.y as usize;

    for dy in 0..PARTICLE_SIZE {
        for dx in 0..PARTICLE_SIZE {
            let x = px + dx;
            let y = py + dy;

            if x < WIDTH && y < HEIGHT {
                buffer[y * WIDTH + x] = particle.color;
            }
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Simulatore di particelle - Premi ESC per uscire",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("Errore nella creazione della finestra: {}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60 FPS

    let mut rng = rand::thread_rng();
    let mut particles: Vec<Particle> = (0..PARTICLE_COUNT)
        .map(|_| {
            Particle::new(
                rng.gen_range(0.0..WIDTH as f32),
                rng.gen_range(0.0..HEIGHT as f32),
                rng.gen_range(-2.0..2.0),           // Velocità X casuale
                rng.gen_range(-2.0..2.0),           // Velocità Y casuale
                rng.gen_range(0x0000FF..=0xFFFFFF), // Colore casuale
            )
        })
        .collect();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut gravity_enabled = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.iter_mut().for_each(|pixel| *pixel = 0x000000); // Pulisce il buffer (nero)

        // Controlla la posizione del mouse
        let mouse_pos = window.get_mouse_pos(MouseMode::Clamp).unwrap_or((0.0, 0.0));
        let mouse_pressed = window.get_mouse_down(MouseButton::Right);

        // Controlla se il tasto "A" è stato premuto per attivare/disattivare la gravità
        if window.is_key_pressed(Key::G, minifb::KeyRepeat::No) {
            gravity_enabled = !gravity_enabled; // Alterna lo stato della gravità
        }

        // Gestisce le collisioni tra particelle
        for i in 0..particles.len() {
            for j in i + 1..particles.len() {
                let (p1, p2) = particles.split_at_mut(j);
                p1[i].apply_collision(&mut p2[0]);
            }
        }

        // Aggiorna e disegna le particelle
        for particle in &mut particles {
            if mouse_pressed {
                particle.move_towards(mouse_pos.0, mouse_pos.1);
            }
            particle.update(gravity_enabled);
            draw_particle(&mut buffer, particle);
        }

        // Aggiorna la finestra con il buffer
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
