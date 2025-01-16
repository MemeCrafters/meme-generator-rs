use rand::{seq::SliceRandom, Rng};
use skia_safe::{Color, Point};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::{InputImage, MemeOptions},
    encoder::encode_png,
    image::ImageExt,
    shortcut,
    text::Text2Image,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint, new_stroke_paint},
};

use crate::{register_meme, tags::MemeTags};

struct Character<'c> {
    name_cn: &'c str,
    name_en: &'c str,
    color: &'c str,
    img_num: i32,
}

const CHARACTERS: [Character<'_>; 26] = [
    Character {
        name_cn: "爱莉",
        name_en: "airi",
        color: "#FB8AAC",
        img_num: 15,
    },
    Character {
        name_cn: "彰人",
        name_en: "akito",
        color: "#FF7722",
        img_num: 13,
    },
    Character {
        name_cn: "杏",
        name_en: "an",
        color: "#00BADC",
        img_num: 13,
    },
    Character {
        name_cn: "梦",
        name_en: "emu",
        color: "#FF66BB",
        img_num: 13,
    },
    Character {
        name_cn: "绘名",
        name_en: "ena",
        color: "#B18F6C",
        img_num: 16,
    },
    Character {
        name_cn: "遥",
        name_en: "haruka",
        color: "#6495F0",
        img_num: 13,
    },
    Character {
        name_cn: "穗波",
        name_en: "honami",
        color: "#F86666",
        img_num: 15,
    },
    Character {
        name_cn: "一歌",
        name_en: "ichika",
        color: "#33AAEE",
        img_num: 15,
    },
    Character {
        name_cn: "KAITO",
        name_en: "kaito",
        color: "#3366CC",
        img_num: 13,
    },
    Character {
        name_cn: "奏",
        name_en: "kanade",
        color: "#BB6688",
        img_num: 14,
    },
    Character {
        name_cn: "心羽",
        name_en: "kohane",
        color: "#FF6699",
        img_num: 14,
    },
    Character {
        name_cn: "连",
        name_en: "len",
        color: "#D3BD00",
        img_num: 14,
    },
    Character {
        name_cn: "流歌",
        name_en: "luka",
        color: "#F88CA7",
        img_num: 13,
    },
    Character {
        name_cn: "真冬",
        name_en: "mafuyu",
        color: "#7171AF",
        img_num: 14,
    },
    Character {
        name_cn: "MEIKO",
        name_en: "meiko",
        color: "#E4485F",
        img_num: 13,
    },
    Character {
        name_cn: "初音未来",
        name_en: "miku",
        color: "#33CCBB",
        img_num: 13,
    },
    Character {
        name_cn: "实乃理",
        name_en: "minori",
        color: "#F39E7D",
        img_num: 14,
    },
    Character {
        name_cn: "瑞希",
        name_en: "mizuki",
        color: "#CA8DB6",
        img_num: 14,
    },
    Character {
        name_cn: "宁宁",
        name_en: "nene",
        color: "#19CD94",
        img_num: 13,
    },
    Character {
        name_cn: "铃",
        name_en: "rin",
        color: "#E8A505",
        img_num: 13,
    },
    Character {
        name_cn: "类",
        name_en: "rui",
        color: "#BB88EE",
        img_num: 16,
    },
    Character {
        name_cn: "咲希",
        name_en: "saki",
        color: "#F5B303",
        img_num: 15,
    },
    Character {
        name_cn: "志步",
        name_en: "shiho",
        color: "#A0C10B",
        img_num: 15,
    },
    Character {
        name_cn: "雫",
        name_en: "shizuku",
        color: "#5CD0B9",
        img_num: 13,
    },
    Character {
        name_cn: "冬弥",
        name_en: "touya",
        color: "#0077DD",
        img_num: 15,
    },
    Character {
        name_cn: "司",
        name_en: "tsukasa",
        color: "#F09A04",
        img_num: 15,
    },
];

