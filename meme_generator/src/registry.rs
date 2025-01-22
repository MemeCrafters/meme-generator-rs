use std::{collections::HashMap, fs::DirEntry, path::PathBuf, rc::Rc, sync::LazyLock};

use libloading::Library;

use meme_generator_core::{
    config::MEME_HOME,
    error::Error,
    meme::{Image, Meme, MemeInfo, OptionValue},
    registry::{MemePackDeclaration, CORE_VERSION, RUSTC_VERSION},
};
use tracing::{info, warn};

use crate::config::CONFIG;

const LIBRARIES_DIR: LazyLock<PathBuf> = LazyLock::new(|| MEME_HOME.join("libraries"));

struct MemeRegistry {
    memes: HashMap<String, Box<dyn Meme>>,
}

impl MemeRegistry {
    fn new() -> Self {
        Self {
            memes: HashMap::default(),
        }
    }
}

impl meme_generator_core::registry::MemeRegistry for MemeRegistry {
    fn register_meme(&mut self, key: &str, meme: Box<dyn Meme>) {
        if CONFIG.meme.meme_disabled_list.contains(&key.to_string()) {
            return;
        }
        self.memes.insert(key.to_string(), meme);
    }
}

struct ExternalMeme {
    meme: Box<dyn Meme>,
    _library: Rc<Library>,
}

impl Meme for ExternalMeme {
    fn key(&self) -> String {
        self.meme.key()
    }

    fn info(&self) -> MemeInfo {
        self.meme.info()
    }

    fn generate(
        &self,
        images: Vec<Image>,
        texts: Vec<String>,
        options: HashMap<String, OptionValue>,
    ) -> Result<Vec<u8>, Error> {
        self.meme.generate(images, texts, options)
    }

    fn generate_preview(&self, options: HashMap<String, OptionValue>) -> Result<Vec<u8>, Error> {
        self.meme.generate_preview(options)
    }
}

unsafe impl Send for ExternalMeme {}
unsafe impl Sync for ExternalMeme {}

struct ExternalMemeRegistry {
    library: Rc<Library>,
    memes: HashMap<String, ExternalMeme>,
}

impl ExternalMemeRegistry {
    fn new(library: Rc<Library>) -> Self {
        Self {
            library,
            memes: HashMap::default(),
        }
    }
}

impl meme_generator_core::registry::MemeRegistry for ExternalMemeRegistry {
    fn register_meme(&mut self, key: &str, meme: Box<dyn Meme>) {
        if CONFIG.meme.meme_disabled_list.contains(&key.to_string()) {
            return;
        }
        self.memes.insert(
            key.to_string(),
            ExternalMeme {
                meme,
                _library: Rc::clone(&self.library),
            },
        );
    }
}

unsafe fn load_library(
    library_path: &DirEntry,
) -> Result<Option<HashMap<String, ExternalMeme>>, libloading::Error> {
    let library = Rc::new(Library::new(library_path.path())?);

    let declaration = library
        .get::<*mut MemePackDeclaration>(b"MEME_PACK_DECLARATION")?
        .read();

    if declaration.rustc_version != RUSTC_VERSION {
        warn!(
            "Library {:?} is compiled with rustc {}, but meme_generator_core is compiled with {}, please recompile the library",
            library_path.file_name(),
            declaration.rustc_version,
            RUSTC_VERSION,
        );
        return Ok(None);
    }
    if declaration.core_version != CORE_VERSION {
        warn!(
            "Library {:?} is compiled with meme_generator_core {}, but current version is {}, please recompile the library",
            library_path.file_name(),
            declaration.core_version,
            CORE_VERSION,
        );
        return Ok(None);
    }

    let mut registry = ExternalMemeRegistry::new(library);
    (declaration.register)(&mut registry);

    Ok(Some(registry.memes))
}

fn load_external_memes(registry: &mut MemeRegistry) -> Result<(), std::io::Error> {
    if !LIBRARIES_DIR.exists() {
        return Ok(());
    }
    for entry in LIBRARIES_DIR.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        if !["dll", "so", "dylib"].contains(&ext) {
            continue;
        }
        match unsafe { load_library(&entry) } {
            Ok(Some(memes)) => {
                info!(
                    "Loaded library {:?} with {} memes",
                    entry.file_name(),
                    memes.len()
                );
                for (key, meme) in memes {
                    registry.memes.insert(key, Box::new(meme));
                }
            }
            Ok(None) => {}
            Err(err) => {
                warn!("Failed to load library {:?}: {}", entry.file_name(), err);
            }
        }
    }
    Ok(())
}

pub(crate) fn load_memes() -> HashMap<String, Box<dyn Meme>> {
    let mut registry = MemeRegistry::new();

    if CONFIG.meme.load_builtin_memes {
        meme_generator_memes::register_memes(&mut registry);
    }

    if CONFIG.meme.load_external_memes {
        if let Err(err) = load_external_memes(&mut registry) {
            warn!("Error while loading external memes: {}", err);
        }
    }

    registry.memes
}
