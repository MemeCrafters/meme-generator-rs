use meme_generator::{meme::Image, registry::MEME_REGISTRY};
use std::fs::{read, write};

fn main() {
    let registry = MEME_REGISTRY.lock().unwrap();

    let petpet = registry.get("petpet").unwrap();
    println!("Loaded meme: {}", petpet.key());
    println!("Meme info: {:?}", petpet.info());
    let image = Image {
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
