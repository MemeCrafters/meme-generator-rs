use crate::meme::ToMemeOptions;
use serde::Deserialize;

#[derive(ToMemeOptions, Deserialize)]
#[serde(default)]
pub(crate) struct NoOptions {}

#[derive(ToMemeOptions, Deserialize)]
#[serde(default)]
pub(crate) struct Circle {
    /// 是否将图片变为圆形
    #[option(short, long, short_aliases = ['圆'])]
    pub circle: bool,
}

#[derive(ToMemeOptions, Deserialize)]
#[serde(default)]
pub(crate) struct Gender {
    /// 性别
    #[option(short, long, default = "unknown", choices = ["male", "female", "unknown"])]
    pub gender: String,
}
