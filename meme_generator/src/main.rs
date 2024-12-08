use meme_generator::{manager::get_meme, meme::InputImage};
use std::fs::{read, write};

fn main() {
    let petpet = get_meme("petpet").unwrap();
    println!("Loaded meme: {}", petpet.key());
    println!("Meme info: {:?}", petpet.info());
    println!(
        "Meme info json: {}",
        serde_json::to_string(&petpet.info()).unwrap()
    );
    let image = InputImage {
        name: "avatar".to_string(),
        data: read("../avatar.jpg").unwrap(),
    };
    let options = r#"
    {
        "circle": true
    }"#
    .to_string();
    let result = petpet.generate(&vec![image], &vec![], options).unwrap();
    let output_path = "output.gif";
    write(output_path, result).unwrap();
}
