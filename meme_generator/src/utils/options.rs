use crate::meme::MemeOptions;

#[derive(MemeOptions)]
pub(crate) struct NoOptions {}

#[derive(MemeOptions)]
pub(crate) struct Circle {
    /// 是否将图片变为圆形
    #[option(short, long, short_aliases = ['圆'])]
    pub circle: bool,
}

#[derive(MemeOptions)]
pub(crate) struct Gender {
    /// 性别
    #[option(short, long, default = "unknown", choices = ["male", "female", "unknown"])]
    pub gender: String,
}
