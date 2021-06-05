use dialoguer::theme::{ColorfulTheme, Theme};

pub fn get_theme() -> impl Theme {
    ColorfulTheme {
        ..ColorfulTheme::default()
    }
}
