extern crate tera;

use tera::{Context, Tera, Result};

lazy_static! {
    pub static ref TEMPLATES: Option<Tera> = Tera::new("templates/**/*").ok();
}

pub fn render_template(template_name: &str, context: &Context) -> Result<String> {
    if let Some(tera) = &*TEMPLATES {
        tera.render(template_name, context)
    } else {
        Err("Template engine was not initialized".into())
    }
}

pub fn create_context(report: impl serde::Serialize) -> Context {
    let mut context = Context::new();
    context.insert("report", &report);
    context
}
