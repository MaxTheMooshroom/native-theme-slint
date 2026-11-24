#![cfg_attr(feature = "nightly", feature(variant_count))]

pub mod slint_modules;

use anyhow::Result;
use slint::{ComponentHandle, Global};

pub use slint_modules::Library;

const INIT_FAILURE_MESSAGE: &str = "Failed to initialize library anchor in crate 'native-theme-slint'";



#[allow(unused)]
pub fn init_library<'a, C: ComponentHandle>(component: &'a C) -> Result<()>
where
    Library<'a>: Global<'a, C>,
{
    component.global::<Library>().on_load(|preset| {
        let theme: native_theme::SystemTheme = match preset {
            slint_modules::ThemePreset::None => {
                native_theme::SystemTheme::from_system()
                    .expect(INIT_FAILURE_MESSAGE)
            },
            preset @ _ => {
                native_theme::SystemTheme::from_system_with_preset(&preset.to_string())
                    .expect(INIT_FAILURE_MESSAGE)
            }
        };

        // theme.into()
        todo!()
    });

    Ok(())
}
