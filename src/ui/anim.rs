use crate::format::format_rate;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub const MAX_PARTICLES: usize = 300;

#[derive(Debug, Clone)]
pub struct FloatingText {
    pub text: String,
    pub x_offset: f32,
    pub y_offset: f32,
    pub life: f32,
    pub max_life: f32,
}

#[derive(Debug, Clone)]
pub struct CrumbParticle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
    pub glyph: char,
}

#[derive(Debug, Clone)]
pub struct AchievementToast {
    pub title: String,
    pub description: String,
    pub timer: f32,
}

#[derive(Debug, Clone)]
pub struct AnimState {
    pub cookie_pulse: f32,
    pub displayed_cookies: f64,
    pub floating_texts: Vec<FloatingText>,
    pub particles: Vec<CrumbParticle>,
    pub toasts: Vec<AchievementToast>,
    pub milk_phase: f32,
    pub shimmer_phase: f32,
    pub ticker_offset: usize,
    ticker_accum: f32,
    rng: SmallRng,
}

impl Default for AnimState {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimState {
    pub fn new() -> Self {
        Self {
            cookie_pulse: 0.0,
            displayed_cookies: 0.0,
            floating_texts: Vec::new(),
            particles: Vec::with_capacity(MAX_PARTICLES),
            toasts: Vec::new(),
            milk_phase: 0.0,
            shimmer_phase: 0.0,
            ticker_offset: 0,
            ticker_accum: 0.0,
            rng: SmallRng::seed_from_u64(0xC00C_1EC1),
        }
    }

    /// Triggers click visual feedback: cookie pulse, floating `+N` label, and falling crumbs.
    pub fn on_cookie_click(&mut self, gain: f64, ascii_only: bool) {
        self.cookie_pulse = 1.0;

        let text = format!("+{}", format_rate(gain));
        let x_offset = self.rng.gen_range(-8.0..8.0);
        let y_offset = self.rng.gen_range(-2.0..1.0);
        if self.floating_texts.len() >= 24 {
            self.floating_texts.remove(0);
        }
        self.floating_texts.push(FloatingText {
            text,
            x_offset,
            y_offset,
            life: 0.9,
            max_life: 0.9,
        });

        let glyphs: &[char] = if ascii_only {
            &['*', '.', ',', '\'', '`']
        } else {
            &['•', '∙', '✦', '·', '*']
        };

        for _ in 0..6 {
            if self.particles.len() >= MAX_PARTICLES {
                self.particles.remove(0);
            }
            let g = glyphs[self.rng.gen_range(0..glyphs.len())];
            self.particles.push(CrumbParticle {
                x: self.rng.gen_range(-9.0..9.0),
                y: self.rng.gen_range(-3.0..3.0),
                vx: self.rng.gen_range(-10.0..10.0),
                vy: self.rng.gen_range(-8.0..-1.0),
                life: self.rng.gen_range(0.5..1.1),
                glyph: g,
            });
        }
    }

    pub fn push_toast(&mut self, title: String, description: String) {
        if self.toasts.len() >= 4 {
            self.toasts.remove(0);
        }
        self.toasts.push(AchievementToast {
            title,
            description,
            timer: 4.0,
        });
    }

    /// Advances all animations by `dt` seconds and smoothly interpolates the cookie counter.
    pub fn update(&mut self, dt: f32, target_cookies: f64) {
        if !dt.is_finite() || dt <= 0.0 {
            return;
        }

        self.cookie_pulse = (self.cookie_pulse - dt * 6.5).max(0.0);

        // Smooth rolling counter interpolation
        let diff = target_cookies - self.displayed_cookies;
        if !self.displayed_cookies.is_finite()
            || diff.abs() > target_cookies.abs() * 0.5 + 1e9
            || diff.abs() < 0.5
        {
            self.displayed_cookies = target_cookies;
        } else {
            let alpha = (dt as f64 * 14.0).clamp(0.0, 1.0);
            self.displayed_cookies += diff * alpha;
        }

        for ft in &mut self.floating_texts {
            ft.y_offset -= dt * 4.5;
            ft.life -= dt;
        }
        self.floating_texts.retain(|ft| ft.life > 0.0);

        for p in &mut self.particles {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vy += 20.0 * dt; // Gravity
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);

        for toast in &mut self.toasts {
            toast.timer -= dt;
        }
        self.toasts.retain(|t| t.timer > 0.0);

        self.milk_phase = (self.milk_phase + dt * 3.2) % std::f32::consts::TAU;
        self.shimmer_phase = (self.shimmer_phase + dt * 5.0) % std::f32::consts::TAU;

        self.ticker_accum += dt;
        if self.ticker_accum >= 0.12 {
            self.ticker_accum -= 0.12;
            self.ticker_offset = self.ticker_offset.wrapping_add(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_cap_and_decay() {
        let mut anim = AnimState::new();
        for _ in 0..80 {
            anim.on_cookie_click(15.0, false);
        }
        assert!(anim.particles.len() <= MAX_PARTICLES);
        assert_eq!(anim.cookie_pulse, 1.0);

        anim.update(0.5, 1_000.0);
        assert!(anim.cookie_pulse < 1.0);
        assert!(anim.displayed_cookies > 0.0);

        anim.update(2.0, 1_000.0);
        assert_eq!(anim.particles.len(), 0);
        assert_eq!(anim.floating_texts.len(), 0);
    }
}
