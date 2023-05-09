use graphics::{
    types::{Color, Rectangle, SourceRectangle},
    Image,
};
use opengl_graphics::{Filter, TextureSettings, Wrap};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "Image")]
pub struct ImageDef {
    pub color: Option<Color>,
    pub rectangle: Option<Rectangle>,
    pub source_rectangle: Option<SourceRectangle>,
}

impl From<ImageDef> for Image {
    fn from(value: ImageDef) -> Self {
        Self {
            color: value.color,
            rectangle: value.rectangle,
            source_rectangle: value.source_rectangle,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "TextureSettings")]
pub struct TextureSettingsDef {
    #[serde(getter = "TextureSettings::get_convert_gamma")]
    convert_gamma: bool,
    #[serde(getter = "TextureSettings::get_compress")]
    compress: bool,
    #[serde(getter = "TextureSettings::get_generate_mipmap")]
    generate_mipmap: bool,
    #[serde(with = "FilterDef")]
    #[serde(getter = "TextureSettings::get_min")]
    min: Filter,
    #[serde(with = "FilterDef")]
    #[serde(getter = "TextureSettings::get_mag")]
    mag: Filter,
    #[serde(with = "FilterDef")]
    #[serde(getter = "TextureSettings::get_mipmap")]
    mipmap: Filter,
    #[serde(with = "WrapDef")]
    #[serde(getter = "TextureSettings::get_wrap_u")]
    wrap_u: Wrap,
    #[serde(with = "WrapDef")]
    #[serde(getter = "TextureSettings::get_wrap_v")]
    wrap_v: Wrap,
    #[serde(getter = "TextureSettings::get_border_color")]
    border_color: [f32; 4],
}

impl From<TextureSettingsDef> for TextureSettings {
    fn from(value: TextureSettingsDef) -> Self {
        TextureSettings::new()
            .convert_gamma(value.convert_gamma)
            .compress(value.compress)
            .generate_mipmap(value.generate_mipmap)
            .min(value.min)
            .mag(value.mag)
            .mipmap(value.mipmap)
            .wrap_u(value.wrap_u)
            .wrap_v(value.wrap_v)
            .border_color(value.border_color)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Filter")]
pub enum FilterDef {
    Linear,
    Nearest,
}

impl From<FilterDef> for Filter {
    fn from(value: FilterDef) -> Self {
        match value {
            FilterDef::Linear => Self::Linear,
            FilterDef::Nearest => Self::Nearest,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Wrap")]
pub enum WrapDef {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder,
}

impl From<WrapDef> for Wrap {
    fn from(value: WrapDef) -> Self {
        match value {
            WrapDef::Repeat => Self::Repeat,
            WrapDef::MirroredRepeat => Self::Repeat,
            WrapDef::ClampToEdge => Self::ClampToEdge,
            WrapDef::ClampToBorder => Self::ClampToBorder,
        }
    }
}
