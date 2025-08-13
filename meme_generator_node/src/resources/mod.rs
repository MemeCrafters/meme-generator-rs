use meme_generator::resources;
use napi_derive::napi;

#[napi(js_name = "check_resources")]
/// 检查资源
pub fn check_resources() {
    resources::check_resources_sync(None);
}

#[napi(js_name = "check_resources_in_background")]
/// 在后台检查资源
pub fn check_resources_in_background() {
    resources::check_resources_in_background(None);
}
