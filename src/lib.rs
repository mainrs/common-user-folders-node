#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{ContextlessResult, Env, JsObject, JsString, Result};

mod r#impl {
    use directories_next::UserDirs;
    use std::path::PathBuf;

    pub fn get_user_audio_folder() -> Option<PathBuf> {
        UserDirs::new().and_then(|dirs| dirs.audio_dir().map(Into::into))
    }

    pub fn get_user_desktop_folder() -> Option<PathBuf> {
        UserDirs::new().and_then(|dirs| dirs.desktop_dir().map(Into::into))
    }

    pub fn get_user_document_folder() -> Option<PathBuf> {
        UserDirs::new().and_then(|dirs| dirs.document_dir().map(Into::into))
    }

    pub fn get_user_download_folder() -> Option<PathBuf> {
        UserDirs::new().and_then(|dirs| dirs.download_dir().map(Into::into))
    }

    pub fn get_user_picture_folder() -> Option<PathBuf> {
        UserDirs::new().and_then(|dirs| dirs.picture_dir().map(Into::into))
    }
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("audioFolder", audio_folder)?;
    exports.create_named_method("documentFolder", document_folder)?;
    exports.create_named_method("desktopFolder", desktop_folder)?;
    exports.create_named_method("downloadFolder", download_folder)?;
    exports.create_named_method("pictureFolder", picture_folder)?;
    Ok(())
}

#[contextless_function]
fn audio_folder(ctx: Env) -> ContextlessResult<JsString> {
    let res = r#impl::get_user_audio_folder()
        .map(|p| p.display().to_string())
        .map(|s| ctx.create_string(&s));

    Ok(match res {
        Some(inner) => match inner {
            Ok(s) => Some(s),
            Err(_) => None,
        },
        None => None,
    })
}

#[contextless_function]
fn document_folder(ctx: Env) -> ContextlessResult<JsString> {
    let res = r#impl::get_user_document_folder()
        .map(|p| p.display().to_string())
        .map(|s| ctx.create_string(&s));

    Ok(match res {
        Some(inner) => match inner {
            Ok(s) => Some(s),
            Err(_) => None,
        },
        None => None,
    })
}

#[contextless_function]
fn desktop_folder(ctx: Env) -> ContextlessResult<JsString> {
    let res = r#impl::get_user_desktop_folder()
        .map(|p| p.display().to_string())
        .map(|s| ctx.create_string(&s));

    Ok(match res {
        Some(inner) => match inner {
            Ok(s) => Some(s),
            Err(_) => None,
        },
        None => None,
    })
}

#[contextless_function]
fn download_folder(ctx: Env) -> ContextlessResult<JsString> {
    let res = r#impl::get_user_download_folder()
        .map(|p| p.display().to_string())
        .map(|s| ctx.create_string(&s));

    Ok(match res {
        Some(inner) => match inner {
            Ok(s) => Some(s),
            Err(_) => None,
        },
        None => None,
    })
}

#[contextless_function]
fn picture_folder(ctx: Env) -> ContextlessResult<JsString> {
    let res = r#impl::get_user_picture_folder()
        .map(|p| p.display().to_string())
        .map(|s| ctx.create_string(&s));

    Ok(match res {
        Some(inner) => match inner {
            Ok(s) => Some(s),
            Err(_) => None,
        },
        None => None,
    })
}
