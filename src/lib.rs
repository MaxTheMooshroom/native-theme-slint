#![cfg_attr(feature = "nightly", feature(variant_count))]

pub mod slint_modules;

use anyhow::Result;
use slint::ComponentHandle as _;

pub use slint_modules::{Anchor, SystemTheme, SystemThemePreset};

const INIT_FAILURE_MESSAGE: &str = "Failed to initialize library anchor in crate 'native-theme-slint'";

thread_local! {
    static ANCHOR: Anchor = Anchor::new().expect(INIT_FAILURE_MESSAGE);
}

#[allow(unused)]
pub fn init_library() -> Result<()> {
    ANCHOR.with(|anchor| {
        let theme = anchor.global::<SystemTheme>();

        theme.on_load(|preset| {
            let theme: native_theme::SystemTheme = match preset.as_str() {
                "" | "none" => {
                    native_theme::SystemTheme::from_system()
                        .expect(INIT_FAILURE_MESSAGE)
                },
                preset @ _ => {
                    native_theme::SystemTheme::from_system_with_preset(preset)
                        .expect(INIT_FAILURE_MESSAGE)
                }
            };

            // theme.into()
            todo!()
        });

        Ok(())
    })
}
