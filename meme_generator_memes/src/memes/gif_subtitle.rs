use std::fs::read;

use skia_safe::{Codec, Color, Data};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    config::IMAGES_DIR,
    decoder::CodecExt,
    encoder::GifEncoder,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{local_date, new_paint, new_stroke_paint},
};

use crate::{options::NoOptions, register_meme};

fn gif_subtitle(
    texts: &Vec<String>,
    pieces: &[(usize, usize)],
    template_name: &str,
    font_size: f32,
) -> Result<Vec<u8>, Error> {
    let path = format!("gif_subtitle/{template_name}.gif");
    let image_path = IMAGES_DIR.join(&path);
    if !(image_path.exists() && image_path.is_file()) {
        return Err(Error::ImageAssetMissing(path));
    }
    let data = Data::new_copy(&read(&image_path).unwrap());
    let mut codec =
        Codec::from_data(data).ok_or(Error::ImageDecodeError("Skia decode error".to_string()))?;

    let mut encoder = GifEncoder::new();
    let duration = codec.get_average_duration()?;
    for i in 0..codec.get_frame_count() {
        let mut frame = codec.get_frame(i)?;

        for (text_i, &(start, end)) in pieces.iter().enumerate() {
            if i >= start && i < end {
                let mut surface = frame.to_surface();
                let canvas = surface.canvas();
                let padding_y = 5.0;
                let text2image = Text2Image::from_text(
                    &texts[text_i],
                    font_size,
                    text_params!(
                        paint = new_paint(Color::WHITE),
                        stroke_paint = new_stroke_paint(Color::BLACK, font_size / 10.0),
                    ),
                );
                text2image.draw_on_canvas(
                    &canvas,
                    (
                        (frame.width() as f32 - text2image.longest_line()) / 2.0,
                        frame.height() as f32 - padding_y - text2image.height(),
                    ),
                );
                frame = surface.image_snapshot();
                break;
            }
        }

        encoder.add_frame(frame, duration)?;
    }

    Ok(encoder.finish()?)
}

macro_rules! register_gif_subtitle {
    ($key:expr, $keywords:expr, $pieces:expr, $default_texts:expr, $font_size:expr, $date_created:expr, $date_modified:expr, $(,)?
    ) => {
        register_meme!(
            $key,
            |_: Vec<InputImage>, texts: Vec<String>, _: NoOptions| -> Result<Vec<u8>, Error> {
                gif_subtitle(&texts, $pieces, $key, $font_size)
            },
            min_texts = $pieces.len() as u8,
            max_texts = $pieces.len() as u8,
            default_texts = $default_texts,
            keywords = $keywords,
            date_created = $date_created,
            date_modified = $date_modified,
        );
    };
}

register_gif_subtitle!(
    "wangjingze",
    &["王境泽"],
    &[(0, 9), (12, 24), (25, 35), (37, 48)],
    &[
        "我就是饿死",
        "死外边 从这里跳下去",
        "不会吃你们一点东西",
        "真香"
    ],
    20.0,
    local_date(2021, 12, 24),
    local_date(2023, 2, 14),
);

register_gif_subtitle!(
    "weisuoyuwei",
    &["为所欲为"],
    &[
        (11, 14),
        (27, 38),
        (42, 61),
        (63, 81),
        (82, 95),
        (96, 105),
        (111, 131),
        (145, 157),
        (157, 167),
    ],
    &[
        "好啊",
        "别说我是一等良民",
        "就算你们真的想要诬告我",
        "我有的是钱请律师帮我打官司",
        "我想我根本不用坐牢",
        "有钱了不起啊",
        "Sorry，有钱真的了不起",
        "不过我想你不会明白这种感觉",
        "不明白，不明白",
    ],
    19.0,
    local_date(2022, 1, 19),
    local_date(2023, 2, 14),
);

register_gif_subtitle!(
    "chanshenzi",
    &["馋身子"],
    &[(0, 16), (16, 31), (33, 40)],
    &["你那叫喜欢吗？", "你那是馋她身子", "你下贱！"],
    18.0,
    local_date(2022, 1, 19),
    local_date(2023, 2, 14),
);

