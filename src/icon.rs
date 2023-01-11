use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use image::imageops::resize;
use rusttype::{Scale, Font};
use std::ptr::null_mut;

use std::collections::HashMap;

pub type GeneratedIcon = *mut winapi::shared::windef::HICON__;

pub struct IconGenerator {
    icon_cache: HashMap<u8, GeneratedIcon>,
}

impl IconGenerator {
    pub fn new() -> Self {
        IconGenerator {
            icon_cache: HashMap::with_capacity(100),
        }
    }

    pub fn generate(&mut self, value: u8) -> GeneratedIcon {
        if self.icon_cache.contains_key(&value) {
            return self.icon_cache[&value];
        } else {
            let new_icon = IconGenerator::create_icon(value);

            self.icon_cache.insert(value, new_icon);

            new_icon
        }
    }

    fn scale_params(n: usize) -> ((i32, i32), Scale) {
        match n {
            1 => {
                ((24, 0), Scale { x: 128.0, y: 128.0 })
            }
            2 => {
                ((0, 0), Scale { x: 120.0, y: 120.0 })
            }
            _ => {
                ((0, 20), Scale { x: 80.0, y: 80.0 })
            }
        }
    }

    fn create_icon(value: u8) -> GeneratedIcon {
        let value_to_draw = value.to_string();

        let mut image = RgbaImage::new(128, 128);

        let font = Font::try_from_bytes(include_bytes!("fonts/Arial.ttf")).unwrap();

        let scale_params = IconGenerator::scale_params(value_to_draw.len());

        let coord = scale_params.0;

        draw_text_mut(
            &mut image,
            Rgba([255u8, 255u8, 255u8, 255u8]),
            coord.0, coord.1,
            scale_params.1,
            &font,
            &value_to_draw,
        );

        let resized_image = resize(
            &mut image,
            32,
            32,
            image::imageops::FilterType::Lanczos3,
        );

        unsafe {
            let hbm_mask = winapi::um::wingdi::CreateCompatibleBitmap(
                winapi::um::winuser::GetDC(null_mut()),
                32,
                32,
            );

            let bytes_raw = resized_image.into_raw().as_mut_ptr();

            let transmuted = std::mem::transmute::<*mut u8, *mut winapi::ctypes::c_void>(bytes_raw);

            let bitmap: winapi::shared::windef::HBITMAP =
                winapi::um::wingdi::CreateBitmap(32, 32, 2, 16, transmuted);

            let mut h_icon = winapi::um::winuser::ICONINFO {
                fIcon: 1,
                hbmColor: bitmap,
                hbmMask: hbm_mask,
                xHotspot: 0,
                yHotspot: 0,
            };

            winapi::um::winuser::CreateIconIndirect(&mut h_icon)
        }
    }
}
