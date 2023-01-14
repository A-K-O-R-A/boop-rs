#[cfg(feature = "plugin_base64")]
pub mod base64;

#[cfg(feature = "plugin_jwt")]
pub mod jwt;

#[cfg(feature = "plugin_md5")]
pub mod md5;

#[cfg(feature = "plugin_json")]
pub mod json;

pub mod text;

#[cfg(feature = "plugin_url")]
pub mod url;
