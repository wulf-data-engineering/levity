use wasm_bindgen::prelude::*;
use pulldown_cmark::{Parser, html};

#[wasm_bindgen]
pub fn parse_markdown(markdown_input: &str) -> String {
    let parser = Parser::new(markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
