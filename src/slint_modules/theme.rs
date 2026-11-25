#![allow(unused_imports)]

use super::{nt, IntoColor, Resolve, ResolveFrom, ResolverContextBase};
use super::{ResolvedDefaults, defaults::DefaultLayoutTheme};
use super::{LayoutTheme, WindowTheme};

impl Resolve<LayoutTheme> for nt::LayoutTheme {
    #[cfg(not(feature = "nightly"))]
    type ResolverContext = ();

    fn resolve(
        self,
        ctx: &ResolverContextBase,
        (): Self::ResolverContext,
    ) -> LayoutTheme {
        LayoutTheme {
            widget_gap: other.widget_gap.unwrap_or(ctx.default_layout.widget_gap),
            container_margin: other.container_margin.unwrap_or(ctx.default_layout.container_margin),
            window_margin: other.window_margin.unwrap_or(ctx.default_layout.window_margin),
            section_gap: other.section_gap.unwrap_or(ctx.default_layout.section_gap),
        }
    }
}

impl Resolve<WindowTheme> for nt::WindowTheme {
    #[cfg(not(feature = "nightly"))]
    type ResolverContext = ResolvedDefaults;

    fn resolve(
        self,
        ctx: &ResolverContextBase,
        defaults: &Self::ResolverContext,
    ) -> WindowTheme {
        WindowTheme {
            background_color: self.background_color
                .map(IntoColor::into_color)
                .unwrap_or(defaults.background_color),
            title_bar_background: self.title_bar_background
                .map(IntoColor::into_color)
                .unwrap_or(defaults.surface_color),
            inactive_title_bar_background: self.inactive_title_bar_background
                .map(IntoColor::into_color)
                .unwrap_or(defaults.surface_color),
            inactive_title_bar_text_color: self.inactive_title_bar_text_color
                .map(IntoColor::into_color)
                .unwrap_or(defaults.disabled_text_color),
            title_bar_font: self.title_bar_font
                .map(|font| font.resolve(ctx, &defaults.font))
                .unwrap_or(defaults.font),
        }
    }
}
