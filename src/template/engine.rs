use anyhow::Result;
use tera::{Context, Tera};

use super::filters;
use crate::palette::models::Palette;

pub struct TemplateEngine {
    tera: Tera,
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateEngine {
    pub fn new() -> Self {
        let mut tera = Tera::default();

        tera.register_filter("hex_hash", filters::hex_hash);
        tera.register_filter("rgb", filters::rgb);

        Self { tera }
    }

    pub fn create_context(&self, palette: &Palette) -> Result<Context> {
        let mut context = Context::new();

        context.insert("name", &palette.name);

        let base16 = palette.base16()?;

        macro_rules! insert_colors {
            ($($field:ident),+ $(,)?) => {
                $(context.insert(stringify!($field), &base16.$field);)+
            };
        }

        insert_colors!(
            base00, base01, base02, base03, base04, base05, base06, base07, base08, base09,
        );

        context.insert("base0A", &base16.base0a);
        context.insert("base0B", &base16.base0b);
        context.insert("base0C", &base16.base0c);
        context.insert("base0D", &base16.base0d);
        context.insert("base0E", &base16.base0e);
        context.insert("base0F", &base16.base0f);

        if let Ok(base30) = palette.base30() {
            macro_rules! insert_base30 {
                ($($field:ident),+ $(,)?) => {
                    $(context.insert(stringify!($field), &base30.$field);)+
                };
            }

            insert_base30!(
                white,
                darker_black,
                black,
                black2,
                one_bg,
                one_bg2,
                one_bg3,
                grey,
                grey_fg,
                grey_fg2,
                light_grey,
                red,
                baby_pink,
                pink,
                line,
                green,
                vibrant_green,
                nord_blue,
                blue,
                yellow,
                sun,
                purple,
                dark_purple,
                teal,
                orange,
                cyan,
                lightbg,
            );
        }

        Ok(context)
    }

    pub fn render(
        &mut self,
        template_name: &str,
        template_content: &str,
        context: &Context,
    ) -> Result<String> {
        self.tera
            .add_raw_template(template_name, template_content)?;
        Ok(self.tera.render(template_name, context)?)
    }

    pub fn render_palette(
        &mut self,
        template_name: &str,
        template_content: &str,
        palette: &Palette,
    ) -> Result<String> {
        let context = self.create_context(palette)?;
        self.render(template_name, template_content, &context)
    }
}
