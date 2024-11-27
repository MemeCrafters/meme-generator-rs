// use skia_safe::{
//     surfaces,
//     textlayout::{
//         Decoration, FontCollection, ParagraphBuilder, ParagraphStyle, TextDecoration, TextStyle,
//         TypefaceFontProvider,
//     },
//     Color, EncodedImageFormat, FontMgr, FontStyle, Paint, Point,
// };
// use std::{fs::File, io::Write};

// fn test() {
//     let mut font_collection = FontCollection::new();
//     font_collection.set_default_font_manager(FontMgr::new(), None);

//     let para_style = ParagraphStyle::new();
//     let mut builder = ParagraphBuilder::new(&para_style, &font_collection);

//     let mut paint = Paint::default();
//     paint.set_anti_alias(true);
//     paint.set_color(Color::BLACK);

//     let mut style = TextStyle::new();
//     style.set_font_size(30.0);
//     style.set_foreground_paint(&paint);
//     style.set_font_families(&["times", "georgia", "serif"]);
//     builder.push_style(&style);

//     let mut style_bold = style.clone();
//     style_bold.set_font_style(FontStyle::bold());
//     builder.push_style(&style_bold);
//     builder.add_text("Typography");
//     builder.pop();

//     builder.add_text(" is the ");

//     let mut style_italic = style.clone();
//     style_italic.set_font_style(FontStyle::italic());
//     builder.push_style(&style_italic);
//     builder.add_text("art and technique");
//     builder.pop();

//     builder.add_text(" of arranging type to make written language ");

//     let mut style_underline = style.clone();
//     let mut decoration = Decoration::default();
//     decoration.ty = TextDecoration::UNDERLINE;
//     decoration.color = Color::BLACK;
//     decoration.thickness_multiplier = 1.5;
//     style_underline.set_decoration(&decoration);
//     builder.push_style(&style_underline);
//     builder.add_text("legible, readable, and appealing");
//     builder.pop();

//     builder.add_text(" when displayed. The arrangement of type involves selecting typefaces, point sizes, line lengths, line-spacing (leading), and letter-spacing (tracking), and adjusting the space between pairs of letters (kerning). The term typography is also applied to the style, arrangement, and appearance of the letters, numbers, and symbols created by the process.");

//     builder.add_text(" Furthermore, العربية نص جميل. द क्विक ब्राउन फ़ॉक्स jumps over the lazy 🐕.");

//     let mut paragraph = builder.build();
//     paragraph.layout(1000.0);

//     let width = paragraph.longest_line();
//     let height = paragraph.height();
//     let mut surface =
//         surfaces::raster_n32_premul((width.ceil() as i32, height.ceil() as i32)).unwrap();

//     let canvas = surface.canvas();
//     canvas.clear(Color::WHITE);
//     paragraph.paint(&canvas, Point::default());

//     let image = surface.image_snapshot();
//     let mut context = surface.direct_context();
//     let data = image
//         .encode(context.as_mut(), EncodedImageFormat::PNG, None)
//         .unwrap();
//     let mut file = File::create("test.png").unwrap();
//     let bytes = data.as_bytes();
//     file.write_all(bytes).unwrap();
// }
