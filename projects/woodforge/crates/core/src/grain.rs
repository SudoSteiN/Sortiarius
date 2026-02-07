use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GrainParams {
    pub ring_frequency: f32,
    pub ring_noise: f32,
    pub fiber_noise: f32,
    pub color_variation: f32,
}

impl Default for GrainParams {
    fn default() -> Self {
        Self {
            ring_frequency: 8.0,
            ring_noise: 0.3,
            fiber_noise: 0.2,
            color_variation: 0.15,
        }
    }
}

// Permutation table for Perlin noise (shuffled 0..255, doubled for wrapping)
const PERM: [u8; 512] = {
    let base: [u8; 256] = [
        151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30,
        69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94,
        252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171,
        168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60,
        211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1,
        216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86,
        164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118,
        126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
        213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39,
        253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34,
        242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49,
        192, 214, 31, 181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254,
        138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
    ];
    let mut table = [0u8; 512];
    let mut i = 0;
    while i < 512 {
        table[i] = base[i % 256];
        i += 1;
    }
    table
};

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

fn grad(hash: u8, x: f32, y: f32) -> f32 {
    match hash & 3 {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        _ => -x - y,
    }
}

/// 2D Perlin noise using a permutation table. Returns values in approximately -1 to 1.
fn perlin_2d(x: f32, y: f32, seed: u32) -> f32 {
    let seed_offset = (seed % 256) as usize;

    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let xf = x - x.floor();
    let yf = y - y.floor();

    let u = fade(xf);
    let v = fade(yf);

    let xi = ((xi % 256 + 256) % 256) as usize;
    let yi = ((yi % 256 + 256) % 256) as usize;

    let aa = PERM[(PERM[(xi + seed_offset) % 512] as usize + yi) % 512];
    let ab = PERM[(PERM[(xi + seed_offset) % 512] as usize + yi + 1) % 512];
    let ba = PERM[(PERM[(xi + 1 + seed_offset) % 512] as usize + yi) % 512];
    let bb = PERM[(PERM[(xi + 1 + seed_offset) % 512] as usize + yi + 1) % 512];

    let x1 = lerp(u, grad(aa, xf, yf), grad(ba, xf - 1.0, yf));
    let x2 = lerp(u, grad(ab, xf, yf - 1.0), grad(bb, xf - 1.0, yf - 1.0));

    lerp(v, x1, x2)
}

/// Fractal Brownian Motion: layered Perlin noise for richer textures.
fn fbm(x: f32, y: f32, seed: u32, octaves: u32) -> f32 {
    let mut value = 0.0f32;
    let mut amplitude = 1.0f32;
    let mut frequency = 1.0f32;
    let mut max_amp = 0.0f32;

    for i in 0..octaves {
        value += amplitude * perlin_2d(x * frequency, y * frequency, seed.wrapping_add(i * 97));
        max_amp += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    value / max_amp
}

/// Generate a wood grain texture as RGBA u8 pixels.
///
/// Returns a `Vec<u8>` with length `width * height * 4` containing RGBA data.
pub fn generate_grain_texture(
    width: u32,
    height: u32,
    base_color: [f32; 3],
    params: &GrainParams,
    seed: u32,
) -> Vec<u8> {
    let len = (width as usize) * (height as usize) * 4;
    let mut pixels = Vec::with_capacity(len);

    let inv_w = 1.0 / width as f32;
    let inv_h = 1.0 / height as f32;

    for y in 0..height {
        for x in 0..width {
            let nx = x as f32 * inv_w;
            let ny = y as f32 * inv_h;

            // Ring pattern: distance from a center line with noise perturbation
            let ring_noise = params.ring_noise * fbm(nx * 4.0, ny * 4.0, seed, 3);
            let ring_dist = ((nx - 0.5 + ring_noise) * params.ring_frequency).abs();
            let ring = (ring_dist * std::f32::consts::PI * 2.0).sin() * 0.5 + 0.5;

            // Fiber noise: elongated along y (grain direction)
            let fiber = params.fiber_noise * fbm(nx * 2.0, ny * 12.0, seed.wrapping_add(1000), 4);

            // Color variation noise
            let variation =
                params.color_variation * fbm(nx * 6.0, ny * 6.0, seed.wrapping_add(2000), 2);

            // Combine: darken at ring boundaries, add fiber and variation
            let grain_factor = 1.0 - ring * 0.25 + fiber + variation;

            for c in 0..3 {
                let val = base_color[c] * grain_factor;
                let clamped = val.clamp(0.0, 1.0);
                pixels.push((clamped * 255.0) as u8);
            }
            pixels.push(255); // Alpha
        }
    }

    pixels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_size() {
        let params = GrainParams::default();
        let pixels = generate_grain_texture(64, 32, [0.8, 0.6, 0.4], &params, 42);
        assert_eq!(pixels.len(), 64 * 32 * 4);
    }

    #[test]
    fn test_rgba_range() {
        let params = GrainParams::default();
        let pixels = generate_grain_texture(32, 32, [0.8, 0.6, 0.4], &params, 42);
        for &byte in &pixels {
            assert!(byte <= 255, "Pixel value out of range: {}", byte);
        }
    }

    #[test]
    fn test_deterministic() {
        let params = GrainParams::default();
        let a = generate_grain_texture(16, 16, [0.7, 0.5, 0.3], &params, 123);
        let b = generate_grain_texture(16, 16, [0.7, 0.5, 0.3], &params, 123);
        assert_eq!(a, b, "Same seed should produce identical output");
    }

    #[test]
    fn test_different_seeds_differ() {
        let params = GrainParams::default();
        let a = generate_grain_texture(16, 16, [0.7, 0.5, 0.3], &params, 1);
        let b = generate_grain_texture(16, 16, [0.7, 0.5, 0.3], &params, 2);
        assert_ne!(a, b, "Different seeds should produce different output");
    }

    #[test]
    fn test_perlin_range() {
        // Perlin noise should stay roughly in [-1, 1]
        for i in 0..100 {
            let val = perlin_2d(i as f32 * 0.1, i as f32 * 0.17, 0);
            assert!(
                val >= -1.5 && val <= 1.5,
                "Perlin value out of expected range: {}",
                val
            );
        }
    }
}
