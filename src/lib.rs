#[derive(Debug)]
pub struct Image{
    width: u32,
    height: u32,
    pixels: Vec<u8>
}

impl Image{
    pub fn new(width: u32, height:u32, pixels: Vec<u8>) -> Result<Self, String>{
        let expected = (width as usize)
            .checked_mul(height as usize)
            .and_then(|n| n.checked_mul(4))
            .ok_or("image dimensions are too large")?;

        if pixels.len() != expected {
            return Err(format!(
                "expected {} bytes for a {}x{} image, got {}",
                expected, width, height, pixels.len()
            ));
        }

        Ok(Image { width, height, pixels })
    }
}