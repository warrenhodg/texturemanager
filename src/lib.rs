extern crate sdl2;

use std::collections::HashMap;
use sdl2::image::LoadTexture;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureAccess, TextureCreator, TextureValueError};
use sdl2::surface::{SurfaceContext};

pub struct TextureManager<'t> {
    textures: HashMap<String, Texture<'t>>,
}

impl <'t> TextureManager<'t> {
    pub fn new() -> TextureManager<'t> {
        TextureManager {
            textures: HashMap::new(),
        }
    }

    pub fn create_texture (&mut self, texture_creator: &'static TextureCreator<SurfaceContext<'t>>, name: String, format: PixelFormatEnum, access: TextureAccess, width: u32, height: u32) -> Result<(), TextureValueError> { 
        let texture_result = texture_creator.create_texture(format, access, width, height);

        match texture_result {
            Err(v) => return Err(v),
            Ok(texture) => {
                self.textures.insert(name, texture);
                return Ok(())
            }
        }
    }

    pub fn load_texture(&mut self, texture_creator: &'static TextureCreator<SurfaceContext<'t>>, name: String, filename: String) -> Result<(), String> {
        let texture_result = texture_creator.load_texture(filename);

        match texture_result {
            Err(v) => return Err(v),
            Ok(texture) => {
                self.textures.insert(name, texture);
                return Ok(())
            }
        }
    }

    pub fn texture(&mut self, name: String) -> Option<&mut Texture<'t>> {
        self.textures.get_mut(&name)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
