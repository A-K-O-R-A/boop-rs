use crate::plugin::Plugin;

#[cfg(feature = "plugin_base64")]
pub mod base64;

#[cfg(feature = "plugin_html")]
pub mod html;

#[cfg(feature = "plugin_jwt")]
pub mod jwt;

#[cfg(feature = "plugin_md5")]
pub mod md5;

#[cfg(feature = "plugin_json")]
pub mod json;

pub mod text;

#[cfg(feature = "plugin_url")]
pub mod url;

pub fn default_plugins() -> Vec<Box<dyn Plugin>> {
    #[allow(unused_mut)]
    let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();

    #[cfg(feature = "plugin_base64")]
    plugins.push(Box::new(base64::Base64DecodePlugin));
    #[cfg(feature = "plugin_base64")]
    plugins.push(Box::new(base64::Base64EncodePlugin));

    #[cfg(feature = "plugin_html")]
    plugins.push(Box::new(html::HTMLDecodePlugin));
    #[cfg(feature = "plugin_base64")]
    plugins.push(Box::new(html::HTMLEncodePlugin));

    #[cfg(feature = "plugin_json")]
    plugins.push(Box::new(json::JsonStringifyPlugin));
    #[cfg(feature = "plugin_json")]
    plugins.push(Box::new(json::JsonParsePlugin));
    #[cfg(feature = "plugin_json")]
    plugins.push(Box::new(json::JsonFormatPlugin));
    #[cfg(feature = "plugin_json")]
    plugins.push(Box::new(json::JsonMinifyPlugin));

    #[cfg(feature = "plugin_jwt")]
    plugins.push(Box::new(jwt::JwtDecodePlugin));
    #[cfg(feature = "plugin_jwt")]
    plugins.push(Box::new(jwt::JwtFormatPlugin));

    #[cfg(feature = "plugin_md5")]
    plugins.push(Box::new(md5::Md5HashPlugin));

    #[cfg(feature = "plugin_url")]
    plugins.push(Box::new(url::UrlDecodePlugin));
    #[cfg(feature = "plugin_url")]
    plugins.push(Box::new(url::UrlEncodePlugin));

    plugins.push(Box::new(text::TextLowercasePlugin));
    plugins.push(Box::new(text::TextUppercasePlugin));
    plugins.push(Box::new(text::TextReversePlugin));
    plugins.push(Box::new(text::TextRemoveNewlinesPlugin));
    plugins.push(Box::new(text::TextCountCharactersPlugin));
    plugins.push(Box::new(text::TextCountLinesPlugin));

    plugins
}
