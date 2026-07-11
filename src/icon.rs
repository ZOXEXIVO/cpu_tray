use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use ab_glyph::{FontRef, PxScale};
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
        *self
            .icon_cache
            .entry(value)
            .or_insert_with(|| IconGenerator::create_icon(value))
    }

    fn scale_params(n: usize) -> ((i32, i32), PxScale) {
        match n {
            1 => {
                ((5, -2), PxScale { x: 30.0, y: 27.0 })
            }
            2 => {
                ((-1, -2), PxScale { x: 30.0, y: 27.0 })
            }
            _ => {
                ((-2,3), PxScale { x: 20.0, y: 21.0 })
            }
        }
    }

    fn create_icon(value: u8) -> GeneratedIcon {
        let value_to_draw = value.to_string();

        let mut image = RgbaImage::new(24, 24);

        let font = FontRef::try_from_slice(include_bytes!("fonts/OpenSans-Semibold.ttf")).unwrap();

        let scale_params = IconGenerator::scale_params(value_to_draw.len());

        let coord = scale_params.0;

        let color = match value {
            0..=39 => Rgba([255u8, 255u8, 255u8, 255u8]),
            40..=79 => Rgba([0u8, 165u8, 255u8, 255u8]),
            _ => Rgba([0u8, 0u8, 255u8, 255u8]),
        };

        draw_text_mut(
            &mut image,
            color,
            coord.0, coord.1,
            scale_params.1,
            &font,
            &value_to_draw,
        );

        let resized_image = image; //resize(&mut image, 16, 16, image::imageops::FilterType::Lanczos3);

        unsafe {
            let screen_dc = winapi::um::winuser::GetDC(null_mut());
            let hbm_mask = winapi::um::wingdi::CreateCompatibleBitmap(screen_dc, 24, 24);

            let mut bytes = resized_image.into_raw();
            let bits = bytes.as_mut_ptr() as *mut winapi::ctypes::c_void;

            let bitmap: winapi::shared::windef::HBITMAP =
                winapi::um::wingdi::CreateBitmap(24, 24, 2, 16, bits);

            let mut h_icon = winapi::um::winuser::ICONINFO {
                fIcon: 1,
                hbmColor: bitmap,
                hbmMask: hbm_mask,
                xHotspot: 0,
                yHotspot: 0,
            };

            let icon = winapi::um::winuser::CreateIconIndirect(&mut h_icon);

            // CreateIconIndirect makes its own copy of the bitmaps, so the
            // sources (and the screen DC) must be released to avoid leaking a
            // GDI object on every generated icon.
            winapi::um::wingdi::DeleteObject(bitmap as winapi::shared::windef::HGDIOBJ);
            winapi::um::wingdi::DeleteObject(hbm_mask as winapi::shared::windef::HGDIOBJ);
            winapi::um::winuser::ReleaseDC(null_mut(), screen_dc);

            icon
        }
    }
}
