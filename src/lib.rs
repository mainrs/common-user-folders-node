#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

macro_rules! create_named_method_for_folder_names {
    ($($folder_name:ident),+) => {
        use paste::paste;

        paste! {
            #[module_exports]
            fn init(mut exports: napi::JsObject) -> napi::Result<()> {
                $(
                    exports.create_named_method(stringify!([<$folder_name Folder>]), [<$folder_name _folder>])?;
                )+

                Ok(())
            }

            mod r#impl {
                $(
                    pub fn [<get_user_ $folder_name _folder>]() -> Option<std::path::PathBuf> {
                        directories_next::UserDirs::new().and_then(|dirs| dirs.[<$folder_name _dir>]().map(Into::into))
                    }
                )+
            }

            $(
                #[contextless_function]
                fn [<$folder_name _folder>](ctx: napi::Env) -> napi::ContextlessResult<napi::JsString> {
                    let res = r#impl::[<get_user_ $folder_name _folder>]()
                        .map(|p| p.display().to_string())
                        .map(|s| ctx.create_string(&s))
                        .and_then(Result::ok);

                    Ok(res)
                }
            )+
        }
    };
}

create_named_method_for_folder_names!(audio, desktop, document, download, picture);