register_gif_subtitle!(
    "qiegewala",
    &["切格瓦拉"],
    &[(0, 15), (16, 31), (31, 38), (38, 48), (49, 68), (68, 86)],
    &[
        "没有钱啊 肯定要做的啊",
        "不做的话没有钱用",
        "那你不会去打工啊",
        "有手有脚的",
        "打工是不可能打工的",
        "这辈子不可能打工的",
    ],
    20.0,
    local_date(2022, 1, 19),
    local_date(2023, 2, 14),
);

register_gif_subtitle!(
    "shuifandui",
    &["谁反对"],
    &[(3, 14), (21, 26), (31, 38), (40, 45)],
    &["我话说完了", "谁赞成", "谁反对", "我反对"],
    19.0,
    local_date(2022, 1, 19),
    local_date(2023, 2, 14),
);

register_gif_subtitle!(
    "zengxiaoxian",
    &["曾小贤"],
    &[(3, 15), (24, 30), (30, 46), (56, 63)],
    &["平时你打电子游戏吗", "偶尔", "星际还是魔兽", "连连看"],
    21.0,
    local_date(2022, 1, 19),
    local_date(2023, 2, 14),
);

register_gif_subtitle!(
    "yalidaye",
    &["压力大爷"],
    &[(0, 16), (21, 47), (52, 77)],
    &[
        "外界都说我们压力大",
        "我觉得吧压力也没有那么大",
        "主要是28岁了还没媳妇儿"
    ],
    21.0,
    local_date(2022, 1, 19),
    local_date(2023, 2, 14),
);

register_gif_subtitle!(
    "nihaosaoa",
    &["你好骚啊"],
    &[(0, 14), (16, 26), (42, 61)],
    &["既然追求刺激", "就贯彻到底了", "你好骚啊"],
    17.0,
    local_date(2022, 1, 19),
    local_date(2023, 2, 14),
);

register_gif_subtitle!(
    "shishilani",
    &["食屎啦你"],
    &[(14, 21), (23, 36), (38, 46), (60, 66)],
    &[
        "穿西装打领带",
        "拿大哥大有什么用",
        "跟着这样的大哥",
        "食屎啦你"
    ],
    17.0,
    local_date(2022, 1, 19),
    local_date(2023, 2, 14),
);

register_gif_subtitle!(
    "wunian",
    &["五年怎么过的"],
    &[(11, 20), (35, 50), (59, 77), (82, 95)],
    &[
        "五年",
        "你知道我这五年是怎么过的吗",
        "我每天躲在家里玩贪玩蓝月",
        "你知道有多好玩吗",
    ],
    16.0,
    local_date(2022, 1, 19),
    local_date(2023, 2, 14),
);

register_gif_subtitle!(
    "maikease",
    &["麦克阿瑟说"],
    &[(0, 22), (24, 46), (48, 70), (72, 84)],
    &[
        "美国前五星上将麦克阿瑟",
        "曾这样评价道",
        "如果让我去阻止xxx",
        "那么我宁愿去阻止上帝",
    ],
    20.0,
    local_date(2023, 7, 30),
    local_date(2023, 7, 30),
);

register_gif_subtitle!(
    "jiamianqishi",
    &["假面骑士"],
    &[(2, 9), (14, 20), (27, 40), (45, 62)],
    &["哦～", "（飞扑）", "一直想看你这幅表情", "这幅嫉妒我的表情"],
    17.0,
    local_date(2024, 10, 30),
    local_date(2024, 10, 30),
);

register_gif_subtitle!(
    "wuyage",
    &["乌鸦哥"],
    &[(8, 32), (35, 58), (60, 85)],
    &["哟 云崽机器人", "今天掉线了没", "来给他弹个版本过低"],
    18.0,
    local_date(2024, 12, 5),
    local_date(2024, 12, 5),
);
