mod render_shader;

#[proc_macro]
pub fn render_shader(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    render_shader::render_shader(input)
}
