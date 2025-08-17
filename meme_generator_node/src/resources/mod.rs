use napi_derive::napi;

use meme_generator::resources;

#[napi]
pub fn check_resources() {
    resources::check_resources_sync(None);
}

#[napi]
pub fn check_resources_in_background() {
    resources::check_resources_in_background(None);
}