#[derive(MemeOptions)]
struct Options {
    /// 角色名
    #[option(short, long, choices = ["airi", "akito", "an", "emu", "ena", "haruka", "honami", "ichika", "kaito", "kanade", "kohane", "len", "luka", "mafuyu", "meiko", "miku", "minori", "mizuki", "nene", "rin", "rui", "saki", "shiho", "shizuku", "touya", "tsukasa"])]
    pub character: Option<String>,

    /// 图片编号
    #[option(short, long, minimum = 1, maximum = 16)]
    pub number: Option<i32>,

    /// 字体大小
    #[option(short, long, minimum = 20, maximum = 50)]
    pub size: Option<i32>,

    /// 文字旋转角度
    #[option(short, long, minimum = -40, maximum = 40)]
    pub rotate: Option<i32>,

    /// x方向偏移
    #[option(long, default = 0, minimum = -100, maximum = 100)]
    pub x_offset: Option<i32>,

    /// y方向偏移
    #[option(long, default = 0, minimum = -50, maximum = 150)]
    pub y_offset: Option<i32>,
}

fn pjsk(_: Vec<InputImage>, texts: Vec<String>, options: Options) -> Result<Vec<u8>, Error> {
    let text = &texts[0];

    let character = match options.character {
        None => {
            let mut rng = rand::thread_rng();
            CHARACTERS.choose(&mut rng).unwrap()
        }
        Some(name) => CHARACTERS.iter().find(|c| c.name_en == name).unwrap(),
    };

    let num = match options.number {
        None => rand::thread_rng().gen_range(1..=character.img_num),
        Some(n) => {
            if n < 1 || n > character.img_num {
                return Err(Error::MemeFeedback(format!(
                    "角色 {} 的图片编号错误，请输入 1-{}",
                    character.name_cn, character.img_num
                )));
            }
            n
        }
    };

    let frame = load_image(format!(
        "pjsk/{}/{}",
        character.name_en,
        format!("{:02}.png", num)
    ))?;
    let color = color_from_hex_code(character.color);

    let font_size = options.size.unwrap_or(50);
    let text2image = Text2Image::from_text(
        text,
        font_size as f32,
        text_params!(
            paint = new_paint(color),
            font_families = &["033-SSFangTangTi"],
            stroke_paint = new_stroke_paint(Color::WHITE, font_size as f32 * 0.24)
        ),
    );
    let text_w = text2image.longest_line().ceil() as i32;
    let text_h = text2image.height().ceil() as i32;

    let angle = options
        .rotate
        .unwrap_or_else(|| rand::thread_rng().gen_range(-40..40)) as f32;
    let x_offset = options.x_offset.unwrap();
    let y_offset = options.y_offset.unwrap();

    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    let center_x = frame.width() / 2 + x_offset;
    let center_y = frame.height() / 5 + y_offset;

    canvas.rotate(angle, Some(Point::new(center_x as f32, center_y as f32)));
    text2image.draw_on_canvas(&canvas, (center_x - text_w / 2, center_y - text_h / 2));
    canvas.reset_matrix();

    encode_png(surface.image_snapshot())
}

register_meme!(
    "pjsk",
    pjsk,
    min_texts = 1,
    max_texts = 1,
    keywords = &["pjsk", "世界计划"],
    shortcuts = CHARACTERS
        .iter()
        .map(|c| {
            shortcut!(
                format!("(:?pjsk|世界计划)[_-]?(:?{}|{})", c.name_en, c.name_cn).as_str(),
                options = &[("character", c.name_en),],
                humanized = format!("pjsk{}", c.name_cn).as_str(),
            )
        })
        .collect::<Vec<_>>()
        .as_slice(),
    tags = MemeTags::project_sekai(),
    date_created = local_date(2024, 12, 19),
    date_modified = local_date(2024, 12, 19),
);
