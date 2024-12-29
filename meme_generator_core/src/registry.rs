use crate::meme::Meme;

#[allow(improper_ctypes_definitions)]
pub struct MemePackDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn MemeRegistry),
}

pub trait MemeRegistry {
    fn register_meme(&mut self, key: &str, meme: Box<dyn Meme>);
}

pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");
pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[macro_export]
macro_rules! declare_meme_pack {
    ($register:expr) => {
        #[no_mangle]
        pub static MEME_PACK_DECLARATION: $crate::registry::MemePackDeclaration =
            $crate::registry::MemePackDeclaration {
                rustc_version: $crate::registry::RUSTC_VERSION,
                core_version: $crate::registry::CORE_VERSION,
                register: $register,
            };
    };
}
