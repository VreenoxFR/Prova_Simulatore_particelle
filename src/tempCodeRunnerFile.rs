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