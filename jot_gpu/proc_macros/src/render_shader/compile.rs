use std::cmp::Ordering;

use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use quote::quote;
use syn::Error;

pub fn compile_shader(src: TokenStream) -> Result<naga::Module, TokenStream> {
    let (src_str, src_spans) = convert_src(src.clone());

    let module = invoke_naga(&src_str).map_err(|errors| convert_errors(errors, &src_spans))?;

    Ok(module)
}

#[derive(Debug)]
struct SpanInString {
    start: usize,
    end: usize,
    token_span: Span,
}

fn convert_src(src: TokenStream) -> (String, Vec<SpanInString>) {
    fn non_identifier_char(c: char) -> bool {
        matches!(
            c,
            '(' | ')'
                | '{'
                | '}'
                | '['
                | ']'
                | '<'
                | '>'
                | ','
                | '+'
                | '*'
                | '/'
                | '!'
                | '\\'
                | '"'
                | '\''
                | '|'
                | '='
                | '^'
                | '&'
                | ';'
                | ':'
                | '?'
                | '%'
                | '@'
                | '#'
                | '~'
                | '.'
                | '£'
                | '$'
                | '`'
        )
    }

    fn should_add_space_between(last: char, next: char) -> bool {
        if last == '-' && next == '>' {
            return false; // Might be a function return like `->`
        }
        if non_identifier_char(last) && next == '=' {
            return false; // Might be a comparison like `>=`, `!=` or `==`
        }
        if last == next && non_identifier_char(next) {
            return false; // Might be a double operator like `++`
        }
        if last == ':' || next == ':' {
            return false; // Might be an import path like `a::b`
        }
        true
    }

    fn push_token(
        token: &str,
        span: Span,
        output: &mut String,
        output_spans: &mut Vec<SpanInString>,
    ) {
        let next_start_char = match token.chars().next() {
            Some(s) => s,
            None => return,
        };

        let start = output.len();
        if output.ends_with(move |last_char| should_add_space_between(last_char, next_start_char)) {
            *output += " ";
        }

        *output += token;
        let end = output.len();

        output_spans.push(SpanInString {
            start,
            end,
            token_span: span,
        })
    }

    fn append_tokens(
        tokens: TokenStream,
        output: &mut String,
        output_spans: &mut Vec<SpanInString>,
    ) {
        for token in tokens {
            match token {
                TokenTree::Group(g) => {
                    let delims = match g.delimiter() {
                        Delimiter::Parenthesis => Some(("(", ")")),
                        Delimiter::Brace => Some(("{", "}")),
                        Delimiter::Bracket => Some(("[", "]")),
                        Delimiter::None => None,
                    };

                    if let Some((start, _)) = delims {
                        push_token(start, g.span_open(), output, output_spans);
                    }

                    append_tokens(g.stream(), output, output_spans);

                    if let Some((_, end)) = delims {
                        push_token(end, g.span_close(), output, output_spans);
                    }
                }
                _ => push_token(&token.to_string(), token.span(), output, output_spans),
            }
        }
    }

    let mut output = String::new();
    let mut output_spans = Vec::new();

    append_tokens(src, &mut output, &mut output_spans);

    (output, output_spans)
}

fn invoke_naga(src: &str) -> Result<naga::Module, Vec<(naga::Span, String)>> {
    let module = match naga::front::wgsl::parse_str(src) {
        Ok(module) => module,
        Err(e) => {
            let mut errors = Vec::new();

            let mut e_base: &dyn std::error::Error = &e;
            let mut message = format!("{}", e);
            while let Some(e) = e_base.source() {
                message = format!("{}: {}", message, e);
                e_base = e;
            }

            let labels = e.labels();
            if labels.len() == 0 {
                errors.push((naga::Span::new(0, u32::MAX), message.clone()));
            } else {
                for (loc, label) in labels {
                    errors.push((loc, format!("at {}: {}", label, message)));
                }
            }

            return Err(errors);
        }
    };

    let mut validator = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::all(),
    );

    let _ = match validator.validate(&module) {
        Ok(info) => info,
        Err(e) => {
            let mut errors = Vec::new();

            let mut e_base: &dyn std::error::Error = e.as_inner();
            let mut message = format!("{}", e);
            while let Some(e) = e_base.source() {
                message = format!("{}: {}", message, e);
                e_base = e;
            }

            if e.spans().len() == 0 {
                errors.push((naga::Span::new(0, 1), message));
            } else {
                for (loc, extra) in e.spans() {
                    errors.push((*loc, format!("{}: {}", message, extra)))
                }
            }

            return Err(errors);
        }
    };

    Ok(module)
}

fn convert_errors(errors: Vec<(naga::Span, String)>, src_spans: &[SpanInString]) -> TokenStream {
    fn get_spans_within(start: usize, end: usize, src_spans: &[SpanInString]) -> Vec<Span> {
        let span_start = src_spans.binary_search_by(move |span| {
            assert!(span.start <= span.end);

            if start >= span.start && start < span.end {
                Ordering::Equal
            } else if start < span.start {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        let span_start = match span_start {
            Ok(s) => s,
            Err(s) => s.saturating_sub(1),
        };

        let span_end = src_spans.binary_search_by(move |span| {
            assert!(span.start <= span.end);

            if end > span.start && end <= span.end {
                Ordering::Equal
            } else if end <= span.start {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        let span_end = match span_end {
            Ok(s) => usize::min(s + 1, src_spans.len()),
            Err(s) => s,
        };

        src_spans[span_start..span_end]
            .iter()
            .map(|span| span.token_span)
            .collect()
    }

    let mut output_errors = Vec::new();

    for (loc, msg) in errors {
        let error_spans = if let Some(loc) = loc.to_range() {
            get_spans_within(loc.start, loc.end, src_spans)
        } else {
            src_spans.iter().map(|s| s.token_span).collect()
        };

        for span in error_spans {
            output_errors.push(Error::new(span, msg.clone()).to_compile_error());
        }
    }

    quote! {
        #(#output_errors)*
    }
}
