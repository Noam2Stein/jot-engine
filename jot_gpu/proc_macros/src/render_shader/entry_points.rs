use naga::{EntryPoint, ShaderStage};
use proc_macro2::{Span, TokenStream};
use syn::Error;

pub fn find_entry_points(module: &naga::Module) -> Result<(&EntryPoint, &EntryPoint), TokenStream> {
    let mut vertex = None;
    let mut fragment = None;

    for entry_point in module.entry_points.iter() {
        match entry_point.stage {
            ShaderStage::Vertex => {
                if vertex.is_some() {
                    return Err(
                        Error::new(Span::call_site(), "multiple vertex entry points")
                            .to_compile_error(),
                    );
                } else {
                    vertex = Some(entry_point);
                }
            }
            ShaderStage::Fragment => {
                if fragment.is_some() {
                    return Err(
                        Error::new(Span::call_site(), "multiple fragment entry points")
                            .to_compile_error(),
                    );
                } else {
                    fragment = Some(entry_point);
                }
            }
            _ => {
                return Err(
                    Error::new(Span::call_site(), "unexpected entry point").to_compile_error()
                );
            }
        }
    }

    let vertex = match vertex {
        Some(v) => v,
        None => {
            return Err(
                Error::new(Span::call_site(), "missing vertex entry point").to_compile_error()
            );
        }
    };

    let fragment = match fragment {
        Some(f) => f,
        None => {
            return Err(
                Error::new(Span::call_site(), "missing fragment entry point").to_compile_error(),
            );
        }
    };

    Ok((vertex, fragment))
}
