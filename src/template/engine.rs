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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::models::{Base16, Base30, Palette};

    fn create_minimal_base16() -> Base16 {
        Base16 {
            base00: "000000".to_string(),
            base01: "111111".to_string(),
            base02: "222222".to_string(),
            base03: "333333".to_string(),
            base04: "444444".to_string(),
            base05: "555555".to_string(),
            base06: "666666".to_string(),
            base07: "777777".to_string(),
            base08: "888888".to_string(),
            base09: "999999".to_string(),
            base0a: "aaaaaa".to_string(),
            base0b: "bbbbbb".to_string(),
            base0c: "cccccc".to_string(),
            base0d: "dddddd".to_string(),
            base0e: "eeeeee".to_string(),
            base0f: "ffffff".to_string(),
        }
    }

    fn create_minimal_base30() -> Base30 {
        Base30 {
            white: "ffffff".to_string(),
            darker_black: "000000".to_string(),
            black: "111111".to_string(),
            black2: "1a1a1a".to_string(),
            one_bg: "222222".to_string(),
            one_bg2: "2a2a2a".to_string(),
            one_bg3: "333333".to_string(),
            grey: "888888".to_string(),
            grey_fg: "999999".to_string(),
            grey_fg2: "aaaaaa".to_string(),
            light_grey: "bbbbbb".to_string(),
            red: "ff0000".to_string(),
            baby_pink: "ffaaaa".to_string(),
            pink: "ff00ff".to_string(),
            line: "444444".to_string(),
            green: "00ff00".to_string(),
            vibrant_green: "00ff88".to_string(),
            nord_blue: "5e81ac".to_string(),
            blue: "0000ff".to_string(),
            yellow: "ffff00".to_string(),
            sun: "ffaa00".to_string(),
            purple: "aa00ff".to_string(),
            dark_purple: "550088".to_string(),
            teal: "00ffff".to_string(),
            orange: "ff8800".to_string(),
            cyan: "00aaff".to_string(),
            lightbg: "eeeeee".to_string(),
        }
    }

    fn create_test_palette_base16_only() -> Palette {
        Palette {
            name: "test-palette".to_string(),
            base_16: Some(create_minimal_base16()),
            base_30: None,
        }
    }

    fn create_test_palette_full() -> Palette {
        Palette {
            name: "full-palette".to_string(),
            base_16: Some(create_minimal_base16()),
            base_30: Some(create_minimal_base30()),
        }
    }

    fn create_empty_palette() -> Palette {
        Palette {
            name: "empty".to_string(),
            base_16: None,
            base_30: None,
        }
    }

    #[test]
    fn test_new_creates_engine_with_filters() {
        let engine = TemplateEngine::new();
        assert!(engine.tera.get_filter("hex_hash").is_ok());
        assert!(engine.tera.get_filter("rgb").is_ok());
    }

    #[test]
    fn test_default_trait_implementation() {
        let engine = TemplateEngine::default();
        assert!(engine.tera.get_filter("hex_hash").is_ok());
        assert!(engine.tera.get_filter("rgb").is_ok());
    }

    #[test]
    fn test_create_context_with_base16() {
        let engine = TemplateEngine::new();
        let palette = create_test_palette_base16_only();

        let context = engine
            .create_context(&palette)
            .expect("Context creation failed");

        let name = context.get("name").expect("Name not found");
        assert_eq!(name.as_str(), Some("test-palette"));

        assert_eq!(context.get("base00").unwrap().as_str(), Some("000000"));
        assert_eq!(context.get("base01").unwrap().as_str(), Some("111111"));
        assert_eq!(context.get("base0A").unwrap().as_str(), Some("aaaaaa"));
        assert_eq!(context.get("base0F").unwrap().as_str(), Some("ffffff"));
    }

    #[test]
    fn test_create_context_with_base30() {
        let engine = TemplateEngine::new();
        let palette = create_test_palette_full();

        let context = engine
            .create_context(&palette)
            .expect("Context creation failed");

        assert_eq!(context.get("white").unwrap().as_str(), Some("ffffff"));
        assert_eq!(context.get("red").unwrap().as_str(), Some("ff0000"));
        assert_eq!(context.get("nord_blue").unwrap().as_str(), Some("5e81ac"));
        assert_eq!(context.get("lightbg").unwrap().as_str(), Some("eeeeee"));
    }

    #[test]
    fn test_create_context_missing_base16_returns_error() {
        let engine = TemplateEngine::new();
        let palette = create_empty_palette();

        let result = engine.create_context(&palette);
        assert!(result.is_err());
    }

    #[test]
    fn test_render_simple_template() {
        let mut engine = TemplateEngine::new();
        let mut context = Context::new();
        context.insert("color", "ff0000");

        let template = "Color: {{ color }}";
        let result = engine
            .render("test", template, &context)
            .expect("Render failed");

        assert_eq!(result, "Color: ff0000");
    }

    #[test]
    fn test_render_with_filter() {
        let mut engine = TemplateEngine::new();
        let mut context = Context::new();
        context.insert("color", "ff0000");

        let template = "Color: {{ color | hex_hash }}";
        let result = engine
            .render("test_filter", template, &context)
            .expect("Render failed");

        assert_eq!(result, "Color: #ff0000");
    }

    #[test]
    fn test_render_palette_base16_only() {
        let mut engine = TemplateEngine::new();
        let palette = create_test_palette_base16_only();

        let template = "Name: {{ name }}\nBase00: {{ base00 }}";
        let result = engine
            .render_palette("test_palette", template, &palette)
            .expect("Render failed");

        assert!(result.contains("Name: test-palette"));
        assert!(result.contains("Base00: 000000"));
    }

    #[test]
    fn test_render_palette_with_base30() {
        let mut engine = TemplateEngine::new();
        let palette = create_test_palette_full();

        let template = "Red: {{ red }}\nBlue: {{ blue }}";
        let result = engine
            .render_palette("test_full", template, &palette)
            .expect("Render failed");

        assert!(result.contains("Red: ff0000"));
        assert!(result.contains("Blue: 0000ff"));
    }

    #[test]
    fn test_render_invalid_template_returns_error() {
        let mut engine = TemplateEngine::new();
        let context = Context::new();

        let template = "{{ unclosed_tag";
        let result = engine.render("invalid", template, &context);

        assert!(result.is_err());
    }

    #[test]
    fn test_render_missing_variable_returns_error() {
        let mut engine = TemplateEngine::new();
        let context = Context::new();

        let template = "{{ missing_var }}";
        let result = engine.render("missing", template, &context);

        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_renders_reuse_engine() {
        let mut engine = TemplateEngine::new();
        let palette1 = create_test_palette_base16_only();
        let palette2 = create_test_palette_full();

        let template = "{{ name }}";

        let result1 = engine
            .render_palette("first", template, &palette1)
            .expect("First render failed");
        let result2 = engine
            .render_palette("second", template, &palette2)
            .expect("Second render failed");

        assert_eq!(result1, "test-palette");
        assert_eq!(result2, "full-palette");
    }

    #[test]
    fn test_render_with_conditional() {
        let mut engine = TemplateEngine::new();
        let palette = create_test_palette_full();

        let template = r#"
{%- if white -%}
white exists: {{ white }}
{%- endif -%}
"#;

        let result = engine
            .render_palette("conditional", template, &palette)
            .expect("Render failed");

        assert!(result.contains("white exists: ffffff"));
    }

    #[test]
    fn test_context_does_not_clone_unnecessarily() {
        let engine = TemplateEngine::new();
        let palette = create_test_palette_full();

        let context = engine.create_context(&palette).expect("Context failed");

        let _ = context.get("base00");
        let _ = context.get("base00");
        let _ = context.get("red");
        let _ = context.get("red");
    }

    #[test]
    fn test_render_all_base16_colors() {
        let mut engine = TemplateEngine::new();
        let palette = create_test_palette_base16_only();

        let template = r#"
{{ base00 }}{{ base01 }}{{ base02 }}{{ base03 }}
{{ base04 }}{{ base05 }}{{ base06 }}{{ base07 }}
{{ base08 }}{{ base09 }}{{ base0A }}{{ base0B }}
{{ base0C }}{{ base0D }}{{ base0E }}{{ base0F }}
"#;

        let result = engine
            .render_palette("all_colors", template, &palette)
            .expect("Render failed");

        // Verify all colors are rendered
        assert!(result.contains("000000"));
        assert!(result.contains("ffffff"));
        assert!(result.contains("aaaaaa"));
    }

    #[test]
    fn test_render_empty_template() {
        let mut engine = TemplateEngine::new();
        let palette = create_test_palette_base16_only();

        let result = engine
            .render_palette("empty", "", &palette)
            .expect("Render failed");

        assert_eq!(result, "");
    }

    #[test]
    fn test_render_template_with_loops() {
        let mut engine = TemplateEngine::new();
        let mut context = Context::new();

        let colors = vec!["ff0000", "00ff00", "0000ff"];
        context.insert("colors", &colors);

        let template = r#"
{%- for color in colors -%}
{{ color }}
{% endfor -%}
"#;

        let result = engine
            .render("loop", template, &context)
            .expect("Render failed");

        assert!(result.contains("ff0000"));
        assert!(result.contains("00ff00"));
        assert!(result.contains("0000ff"));
    }

    #[test]
    fn test_engine_is_reusable_after_error() {
        let mut engine = TemplateEngine::new();
        let palette = create_test_palette_base16_only();

        // First render with invalid template
        let _ = engine.render_palette("bad", "{{ unclosed", &palette);

        let result = engine
            .render_palette("good", "{{ name }}", &palette)
            .expect("Render should work after error");

        assert_eq!(result, "test-palette");
    }
}
