use crate::{base83, utils};
use std::f64::consts::PI;

/// Encode Error
#[derive(Debug)]
pub enum EncodeError {
    /// `component_x` OR `component_y` is not between `1` ANd `9`.
    RangError,
    /// the ` width * height * 4` is not match pixels.len().
    NoMatch,
}

const BYTES_PER_PIXEL: u32 = 4;

/// # This function returns a string containing the BlurHash.
///
/// * `pixels`- A pointer to the pixel data. This is supplied in RGBA order, with 4 bytes per pixels.
/// * `width` - The width in pixels of the supplied image.
/// * `height` - The height in pixels of the supplied image.
/// * `component_x` - The number of components in the X direction. Must be between 1 and 9. batter is 3 to 5.
/// * `component_y` - The number of components in the Y direction. Must be between 1 and 9. batter is 3 to 5.
///
/// # Examples
/// ```
/// use image::{open, GenericImageView};
/// use std::path::Path;
/// use blurhash_rs::encode;
///
/// let img = open(&Path::new("./test/test1.jpg")).expect("not found");
/// let width = img.dimensions().0;
/// let height = img.dimensions().1;
/// let mut data = Vec::with_capacity((width * height) as usize);
///
/// for p in img.pixels() {
///     let p2 = p.2;
///     data.push(p2.0[0]);
///     data.push(p2.0[1]);
///     data.push(p2.0[2]);
///     data.push(p2.0[3]);
/// }
/// let v = encode(&data, width, height, 4, 3);
///
/// assert_eq!(v.unwrap(), "LMRC9h$u}89I0~RV^*E1F5xxxwDi");
/// ```
pub fn encode(
    pixels: &Vec<u8>,
    width: u32,
    height: u32,
    component_x: u8,
    component_y: u8,
) -> Result<String, EncodeError> {
    if component_x < 1 || component_x > 9 || component_y < 1 || component_y > 9 {
        return Err(EncodeError::RangError);
    }

    if width * height * 4 != pixels.len() as u32 {
        return Err(EncodeError::NoMatch);
    }

    let mut factors: Vec<[f64; 3]> = Vec::new();
    for y in 0..component_y {
        for x in 0..component_x {
            let normalization = if x == 0 && y == 0 { 1 } else { 2 };
            let factor = multiply_basic(pixels, width, height, x, y, normalization);
            factors.push(factor);
        }
    }

    let dc = factors[0];
    let ac = &factors[1..];

    let size_flag = component_x - 1 + (component_y - 1) * 9;

    let mut hash = base83::encode83(size_flag as u32, 1);

    let maximum = if ac.len() > 0 {
        let actual_max = ac
            .iter()
            .map(|v3| v3.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())
            .collect::<Vec<_>>();
        let actual_max = actual_max
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let questioned_max = (82.0f64.min(**actual_max * 166.0 - 0.5).floor() as u32).max(0);
        hash += &base83::encode83(questioned_max, 1);
        (questioned_max + 1) as f64 / 166f64
    } else {
        hash += base83::encode83(0, 1).as_str();
        1f64
    };

    hash += &base83::encode83(encode_dc(dc), 4);
    for factor in ac.iter() {
        hash += &base83::encode83(encode_ac(factor, maximum), 2)
    }
    Ok(hash)
}

fn encode_ac(ac: &[f64; 3], max: f64) -> u32 {
    ac.into_iter()
        .map(|x| {
            (utils::sign_pow(x / max, 0.5) * 9f64 + 9.5)
                .floor()
                .min(18.0)
                .max(0.0)
        })
        .reduce(|a, b| a * 19.0 + b)
        .unwrap()
        .floor() as u32
}

fn encode_dc(dc: [f64; 3]) -> u32 {
    let v = dc
        .iter()
        .map(|a| utils::linear_to_rgb(*a))
        .collect::<Vec<_>>();
    ((v.get(0).unwrap().floor() as u32) << 16)
        + ((v.get(1).unwrap().floor() as u32) << 8)
        + (v.get(2).unwrap().floor() as u32)
}

// fn encode_dc() {}

fn multiply_basic(
    pixels: &Vec<u8>,
    width: u32,
    height: u32,
    component_x: u8,
    component_y: u8,
    normalization: u32,
) -> [f64; 3] {
    let mut r = 0f64;
    let mut g = 0f64;
    let mut b = 0f64;

    let bytes_per_row = width * BYTES_PER_PIXEL;
    for y in 0..height {
        for x in 0..width {
            let basic = (PI * component_x as f64 * x as f64 / width as f64).cos()
                * (PI * component_y as f64 * y as f64 / height as f64).cos();
            let index: usize = (BYTES_PER_PIXEL * x + y * bytes_per_row) as usize;

            r += basic * utils::rgb_to_linear(*pixels.get(index).unwrap() as f64);
            g += basic * utils::rgb_to_linear(*pixels.get((index + 1) as usize).unwrap() as f64);
            b += basic * utils::rgb_to_linear(*pixels.get((index + 2) as usize).unwrap() as f64);
        }
    }

    let scale = normalization as f64 / ((width * height) as f64);

    [r * scale, g * scale, b * scale]
}
