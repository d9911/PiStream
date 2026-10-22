use eframe::egui::{self, Color32, FontId, Stroke, TextStyle};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

#[derive(Clone, Copy)]
pub struct Palette {
    pub canvas: Color32,
    pub surface: Color32,
    pub soft: Color32,
    pub border: Color32,
    pub ink: Color32,
    pub muted: Color32,
    pub accent: Color32,
    pub primary_button: Color32,
    pub accent_soft: Color32,
    pub error: Color32,
}

impl Theme {
    pub fn palette(self) -> Palette {
        let rgb = Color32::from_rgb;
        match self {
            Self::Light => Palette {
                canvas: Color32::WHITE,
                surface: Color32::WHITE,
                soft: rgb(247, 247, 247),
                border: rgb(221, 221, 221),
                ink: rgb(34, 34, 34),
                muted: rgb(106, 106, 106),
                accent: rgb(255, 56, 92),
                primary_button: rgb(224, 11, 65),
                accent_soft: rgb(255, 239, 242),
                error: rgb(193, 53, 21),
            },
            Self::Dark => Palette {
                canvas: rgb(22, 23, 26),
                surface: rgb(29, 30, 34),
                soft: rgb(38, 39, 44),
                border: rgb(65, 66, 73),
                ink: rgb(246, 246, 247),
                muted: rgb(174, 175, 183),
                accent: rgb(255, 95, 122),
                primary_button: rgb(224, 11, 65),
                accent_soft: rgb(65, 34, 43),
                error: rgb(255, 151, 137),
            },
        }
    }
    pub fn apply(self, ctx: &egui::Context) {
        ctx.set_theme(match self {
            Self::Light => egui::Theme::Light,
            Self::Dark => egui::Theme::Dark,
        });
        let p = self.palette();
        let mut v = match self {
            Self::Light => egui::Visuals::light(),
            Self::Dark => egui::Visuals::dark(),
        };
        v.panel_fill = p.canvas;
        v.window_fill = p.surface;
        v.extreme_bg_color = p.soft;
        v.text_edit_bg_color = Some(p.surface);
        v.faint_bg_color = p.soft;
        v.code_bg_color = p.soft;
        v.override_text_color = Some(p.ink);
        v.weak_text_color = Some(p.muted);
        v.selection.bg_fill = p.accent_soft;
        v.selection.stroke = Stroke::new(1.0, p.ink);
        v.hyperlink_color = p.accent;
        v.error_fg_color = p.error;
        v.window_corner_radius = 14.into();
        v.window_stroke = Stroke::new(1.0, p.border);
        for w in [
            &mut v.widgets.noninteractive,
            &mut v.widgets.inactive,
            &mut v.widgets.hovered,
            &mut v.widgets.active,
            &mut v.widgets.open,
        ] {
            w.corner_radius = 8.into();
            w.bg_fill = p.surface;
            w.weak_bg_fill = p.surface;
            w.bg_stroke = Stroke::new(1.0, p.border);
            w.fg_stroke = Stroke::new(1.0, p.ink);
            w.expansion = 0.0;
        }
        v.widgets.hovered.bg_fill = p.soft;
        v.widgets.hovered.weak_bg_fill = p.soft;
        v.widgets.hovered.bg_stroke = Stroke::new(1.0, p.ink);
        v.widgets.active.bg_fill = p.accent_soft;
        v.widgets.active.weak_bg_fill = p.accent_soft;
        v.widgets.active.bg_stroke = Stroke::new(1.5, p.ink);
        ctx.set_visuals(v);
        ctx.global_style_mut(|s| {
            s.wrap_mode = Some(egui::TextWrapMode::Wrap);
            s.spacing.item_spacing = egui::vec2(12.0, 8.0);
            s.spacing.button_padding = egui::vec2(14.0, 8.0);
            s.spacing.interact_size = egui::vec2(44.0, 38.0);
            s.text_styles
                .insert(TextStyle::Body, FontId::proportional(15.0));
            s.text_styles
                .insert(TextStyle::Button, FontId::proportional(14.0));
            s.text_styles
                .insert(TextStyle::Small, FontId::proportional(12.0));
            s.text_styles
                .insert(TextStyle::Heading, FontId::proportional(28.0));
            s.text_styles
                .insert(TextStyle::Monospace, FontId::monospace(16.0));
        });
        ctx.send_viewport_cmd(egui::ViewportCommand::SetTheme(match self {
            Self::Light => egui::SystemTheme::Light,
            Self::Dark => egui::SystemTheme::Dark,
        }));
    }
}

pub fn app_icon() -> egui::IconData {
    eframe::icon_data::from_png_bytes(include_bytes!("../../assets/icons/pistream-128.png"))
        .expect("The bundled PiStreamTerminal icon must decode")
}

pub fn grouped(value: usize) -> String {
    let s = value.to_string();
    let len = s.len();
    s.chars()
        .enumerate()
        .fold(String::new(), |mut out, (i, c)| {
            if i > 0 && (len - i).is_multiple_of(3) {
                out.push(' ');
            }
            out.push(c);
            out
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn icon_is_valid_and_has_transparent_outer_corners() {
        let icon = app_icon();
        assert_eq!((icon.width, icon.height), (128, 128));
        assert!(icon.rgba.chunks_exact(4).any(|pixel| pixel[3] == 255));
        for i in [0, 127, 128 * 127, 128 * 128 - 1] {
            assert_eq!(icon.rgba[i * 4 + 3], 0);
        }
    }
    #[test]
    fn selected_theme_survives_system_theme_events() {
        let ctx = egui::Context::default();
        for theme in [Theme::Light, Theme::Dark] {
            theme.apply(&ctx);
            assert_eq!(
                ctx.global_style().visuals.panel_fill,
                theme.palette().canvas
            );
            assert_eq!(
                ctx.theme(),
                if theme == Theme::Light {
                    egui::Theme::Light
                } else {
                    egui::Theme::Dark
                }
            );
        }
    }
    #[test]
    fn precision_labels_use_readable_groups() {
        assert_eq!(grouped(1000), "1 000");
        assert_eq!(grouped(1_000_000), "1 000 000");
        assert_eq!(grouped(50), "50");
    }
}
