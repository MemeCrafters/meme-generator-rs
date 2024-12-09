pub mod common_used_options {
    use crate::meme::MemeOptions;
    use serde::Deserialize;

    #[derive(MemeOptions, Deserialize)]
    #[serde(default)]
    pub struct NoOptions {}

    #[derive(MemeOptions, Deserialize)]
    #[serde(default)]
    pub struct Circle {
        /// 是否将图片变为圆形
        #[option(short, long, short_aliases = ['圆'])]
        pub circle: bool,
    }

    #[derive(MemeOptions, Deserialize)]
    #[serde(default)]
    pub struct Gender {
        /// 性别
        #[option(short, long, default = "unknown", choices = ["male", "female", "unknown"])]
        pub gender: String,
    }
}
