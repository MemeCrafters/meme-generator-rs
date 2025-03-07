use std::{
    fs::{create_dir, read, write},
    path::{Path, PathBuf},
};

use clap::ArgMatches;

use meme_generator::{
    error::Error,
    tools::image_operations::{
        crop, flip_horizontal, flip_vertical, gif_change_duration, gif_merge, gif_reverse,
        gif_split, grayscale, invert, merge_horizontal, merge_vertical, resize, rotate,
    },
};

pub(crate) fn handle_image(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("flip_h", sub_matches)) => {
            handle_image_flip_h(sub_matches);
        }
        Some(("flip_v", sub_matches)) => {
            handle_image_flip_v(sub_matches);
        }
        Some(("rotate", sub_matches)) => {
            handle_image_rotate(sub_matches);
        }
        Some(("resize", sub_matches)) => {
            handle_image_resize(sub_matches);
        }
        Some(("crop", sub_matches)) => {
            handle_image_crop(sub_matches);
        }
        Some(("grayscale", sub_matches)) => {
            handle_image_grayscale(sub_matches);
        }
        Some(("invert", sub_matches)) => {
            handle_image_invert(sub_matches);
        }
        Some(("merge_h", sub_matches)) => {
            handle_image_merge_h(sub_matches);
        }
        Some(("merge_v", sub_matches)) => {
            handle_image_merge_v(sub_matches);
        }
        _ => {}
    }
}

pub(crate) fn handle_gif(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("split", sub_matches)) => {
            handle_gif_split(sub_matches);
        }
        Some(("merge", sub_matches)) => {
            handle_gif_merge(sub_matches);
        }
        Some(("reverse", sub_matches)) => {
            handle_gif_reverse(sub_matches);
        }
        Some(("duration", sub_matches)) => {
            handle_gif_duration(sub_matches);
        }
        _ => {}
    }
}

fn read_image(path: &PathBuf) -> Vec<u8> {
    read(path).expect(format!("文件读取失败：{path:?}").as_str())
}

fn parse_image(sub_matches: &ArgMatches) -> Vec<u8> {
    read_image(sub_matches.get_one::<PathBuf>("IMAGE").unwrap())
}

fn parse_images(sub_matches: &ArgMatches) -> Vec<Vec<u8>> {
    sub_matches
        .get_many::<PathBuf>("IMAGES")
        .into_iter()
        .flatten()
        .map(|path| read_image(&PathBuf::from(path)))
        .collect()
}

fn handle_result(result: Result<Vec<u8>, Error>) {
    match result {
        Err(Error::ImageDecodeError(err)) => {
            eprintln!("图片解码失败：{err}");
        }
        Err(Error::ImageEncodeError(err)) => {
            eprintln!("图片编码失败：{err}");
        }
        Ok(result) => {
            let kind = infer::get(&result).unwrap();
            let extension = kind.extension();
            let filename_string = format!("result.{extension}");
            let filename = filename_string.as_str();
            write(filename, result).expect("图片保存失败");
            println!("操作成功，生成的文件为 `{filename}`");
        }
        _ => {}
    };
}

fn handle_results(results: Result<Vec<Vec<u8>>, Error>) {
    match results {
        Err(Error::ImageDecodeError(err)) => {
            eprintln!("图片解码失败：{err}");
        }
        Err(Error::ImageEncodeError(err)) => {
            eprintln!("图片编码失败：{err}");
        }
        Ok(results) => {
            let output_dir = Path::new("result");
            if !output_dir.exists() {
                create_dir(output_dir).expect("创建输出目录失败");
            }
            for (i, result) in results.iter().enumerate() {
                let kind = infer::get(&result).unwrap();
                let extension = kind.extension();
                let filename_string = format!("result/{i}.{extension}");
                let filename = filename_string.as_str();
                write(filename, result).expect(&format!("图片 `{filename}` 保存失败"));
            }
            println!("操作成功，生成的文件保存在 `result` 目录中");
        }
        _ => {}
    };
}

fn handle_image_flip_h(sub_matches: &ArgMatches) {
    let data = parse_image(sub_matches);
    let result = flip_horizontal(data);
    handle_result(result)
}

fn handle_image_flip_v(sub_matches: &ArgMatches) {
    let data = parse_image(sub_matches);
    let result = flip_vertical(data);
    handle_result(result)
}

fn handle_image_rotate(sub_matches: &ArgMatches) {
    let data = parse_image(sub_matches);
    let angle = sub_matches.get_one::<f32>("degrees").cloned();
    let result = rotate(data, angle);
    handle_result(result)
}

fn handle_image_resize(sub_matches: &ArgMatches) {
    let data = parse_image(sub_matches);
    let width = sub_matches.get_one::<i32>("width").cloned();
    let height = sub_matches.get_one::<i32>("height").cloned();
    let result = resize(data, width, height);
    handle_result(result)
}

fn handle_image_crop(sub_matches: &ArgMatches) {
    let data = parse_image(sub_matches);
    let left = sub_matches.get_one::<i32>("left").cloned();
    let top = sub_matches.get_one::<i32>("top").cloned();
    let right = sub_matches.get_one::<i32>("right").cloned();
    let bottom = sub_matches.get_one::<i32>("bottom").cloned();
    let result = crop(data, left, top, right, bottom);
    handle_result(result)
}

fn handle_image_grayscale(sub_matches: &ArgMatches) {
    let data = parse_image(sub_matches);
    let result = grayscale(data);
    handle_result(result)
}

fn handle_image_invert(sub_matches: &ArgMatches) {
    let data = parse_image(sub_matches);
    let result = invert(data);
    handle_result(result)
}

fn handle_image_merge_h(sub_matches: &ArgMatches) {
    let images = parse_images(sub_matches);
    let result = merge_horizontal(images);
    handle_result(result)
}

fn handle_image_merge_v(sub_matches: &ArgMatches) {
    let images = parse_images(sub_matches);
    let result = merge_vertical(images);
    handle_result(result)
}

fn handle_gif_split(sub_matches: &ArgMatches) {
    let data = parse_image(sub_matches);
    let result = gif_split(data);
    handle_results(result)
}

fn handle_gif_merge(sub_matches: &ArgMatches) {
    let images = parse_images(sub_matches);
    let duration = sub_matches.get_one::<f32>("duration").cloned();
    let result = gif_merge(images, duration);
    handle_result(result)
}

fn handle_gif_reverse(sub_matches: &ArgMatches) {
    let data = parse_image(sub_matches);
    let result = gif_reverse(data);
    handle_result(result)
}

fn handle_gif_duration(sub_matches: &ArgMatches) {
    let data = parse_image(sub_matches);
    let duration = *sub_matches.get_one::<f32>("DURATION").unwrap();
    let result = gif_change_duration(data, duration);
    handle_result(result)
}
