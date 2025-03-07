#[cfg(feature = "server")]
use std::net::IpAddr;
use std::{
    collections::HashMap,
    fs::{read, write},
    path::PathBuf,
};

use clap::{
    Arg, ArgAction, ArgMatches, Command, arg,
    builder::{PossibleValue, ValueParser},
    value_parser,
};

use meme_generator::{
    VERSION,
    error::Error,
    get_meme, get_meme_keys, get_memes,
    meme::{Image, MemeOption, OptionValue},
    resources::check_resources_sync,
    search_memes,
};
#[cfg(feature = "server")]
use meme_generator_server::run_server_sync;

use crate::tools::{handle_gif, handle_image};

fn build_arg(option: MemeOption) -> Arg {
    match option {
        MemeOption::Boolean {
            name,
            default,
            description,
            parser_flags,
        } => {
            let mut arg = Arg::new(&name);
            if let Some(default) = default {
                if default {
                    arg = arg.action(ArgAction::SetFalse);
                } else {
                    arg = arg.action(ArgAction::SetTrue);
                }
            }
            if let Some(description) = description {
                arg = arg.help(description);
            }
            if parser_flags.short {
                arg = arg.short(name.chars().next().unwrap());
            }
            if parser_flags.long {
                arg = arg.long(&name);
            }
            for alias in parser_flags.short_aliases {
                arg = arg.short_alias(alias);
            }
            for alias in parser_flags.long_aliases {
                arg = arg.alias(alias);
            }
            arg
        }
        MemeOption::String {
            name,
            default,
            choices,
            description,
            parser_flags,
        } => {
            let mut arg = Arg::new(&name).value_name(name.to_uppercase());
            if let Some(default) = default {
                arg = arg.default_value(default);
            }
            if let Some(choices) = choices {
                arg = arg.value_parser(
                    choices
                        .into_iter()
                        .map(|s| PossibleValue::new(s))
                        .collect::<Vec<PossibleValue>>(),
                );
            }
            if let Some(description) = description {
                arg = arg.help(description);
            }
            if parser_flags.short {
                arg = arg.short(name.chars().next().unwrap());
            }
            if parser_flags.long {
                arg = arg.long(&name);
            }
            for alias in parser_flags.short_aliases {
                arg = arg.short_alias(alias);
            }
            for alias in parser_flags.long_aliases {
                arg = arg.alias(alias);
            }
            arg
        }
        MemeOption::Integer {
            name,
            default,
            minimum,
            maximum,
            description,
            parser_flags,
        } => {
            let mut arg = Arg::new(&name).value_name(name.to_uppercase());
            if let Some(default) = default {
                arg = arg.default_value(default.to_string());
            }
            let mut parser = value_parser!(i32);
            if let Some(minimum) = minimum {
                if let Some(maximum) = maximum {
                    parser = parser.range(minimum as i64..=maximum as i64)
                } else {
                    parser = parser.range(minimum as i64..)
                }
            } else if let Some(maximum) = maximum {
                parser = parser.range(..=maximum as i64)
            }
            arg = arg.value_parser(parser);
            if let Some(description) = description {
                arg = arg.help(description);
            }
            if parser_flags.short {
                arg = arg.short(name.chars().next().unwrap());
            }
            if parser_flags.long {
                arg = arg.long(&name);
            }
            for alias in parser_flags.short_aliases {
                arg = arg.short_alias(alias);
            }
            for alias in parser_flags.long_aliases {
                arg = arg.alias(alias);
            }
            arg = arg.allow_hyphen_values(true);
            arg
        }
        MemeOption::Float {
            name,
            default,
            minimum,
            maximum,
            description,
            parser_flags,
        } => {
            let mut arg = Arg::new(&name).value_name(name.to_uppercase());
            if let Some(default) = default {
                arg = arg.default_value(default.to_string());
            }
            let parser = ValueParser::new(move |s: &str| {
                let value: f32 = s.parse().map_err(|_| {
                    clap::Error::raw(clap::error::ErrorKind::InvalidValue, "Not a valid float")
                })?;
                if let Some(minimum) = minimum {
                    if let Some(maximum) = maximum {
                        if !(minimum..=maximum).contains(&value) {
                            return Err(clap::Error::raw(
                                clap::error::ErrorKind::InvalidValue,
                                format!(
                                    "Value must be between {minimum} and {maximum} (inclusive)"
                                ),
                            ));
                        }
                    } else {
                        if !(minimum..).contains(&value) {
                            return Err(clap::Error::raw(
                                clap::error::ErrorKind::InvalidValue,
                                format!("Value must be greater than {minimum} (inclusive)"),
                            ));
                        }
                    }
                } else if let Some(maximum) = maximum {
                    if !(..=maximum).contains(&value) {
                        return Err(clap::Error::raw(
                            clap::error::ErrorKind::InvalidValue,
                            format!("Value must be less than {maximum} (inclusive)"),
                        ));
                    }
                }
                Ok(value)
            });
            arg = arg.value_parser(parser);
            if let Some(description) = description {
                arg = arg.help(description);
            }
            if parser_flags.short {
                arg = arg.short(name.chars().next().unwrap());
            }
            if parser_flags.long {
                arg = arg.long(&name);
            }
            for alias in parser_flags.short_aliases {
                arg = arg.short_alias(alias);
            }
            for alias in parser_flags.long_aliases {
                arg = arg.alias(alias);
            }
            arg = arg.allow_hyphen_values(true);
            arg
        }
    }
}

