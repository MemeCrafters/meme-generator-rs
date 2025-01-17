use meme_generator_core::{meme::Meme, registry::MemeRegistry};

pub(crate) struct MemeDeclaration {
    pub name: &'static str,
    pub builder: fn() -> Box<dyn Meme>,
}

inventory::collect!(MemeDeclaration);

#[macro_export]
macro_rules! register_meme {
    ($key:expr, $function:expr, $($field:ident = $value:expr),* $(,)?) => {
        inventory::submit! {
            $crate::registry::MemeDeclaration {
                name: $key,
                builder: || -> Box<dyn meme_generator_core::meme::Meme> {
                    Box::new(
                        meme_generator_utils::builder::MemeBuilder {
                            key: $key.to_string(),
                            function: $function,
                            $(
                                $field: meme_generator_utils::builder::meme_setters::$field($value),
                            )*
                            ..Default::default()
                        }
                    )
                }
            }
        }
    }
}

pub fn register_memes(registry: &mut dyn MemeRegistry) {
    for meme_declaration in inventory::iter::<MemeDeclaration> {
        registry.register_meme(meme_declaration.name, (meme_declaration.builder)());
    }
}
