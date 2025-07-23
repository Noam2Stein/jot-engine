use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{ToTokens, quote};

pub struct AssetsMod {
    assets_dir: PathBuf,
    consts: HashMap<Ident, TokenStream>,
    mods: HashMap<Ident, AssetsMod>,
}

impl AssetsMod {
    pub fn new(assets_dir: PathBuf) -> Self {
        Self {
            assets_dir,
            consts: HashMap::new(),
            mods: HashMap::new(),
        }
    }

    pub fn push(&mut self, relative_path: &Path, type_path: &TokenStream) {
        assert!(
            !relative_path.is_absolute(),
            "path must be relative to assets_dir"
        );

        let Some(file_name) = relative_path.file_stem() else {
            panic!("asset path must have a file name");
        };

        let file_name_str = file_name.to_string_lossy().to_string();

        let ident = Ident::new(&file_name_str, Span::call_site());

        let mut components = relative_path.components().peekable();
        let mut current = self;

        // Walk all but the last component (the file itself)
        while let Some(comp) = components.peek().map(|comp| *comp) {
            if components.clone().count() == 1 {
                break;
            }

            let comp_str = comp.as_os_str().to_string_lossy().to_string();
            let mod_ident = Ident::new(&comp_str, Span::call_site());

            current = current
                .mods
                .entry(mod_ident.clone())
                .or_insert_with(|| AssetsMod {
                    assets_dir: current.assets_dir.clone(),
                    consts: HashMap::new(),
                    mods: HashMap::new(),
                });

            components.next();
        }

        // Full asset path from root
        let full_path = current.assets_dir.join(relative_path);
        let full_path_str = full_path.to_string_lossy().replace('\\', "/");

        let const_value = quote! {
            unsafe { ::jot::asset::AssetId::<#type_path>::new_unchecked(#full_path_str) }
        };

        current.consts.insert(ident, const_value);
    }
}

impl ToTokens for AssetsMod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (ident, submod) in &self.mods {
            let mut mod_tokens = TokenStream::new();
            submod.to_tokens(&mut mod_tokens);

            tokens.extend(quote! {
                pub mod #ident {
                    #mod_tokens
                }
            });
        }

        for (ident, value) in &self.consts {
            tokens.extend(quote! {
                pub const #ident: ::jot::asset::AssetId<_> = #value;
            });
        }
    }
}
