use meme_generator_utils::builder::MemeOptions;

#[derive(MemeOptions)]
pub(crate) struct NoOptions {}

#[derive(MemeOptions)]
pub(crate) struct Circle {
    /// 是否将图片变为圆形
    #[option(short, long, short_aliases = ['圆'], default=false)]
    pub circle: Option<bool>,
}

#[derive(MemeOptions)]
pub(crate) struct Gender {
    /// 性别
    #[option(short, long, default = "unknown", choices = ["male", "female", "unknown"])]
    pub gender: Option<String>,
}

macro_rules! number_option {
    ($name:ident, $min:tt, $max:tt) => {
        #[derive(MemeOptions)]
        struct $name {
            /// 图片编号
            #[option(short, long, minimum = $min, maximum = $max)]
            number: Option<i32>,
        }
    };
}

pub(crate) use number_option;
