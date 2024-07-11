use std::path::PathBuf;

use clap::Parser;
use stampsmith::core::{render::render_template, uri::TemplateURI};

#[derive(clap::Parser)]
struct StampCLI {
    template: String,
    target: Option<String>,
}

fn main() {
    let args = StampCLI::parse();
    let template_uri = args.template;
    let template_path = TemplateURI::from(template_uri).fetch().unwrap();
    render_template(
        &template_path,
        &PathBuf::from(args.target.unwrap_or(".".to_string())),
    )
    .unwrap();
}
