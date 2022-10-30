use std::f64::consts::PI;

use crate::{base83, utils};

/// Decoder Error
#[derive(Debug)]
pub enum DecodeError {
    /// hash length less than 6.
    LengthError,
    /// hash format error
    FormatError,
}
/// Returns the pixel(RGBA) array of the result image given the blurhash string,
/// - `blurhash` - A string representing the blurhash to be decoded.
/// - `width` - Width of the resulting image
/// - `height` - Height of the resulting image
/// - `punch` - The factor to improve the contrast
/// # Examples
/// ```
/// use blurhash_rs;
///
/// let hash = "LEHV6nWB2yk8pyo0adR*.7kCMdnj";
/// let v = blurhash_rs::decode(hash, 32, 32, None).unwrap();
/// 
/// assert_eq!(v.len(), 4096);
/// ```
pub fn decode(
    hash: &str,
    width: u32,
    height: u32,
    punch: Option<u32>,
) -> Result<Vec<u8>, DecodeError> {
    if let Err(err) = validate(hash) {
        return Err(err);
    }
    let punch = if let Some(p) = punch { p | 1 } else { 1 };
    let size_flag = base83::decode83(&hash[..1]);
    let num_y = (size_flag as f64 / 9.0).floor() as u32 + 1;
    let num_x = ((size_flag % 9.0) + 1.0) as u32;

    let questioned_max = base83::decode83(&hash[1..2]);
    let maximum = (questioned_max + 1.0) / 166f64;

    let mut colors: Vec<[f64; 3]> = Vec::with_capacity((num_x * num_y) as usize);
    for i in 0..(num_x * num_y) as usize {
        if i == 0 {
            let v = base83::decode83(&hash[2..6]);
            colors.push(decode_dc(v as u32));
        } else {
            let v = base83::decode83(&hash[4 + 2 * i..6 + 2 * i]);
            colors.push(decode_ac(v as f64, maximum * punch as f64));
        };
    }

    let mut data = Vec::with_capacity((4 * width * height) as usize);
    data.resize((4 * width * height) as usize, 0u8);

    for y in 0..height {
        for x in 0..width {
            let mut r = 0f64;
            let mut g = 0f64;
            let mut b = 0f64;

            for j in 0..num_y {
                for i in 0..num_x {
                    let basic = ((PI * (x as f64) * (i as f64)) / width as f64).cos()
                        * ((PI * (y as f64) * (j as f64)) / height as f64).cos();
                    let color = colors[(i + j * num_x) as usize];
                    r += color[0] * basic;
                    g += color[1] * basic;
                    b += color[2] * basic;
                }
            }

            let r = utils::linear_to_rgb(r as f64) as u8;
            let g = utils::linear_to_rgb(g as f64) as u8;
            let b = utils::linear_to_rgb(b as f64) as u8;

            let index = 4 * (x + y * width) as usize;
            data[index] = r;
            data[index + 1] = g;
            data[index + 2] = b;
            data[index + 3] = 255;
        }
    }

    Ok(data)
}

fn decode_ac(v: f64, maximum: f64) -> [f64; 3] {
    let r = (v / (19.0 * 19.0)).floor();
    let g = (v / 19.0).floor() % 19.0;
    let b = v % 19.0;

    [
        utils::sign_pow((r - 9.0) / 9.0, 2.0) * maximum,
        utils::sign_pow((g - 9.0) / 9.0, 2.0) * maximum,
        utils::sign_pow((b - 9.0) / 9.0, 2.0) * maximum,
    ]
}

fn decode_dc(v: u32) -> [f64; 3] {
    let r = v >> 16;
    let g = (v >> 8) & 255;
    let b = v & 255;
    [
        utils::rgb_to_linear(r as f64),
        utils::rgb_to_linear(g as f64),
        utils::rgb_to_linear(b as f64),
    ]
}
/// Checks if the Blurhash is valid or not.
/// - `hash` -A string representing the blurhash
///
/// # Example
/// ```
/// use blurhash_rs;
///
/// let hash = "LEHV6nWB2yk8pyo0adR*.7kCMdnj";
/// let v = blurhash_rs::validate(hash).unwrap();
///
/// assert_eq!((), v)
/// ```
///
pub fn validate(hash: &str) -> Result<(), DecodeError> {
    if hash.len() < 6 {
        return Err(DecodeError::LengthError);
    }
    let size_flag = base83::decode83(&hash[..1]);
    let num_y = (size_flag / 9.0).floor() + 1.0;
    let num_x = (size_flag % 9.0) + 1.0;

    if hash.len() as f64 != 4.0 + num_y * num_x * 2.0 {
        return Err(DecodeError::FormatError);
    }
    Ok(())
}
