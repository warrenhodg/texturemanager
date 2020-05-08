/*
 * Extract from https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/resource-manager.rs
 */

extern crate sdl2;

use sdl2::image::LoadTexture;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::ttf::{Font, Sdl2TtfContext};

use std::borrow::Borrow;
use std::fmt::Display;
use std::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;

// Generic trait to Load any Resource Kind
pub trait ResourceLoader<'l, R> {
    type Args: ?Sized;
    fn load(&'l self, details: &Self::Args) -> Result<R, String>;
}

// Generic struct to cache any resource loaded by a ResourceLoader
pub struct ResourceManager<'l, K, R, L>
    where K: Hash + Display + Eq,
          L: 'l + ResourceLoader<'l, R>
{
    loader: &'l L,
    cache: HashMap<K, Rc<R>>,
}

// Manage textures
pub type TextureManager<'l, T> = ResourceManager<'l, String, Texture<'l>, TextureCreator<T>>;

// Manage fonts
pub type FontManager<'l> = ResourceManager<'l, String, Font<'l, 'static>, Sdl2TtfContext>;


impl<'l, K, R, L> ResourceManager<'l, K, R, L>
    where K: Hash + Display + Eq,
          L: ResourceLoader<'l, R>
{
    pub fn new(loader: &'l L) -> Self {
        ResourceManager {
            cache: HashMap::new(),
            loader: loader,
        }
    }

    // load is designed to use more specific load function to load the resource,
    // and add it to the hashmap using the specified name.
    //
    // Generics magic to allow a HashMap to use String as a key
    // while allowing it to use &str for gets
    pub fn load<D>(&mut self, name: &str, details: &D) -> Result<Rc<R>, String>
        where 
            D: ?Sized,
            L: ResourceLoader<'l, R, Args = D>,
            K: Borrow<str> + for<'a> From<&'a str>,
    {
        self.cache
            .get(name)
            .cloned()
            .map_or_else(|| {
                let resource = Rc::new(self.loader.load(details)?);
                self.cache.insert(name.into(), resource.clone());
                Ok(resource)
            },
            Ok)
    }
    
    // add adds an externally created resource to the HashMap using the specified name
    //
    // Generics magic to allow a HashMap to use String as a key
    // while allowing it to use &str for gets
    pub fn add(&mut self, name: &str, item: R) -> Result<Rc<R>, String>
        where K: Borrow<str> + for<'a> From<&'a str>,
    {
        self.cache
            .get(name)
            .cloned()
            .map_or_else(|| {
                             let resource = Rc::new(item);
                             self.cache.insert(name.into(), resource.clone());
                             Ok(resource)
                         },
                         Ok)
    }

    // get retrieves the specified resource from the HashMap
    pub fn get(&mut self, name: &str) -> Result<Rc<R>, String>
        where K: Borrow<str> + for<'a> From<&'a str>,
    {
        match self
            .cache
            .get(name)
            .cloned() {
            None => Err(format!("{} not found", name).to_string()),
            Some(v) => Ok(v),
        }
    }

}

// TextureCreator knows how to load Textures
impl<'l, R> ResourceLoader<'l, Texture<'l>> for TextureCreator<R> {
    type Args = str;
    fn load(&'l self, path: &str) -> Result<Texture, String> {
        self.load_texture(path)
    }
}

// Font Context knows how to load Fonts
impl<'l> ResourceLoader<'l, Font<'l, 'static>> for Sdl2TtfContext {
    type Args = FontDetails;
    fn load(&'l self, details: &FontDetails) -> Result<Font<'l, 'static>, String> {
        self.load_font(&details.path, details.size)
    }
}

// Information needed to load a Font
#[derive(PartialEq, Eq, Hash)]
pub struct FontDetails {
    pub path: String,
    pub size: u16,
}

impl<'a> From<&'a FontDetails> for FontDetails {
    fn from(details: &'a FontDetails) -> FontDetails {
        FontDetails {
            path: details.path.clone(),
            size: details.size,
        }
    }
}
