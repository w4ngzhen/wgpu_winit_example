use image::GenericImageView;
use std::fs::File;
use std::io;
use std::io::Read;

pub struct RgbaImg {
    pub width: u32,
    pub height: u32,
    pub bytes: Vec<u8>,
}

impl RgbaImg {
    pub fn new(file_path: &str) -> Option<Self> {
        if let Ok(file_bytes) = read_file_to_memory(file_path) {
            let dynamic_img = image::load_from_memory(&file_bytes[..]).unwrap();
            let rgba_img = dynamic_img.to_rgba8();
            let (width, height) = dynamic_img.dimensions();
            Some(Self {
                width,
                height,
                bytes: rgba_img.into_raw(),
            })
        } else {
            None
        }
    }
}

fn read_file_to_memory(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    // 读取文件内容到缓冲区
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
mod test {
    use crate::img_utils::RgbaImg;

    #[test]
    pub fn test_valid_img() {
        // 请确保assets/目录下存在一个合法的example-img.png（320x320的图片）
        let img = RgbaImg::new("assets/example-img.png");
        assert!(img.is_some());
        if let Some(img) = img {
            assert_eq!(img.width, 320);
            assert_eq!(img.height, 320);
        }
    }

    #[test]
    pub fn test_not_exists() {
        assert!(RgbaImg::new("abc/efg/img.png").is_none());
    }
}
