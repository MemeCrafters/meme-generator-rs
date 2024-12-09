use meme_generator::{manager::get_meme, meme::RawImage};
use std::fs::{read, write};

fn main() {
    let meme = get_meme("little_angel").unwrap();
    println!("Loaded meme: {}", meme.key());
    println!(
        "Meme info json: {}",
        serde_json::to_string(&meme.info()).unwrap()
    );
    let image = RawImage {
        name: "avatar".to_string(),
        data: read("../avatar.jpg").unwrap(),
    };
    let options = r#"{}"#.to_string();
    let result = meme.generate(&vec![image], &vec![], options).unwrap();
    let output_path = "output.png";
    write(output_path, result).unwrap();
}
