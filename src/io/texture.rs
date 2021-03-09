
use std::path::Path;
use crate::io::*;
use crate::definition::*;

impl Deserialize {
    ///
    /// Deserialize the loaded image resource at the given path into a [CPUTexture](crate::CPUTexture) using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CPUTexture can then be used to create a [Texture2D](crate::Texture2D).
    ///
    /// # Feature
    /// Only available when the `image-io` feature is enabled.
    ///
    pub fn image<P: AsRef<Path>>(loaded: &Loaded, path: P) -> Result<CPUTexture<u8>, IOError> {
        use image::GenericImageView;
        let img = image::load_from_memory(loaded.get(path)?)?;
        let bytes = img.to_bytes();
        let number_of_channels = bytes.len() / (img.width() * img.height()) as usize;
        let format = match number_of_channels {
            1 => Ok(Format::R8),
            3 => Ok(Format::RGB8),
            4 => Ok(Format::RGBA8),
            _ => Err(IOError::FailedToLoad {message: format!("Could not determine the pixel format for the texture.")})
        }?;

        Ok(CPUTexture {data: bytes, width: img.width() as usize, height: img.height() as usize, format, ..Default::default()})
    }

    ///
    /// Deserialize the 6 loaded image resources at the given paths into a [CPUTexture](crate::CPUTexture) using
    /// the [image](https://crates.io/crates/image/main.rs) crate.
    /// The CPUTexture can then be used to create a [TextureCubeMap](crate::TextureCubeMap).
    ///
    /// # Feature
    /// Only available when the `image-io` feature is enabled.
    ///
    pub fn cube_image<P: AsRef<Path>>(loaded: &Loaded, right_path: P, left_path: P,
                                      top_path: P, bottom_path: P, front_path: P, back_path: P) -> Result<CPUTexture<u8>, IOError> {
        let mut right = Self::image(loaded, right_path)?;
        let left = Self::image(loaded, left_path)?;
        let top = Self::image(loaded, top_path)?;
        let bottom = Self::image(loaded, bottom_path)?;
        let front = Self::image(loaded, front_path)?;
        let back = Self::image(loaded, back_path)?;

        right.data.extend(left.data);
        right.data.extend(top.data);
        right.data.extend(bottom.data);
        right.data.extend(front.data);
        right.data.extend(back.data);
        Ok(right)
    }
}