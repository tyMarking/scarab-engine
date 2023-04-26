/// Rendering specific registries
use std::{collections::HashMap, path::PathBuf};

use derivative::Derivative;
use opengl_graphics::{Filter, Texture, TextureSettings};
use serde::{Deserialize, Serialize};

use crate::{
    error::{RenderError, RenderResult},
    ScarabError, ScarabResult,
};

#[derive(Derivative, Serialize)]
#[derivative(Debug)]
/// Wraps a texture along with its source path
pub struct PathTexture {
    #[derivative(Debug = "ignore")]
    #[serde(skip_serializing)]
    texture: Texture,
    path: PathBuf,
}

impl PathTexture {
    /// Pairs a texture and path to the texture
    pub fn new(texture: Texture, path: PathBuf) -> Self {
        Self { texture, path }
    }

    /// Returns a reference to the inner path
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns a reference to the inner texture
    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}

#[derive(Derivative, Deserialize)]
#[derivative(Debug)]
#[serde(try_from = "TextureList")]
/// Owns all loaded textures and can provide a default texture
pub struct TextureRegistry {
    assets_path: PathBuf,
    default_path_texture: PathTexture,
    #[derivative(Debug = "ignore")]
    textures: HashMap<PathBuf, Texture>,
}

impl TextureRegistry {
    /// Creates a new `TextureRegistry` given the default texture path and a list of other textures.
    /// Pre-loads all texture paths given.
    /// Textures default with "Nearest" filtering
    /// The assets_path should be a path to the binary's assets folder
    /// This can be an absolute or relative path.
    /// If a relative path is given, it's assumed to be the path from the binary executable to the assets folder.
    pub fn new(
        assets_path: PathBuf,
        default_path: PathBuf,
        other_texture_paths: &[PathBuf],
    ) -> ScarabResult<Self> {
        let default_texture = Self::load_inner(&assets_path.join(&default_path))?;
        let default_path_texture = PathTexture::new(default_texture, default_path);
        let mut textures = HashMap::new();

        for path in other_texture_paths {
            let texture = Self::load_inner(&assets_path.join(&path))?;
            textures.insert(path.to_path_buf(), texture);
        }

        Ok(Self {
            assets_path,
            default_path_texture,
            textures,
        })
    }

    /// Gets a loaded texture a the given path or the default texture
    pub fn get_or_default(&self, path: &PathBuf) -> &Texture {
        self.textures
            .get(path)
            .unwrap_or_else(|| &self.default_path_texture.texture())
    }

    /// Gets a texture at the given path if it's already loaded
    pub fn get(&self, path: &PathBuf) -> Option<&Texture> {
        if path == self.default_path_texture.path() {
            Some(self.default_path_texture.texture())
        } else {
            self.textures.get(path)
        }
    }

    /// Loads the texture at the path
    /// Note: uses default texture settings
    /// TODO: optionally deserialize texture settings
    /// Returns the previously loaded texture for the path if it exists
    pub fn load(&mut self, path: PathBuf) -> RenderResult<Option<Texture>> {
        let texture = Self::load_inner(&self.assets_path.join(&path))?;
        Ok(self.textures.insert(path, texture))
    }

    fn load_inner(path: &PathBuf) -> RenderResult<Texture> {
        let settings = TextureSettings::new().filter(Filter::Nearest);
        Texture::from_path(path, &settings)
            .or_else(|e| Err(RenderError::CouldNotLoadTexture(path.clone(), e)))
    }
}

impl TryFrom<TextureList> for TextureRegistry {
    type Error = ScarabError;
    fn try_from(value: TextureList) -> ScarabResult<Self> {
        Self::new(
            value.assets_path,
            value.default_texture_path,
            &value.other_texture_paths,
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A list of texture paths loaded in a `TextureRegistry`
pub struct TextureList {
    assets_path: PathBuf,
    default_texture_path: PathBuf,
    other_texture_paths: Vec<PathBuf>,
}

impl From<TextureRegistry> for TextureList {
    fn from(value: TextureRegistry) -> Self {
        Self {
            assets_path: value.assets_path,
            default_texture_path: value.default_path_texture.path().clone(),
            other_texture_paths: value.textures.keys().map(|k| k.clone()).collect(),
        }
    }
}

impl From<&TextureRegistry> for TextureList {
    fn from(value: &TextureRegistry) -> Self {
        Self {
            assets_path: value.assets_path.clone(),
            default_texture_path: value.default_path_texture.path().clone(),
            other_texture_paths: value.textures.keys().map(|k| k.clone()).collect(),
        }
    }
}
