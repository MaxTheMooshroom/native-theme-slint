#![cfg_attr(feature = "nightly", feature(associated_type_defaults))]
#![cfg_attr(feature = "nightly-checks", feature(variant_count, macro_metavar_expr))]

pub mod slint_modules;

use anyhow::Result;
use slint::{ComponentHandle, Global};

pub use slint_modules::{Library, ResolverContextBase, SystemThemeInner};

const INIT_FAILURE_MESSAGE: &str = "Failed to initialize library anchor in crate 'native-theme-slint'";

#[allow(unused)]
pub fn init_library<'a, C: ComponentHandle>(ctx: &'static ResolverContextBase, component: &'a C) -> Result<()>
where
    Library<'a>: Global<'a, C>,
{
    component.global::<Library>().on_get_system_theme(|preset| {
        use slint_modules::ResolveFrom;

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

        SystemThemeInner::resolve_from(ctx, theme)
    });

    Ok(())
}