pub(crate) fn build_command() -> Command {
    let mut sub_commands: Vec<Command> = Vec::new();
    for meme in get_memes() {
        let key = meme.key();
        let info = meme.info();
        let options = info.params.options;
        let keywords = info.keywords.join("/");
        let mut command = Command::new(key)
            .about(keywords)
            .arg(
                arg!(--images [IMAGES] "图片路径")
                    .value_parser(value_parser!(PathBuf))
                    .num_args(1..),
            )
            .arg(arg!(--names [NAMES] "图片名").num_args(1..))
            .arg(arg!(--texts [TEXTS] "文字").num_args(1..))
            .arg_required_else_help(true);
        for option in options {
            let arg = build_arg(option);
            command = command.arg(arg);
        }
        sub_commands.push(command);
    }

    let mut command = Command::new("meme")
        .about("表情包制作")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .version(VERSION);
    command = command
        .subcommand(Command::new("list").about("查看所有可用表情").alias("ls"))
        .subcommand(
            Command::new("info")
                .about("查看表情详情")
                .alias("show")
                .arg(
                    arg!(<KEY> "表情名").value_parser(
                        get_meme_keys()
                            .into_iter()
                            .map(|s| PossibleValue::new(s))
                            .collect::<Vec<PossibleValue>>(),
                    ),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("search")
                .about("搜索表情")
                .alias("find")
                .arg(arg!(<KEYWORD> "关键词").value_parser(value_parser!(String)))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("preview")
                .about("生成表情预览")
                .arg(
                    arg!(<KEY> "表情名").value_parser(
                        get_meme_keys()
                            .into_iter()
                            .map(|s| PossibleValue::new(s))
                            .collect::<Vec<PossibleValue>>(),
                    ),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("generate")
                .alias("make")
                .about("制作表情")
                .subcommands(sub_commands)
                .subcommand_required(true),
        )
        .subcommand(
            Command::new("download").about("下载表情包所需的资源").arg(
                arg!(--url <URL> "资源链接")
                    .overrides_with("url")
                    .value_parser(value_parser!(String)),
            ),
        )
        .subcommand(
            Command::new("tools")
                .about("工具箱")
                .subcommand(
                    Command::new("image")
                        .about("图片操作")
                        .subcommands(vec![
                            Command::new("inspect")
                                .about("查看图片信息")
                                .arg(
                                    arg!(<IMAGE> "图片路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .required(true),
                                )
                                .arg_required_else_help(true),
                            Command::new("flip_h")
                                .about("水平翻转")
                                .arg(
                                    arg!(<IMAGE> "图片路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .required(true),
                                )
                                .arg_required_else_help(true),
                            Command::new("flip_v")
                                .about("竖直翻转")
                                .arg(
                                    arg!(<IMAGE> "图片路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .required(true),
                                )
                                .arg_required_else_help(true),
                            Command::new("rotate")
                                .about("旋转")
                                .arg(
                                    arg!(<IMAGE> "图片路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .required(true),
                                )
                                .arg(
                                    arg!(-d --degrees <DEGREES> "角度")
                                        .overrides_with("degrees")
                                        .value_parser(value_parser!(f32)),
                                )
                                .arg_required_else_help(true),
                            Command::new("resize")
                                .about("调整大小")
                                .arg(
                                    arg!(<IMAGE> "图片路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .required(true),
                                )
                                .arg(
                                    arg!(-w --width <WIDTH> "宽度")
                                        .overrides_with("width")
                                        .value_parser(value_parser!(i32)),
                                )
                                .arg(
                                    arg!(-h --height <HEIGHT> "高度")
                                        .overrides_with("height")
                                        .value_parser(value_parser!(i32)),
                                )
                                .arg_required_else_help(true),
                            Command::new("crop")
                                .about("裁剪")
                                .arg(
                                    arg!(<IMAGE> "图片路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .required(true),
                                )
                                .arg(
                                    arg!(-l --left <LEFT> "左边界")
                                        .overrides_with("left")
                                        .value_parser(value_parser!(i32)),
                                )
                                .arg(
                                    arg!(-t --top <TOP> "上边界")
                                        .overrides_with("top")
                                        .value_parser(value_parser!(i32)),
                                )
                                .arg(
                                    arg!(-r --right <RIGHT> "右边界")
                                        .overrides_with("right")
                                        .value_parser(value_parser!(i32)),
                                )
                                .arg(
                                    arg!(-b --bottom <BOTTOM> "下边界")
                                        .overrides_with("bottom")
                                        .value_parser(value_parser!(i32)),
                                )
                                .arg_required_else_help(true),
                            Command::new("grayscale")
                                .about("灰度化")
                                .arg(
                                    arg!(<IMAGE> "图片路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .required(true),
                                )
                                .arg_required_else_help(true),
                            Command::new("invert")
                                .about("反色")
                                .arg(
                                    arg!(<IMAGE> "图片路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .required(true),
                                )
                                .arg_required_else_help(true),
                            Command::new("merge_h")
                                .about("水平拼接")
                                .arg(
                                    arg!(<IMAGES> "图片路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .num_args(2..),
                                )
                                .arg_required_else_help(true),
                            Command::new("merge_v")
                                .about("竖直拼接")
                                .arg(
                                    arg!(<IMAGES> "图片路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .num_args(2..),
                                )
                                .arg_required_else_help(true),
                        ])
                        .subcommand_required(true),
                )
                .subcommand(
                    Command::new("gif")
                        .about("gif操作")
                        .subcommands(vec![
                            Command::new("split")
                                .about("拆分")
                                .arg(
                                    arg!(<IMAGE> "gif路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .required(true),
                                )
                                .arg_required_else_help(true),
                            Command::new("merge")
                                .about("合并")
                                .arg(
                                    arg!(<IMAGES> "gif路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .num_args(2..),
                                )
                                .arg(
                                    arg!(-d --duration <DURATION> "帧间隔时间")
                                        .overrides_with("duration")
                                        .value_parser(value_parser!(f32)),
                                )
                                .arg_required_else_help(true),
                            Command::new("reverse").about("反转").arg(
                                arg!(<IMAGE> "gif路径")
                                    .value_parser(value_parser!(PathBuf))
                                    .required(true),
                            ),
                            Command::new("duration")
                                .about("调整帧间隔时间")
                                .arg(
                                    arg!(<DURATION> "帧间隔时间")
                                        .value_parser(value_parser!(f32))
                                        .required(true),
                                )
                                .arg(
                                    arg!(<IMAGE> "gif路径")
                                        .value_parser(value_parser!(PathBuf))
                                        .required(true),
                                )
                                .arg_required_else_help(true),
                        ])
                        .subcommand_required(true),
                )
                .subcommand_required(true),
        );
    #[cfg(feature = "server")]
    {
        command = command.subcommand(
            Command::new("run")
                .about("启动 web server")
                .alias("start")
                .arg(
                    arg!(--host <HOST> "监听地址")
                        .overrides_with("host")
                        .value_parser(value_parser!(IpAddr)),
                )
                .arg(
                    arg!(--port <PORT> "端口号")
                        .overrides_with("port")
                        .value_parser(value_parser!(u16)),
                ),
        );
    }
    command
}

pub(crate) fn handle_list() {
    let list = get_memes()
        .into_iter()
        .enumerate()
        .map(|(i, meme)| {
            let index = i + 1;
            let key = meme.key();
            let info = meme.info();
            let keywords = info.keywords.join("/");
            format!("{index}. {key} ({keywords})")
        })
        .collect::<Vec<_>>()
        .join("\n");
    println!("表情列表：\n{list}");
}

pub(crate) fn handle_info(sub_matches: &ArgMatches) {
    let key = sub_matches.get_one::<String>("KEY").unwrap();
    let meme = get_meme(key).expect(format!("表情 `{key}` 不存在").as_str());
    let info = meme.info();
    let options = info.params.options;
    let options = options
        .iter()
        .map(|option| match option {
            MemeOption::Boolean {
                name,
                default,
                description,
                ..
            } => {
                let default = default.map(|b| b.to_string()).unwrap_or("无".to_string());
                let description = description.as_deref().unwrap_or("");
                format!(" * {name}：{description} (默认值：{default})")
            }
            MemeOption::String {
                name,
                default,
                choices,
                description,
                ..
            } => {
                let default = default.as_deref().unwrap_or("无");
                let choices = choices
                    .as_deref()
                    .map(|choices| choices.join("、"))
                    .unwrap_or("无".to_string());
                let description = description.as_deref().unwrap_or("");
                format!(" * {name}：{description} （默认值：{default}）（可选项：{choices}）")
            }
            MemeOption::Integer {
                name,
                default,
                minimum,
                maximum,
                description,
                ..
            } => {
                let default = default.map(|i| i.to_string()).unwrap_or("无".to_string());
                let range = match (minimum, maximum) {
                    (Some(min), Some(max)) => format!("{min}~{max}"),
                    (Some(min), None) => format!("{min}~"),
                    (None, Some(max)) => format!("~{max}"),
                    _ => "无".to_string(),
                };
                let description = description.as_deref().unwrap_or("");
                format!(" * {name}：{description} （默认值：{default}）（范围：{range}）")
            }
            MemeOption::Float {
                name,
                default,
                minimum,
                maximum,
                description,
                ..
            } => {
                let default = default
                    .map(|f| format!("{f:.1}"))
                    .unwrap_or("无".to_string());
                let range = match (minimum, maximum) {
                    (Some(min), Some(max)) => format!("{min:.1}~{max:.1}"),
                    (Some(min), None) => format!("{min:.1}~"),
                    (None, Some(max)) => format!("~{max:.1}"),
                    _ => "无".to_string(),
                };
                let description = description.as_deref().unwrap_or("");
                format!(" * {name}：{description} （默认值：{default}）（范围：{range}）")
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let shortcuts = info
        .shortcuts
        .into_iter()
        .map(|shortcut| {
            let humanized = match &shortcut.humanized {
                Some(humanized) => humanized,
                None => &shortcut.pattern,
            };
            format!(" * {humanized} ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    let tags = info.tags.into_iter().collect::<Vec<_>>().join("、");
    let keywords = info.keywords.join("/");
    let image_num = {
        let min = info.params.min_images;
        let max = info.params.max_images;
        if min == max {
            min.to_string()
        } else {
            format!("{min}~{max}")
        }
    };
    let text_num = {
        let min = info.params.min_texts;
        let max = info.params.max_texts;
        if min == max {
            min.to_string()
        } else {
            format!("{min}~{max}")
        }
    };
    let default_texts = info.params.default_texts.join("、");

    let mut output = format!("表情名：{key}\n关键词：{keywords}\n");
    if !shortcuts.is_empty() {
        output += &format!("快捷指令：\n{shortcuts}\n");
    }
    if !tags.is_empty() {
        output += &format!("标签：{tags}\n");
    }
    output += &format!("需要图片数目：{image_num}\n需要文字数目：{text_num}\n");
    if !default_texts.is_empty() {
        output += &format!("默认文字：[{default_texts}]\n");
    }
    if !options.is_empty() {
        output += &format!("其他参数：\n{options}\n");
    }
    println!("{output}");
}

pub(crate) fn handle_search(sub_matches: &ArgMatches) {
    let keyword = sub_matches.get_one::<String>("KEYWORD").unwrap();
    let meme_keys = search_memes(keyword, true);
    if meme_keys.is_empty() {
        eprintln!("未找到相关表情");
    } else {
        let list = meme_keys
            .into_iter()
            .enumerate()
            .map(|(i, key)| {
                let index = i + 1;
                let meme = get_meme(&key).unwrap();
                let info = meme.info();
                let keywords = info.keywords.join("/");
                let tags = info.tags.into_iter().collect::<Vec<_>>().join("、");
                let result = format!("{index}. {key} ({keywords})");
                if !tags.is_empty() {
                    format!("{result} [标签：{tags}]")
                } else {
                    result
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        println!("相关表情：\n{list}");
    }
}

pub(crate) fn handle_preview(sub_matches: &ArgMatches) {
    let key = sub_matches.get_one::<String>("KEY").unwrap();
    let meme = get_meme(key).expect(format!("表情 `{key}` 不存在").as_str());
    let result = meme.generate_preview(HashMap::new());
    handle_result(result)
}

pub(crate) fn handle_generate(sub_matches: &ArgMatches) {
    let (key, sub_matches) = sub_matches.subcommand().unwrap();
    let meme = get_meme(key).expect(format!("表情 `{key}` 不存在").as_str());
    let mut images = sub_matches
        .get_many::<PathBuf>("images")
        .into_iter()
        .flatten()
        .map(|path| {
            let data = read(path).expect(format!("文件读取失败：{path:?}").as_str());
            let name = if let Some(file_stem) = path.file_stem() {
                file_stem.to_string_lossy().to_string()
            } else {
                "".to_string()
            };
            Image { name, data }
        })
        .collect::<Vec<_>>();
    let names = sub_matches
        .get_many::<String>("names")
        .into_iter()
        .flatten()
        .map(|text| text.to_string())
        .collect::<Vec<_>>();
    for (i, name) in names.into_iter().enumerate() {
        if i < images.len() {
            images[i].name = name;
        }
    }
    let texts = sub_matches
        .get_many::<String>("texts")
        .into_iter()
        .flatten()
        .map(|text| text.to_string())
        .collect::<Vec<_>>();
    let mut options = HashMap::new();
    for option in meme.info().params.options {
        match option {
            MemeOption::Boolean { name, .. } => {
                if let Ok(Some(value)) = sub_matches.try_get_one::<bool>(name.as_str()) {
                    options.insert(name, OptionValue::Boolean(*value));
                }
            }
            MemeOption::String { name, .. } => {
                if let Ok(Some(value)) = sub_matches.try_get_one::<String>(name.as_str()) {
                    options.insert(name, OptionValue::String(value.clone()));
                }
            }
            MemeOption::Integer { name, .. } => {
                if let Ok(Some(value)) = sub_matches.try_get_one::<i32>(name.as_str()) {
                    options.insert(name, OptionValue::Integer(*value));
                }
            }
            MemeOption::Float { name, .. } => {
                if let Ok(Some(value)) = sub_matches.try_get_one::<f32>(name.as_str()) {
                    options.insert(name, OptionValue::Float(*value));
                }
            }
        }
    }
    let result = meme.generate(images, texts, options);
    handle_result(result)
}

fn handle_result(result: Result<Vec<u8>, Error>) {
    match result {
        Err(Error::ImageDecodeError(err)) => {
            eprintln!("图片解码失败：{err}");
        }
        Err(Error::ImageEncodeError(err)) => {
            eprintln!("图片编码失败：{err}");
        }
        Err(Error::ImageAssetMissing(path)) => {
            eprintln!("图片资源缺失：{path}");
        }
        Err(Error::DeserializeError(err)) => {
            eprintln!("反序列化失败：{err}");
        }
        Err(Error::ImageNumberMismatch(min, max, actual)) => {
            let range = {
                if min == max {
                    min.to_string()
                } else {
                    format!("{min}~{max}")
                }
            };
            eprintln!("图片数量不符，应为 {range}，实际传入 {actual}");
        }
        Err(Error::TextNumberMismatch(min, max, actual)) => {
            let range = {
                if min == max {
                    min.to_string()
                } else {
                    format!("{min}~{max}")
                }
            };
            eprintln!("文本数量不符，应为 {range}，实际传入 {actual}");
        }
        Err(Error::TextOverLength(text)) => {
            eprintln!("文字过长：{text}");
        }
        Err(Error::MemeFeedback(feedback)) => {
            eprintln!("{feedback}");
        }
        Ok(result) => {
            let kind = infer::get(&result).unwrap();
            let extension = kind.extension();
            let filename_string = format!("result.{extension}");
            let filename = filename_string.as_str();
            write(filename, result).expect("图片保存失败");
            println!("表情制作成功！生成的表情文件为 `{filename}`");
        }
    };
}

pub(crate) fn handle_download(sub_matches: &ArgMatches) {
    let resource_url = sub_matches.get_one::<String>("url");
    check_resources_sync(resource_url.cloned());
}

pub(crate) fn handle_tools(sub_matches: &ArgMatches) {
    match sub_matches.subcommand() {
        Some(("image", sub_matches)) => {
            handle_image(sub_matches);
        }
        Some(("gif", sub_matches)) => {
            handle_gif(sub_matches);
        }
        _ => {}
    }
}

#[cfg(feature = "server")]
pub(crate) fn handle_run(sub_matches: &ArgMatches) {
    let host = sub_matches.get_one::<IpAddr>("host");
    let port = sub_matches.get_one::<u16>("port");
    run_server_sync(host.cloned(), port.cloned());
}
