#![allow(unused_imports)]

use slint::ToSharedString;

use super::macros::*;
use super::{nt, IntoColor as _, Resolve, ResolveFrom, ResolverContextBase};
use super::{
    AccessibilityPreferences, ColorMode, FontStyle, IconSet,
    ThemePreset, VersionMatch, WidgetBorderSpec,

    ResolvedTheme, SystemThemeInner,
};

native_theme_enums! {
    nt::ColorMode => ColorMode [ Light, Dark ];
    nt::IconSet => IconSet [
        SfSymbols, SegoeFluent, Freedesktop, Material, Lucide,
    ];
    nt::FontStyle => FontStyle [ Normal, Italic, Oblique ];
}

native_theme_optional_enums! {
    Option<nt::VersionMatch> => VersionMatch [ Exact, PresetOlder, PresetNewer ];
}

string_enum! {
    enum ThemePreset {
        CatppuccinLatte, CatppuccinFrappe, CatppuccinMacchiato, CatppuccinMocha,
        Nord, Dracula, Gruvbox, Solarized, TokyoNight, OneDark, KdeBreeze,
        Adwaita, Windows11, MacosMonterey, MacosTahoe, Material3, Ios13,
    }
}

impl From<nt::AccessibilityPreferences> for AccessibilityPreferences {
    fn from(other: nt::AccessibilityPreferences) -> Self {
        Self {
            text_scaling_factor: other.text_scaling_factor,
            reduce_motion: other.reduce_motion,
            high_contrast: other.high_contrast,
            reduce_transparency: other.reduce_transparency,
        }
    }
}

impl Resolve<ResolvedTheme> for nt::ResolvedTheme {
    #[cfg(not(feature = "nightly"))]
    type ResolverContext = ();

    fn resolve(self, ctx: &ResolverContextBase, (): &Self::ResolverContext) -> ResolvedTheme {
        let defaults = self.defaults.resolve(ctx, &());
        let window = self.window.resolve(ctx, &defaults);
        Self {
            defaults,
            text_scale: self.text_scale.into(),

            // ---- Per-widget resolved structs ----
            window,
        }
    }
}

impl Resolve<SystemThemeInner> for nt::SystemTheme {
    #[cfg(not(feature = "nightly"))]
    type ResolverContext = ();

    fn resolve(self, ctx: &ResolverContextBase, (): &Self::ResolverContext) -> SystemThemeInner {
        SystemThemeInner {
            name: self.name.to_shared_string(),
            mode: self.mode.into(),
            light: self.light.resolve(ctx),
            dark: self.dark.resolve(ctx),
            preset: self.preset.as_str().try_into()
                .unwrap_or(ThemePreset::None),
            version_match: self.version_match.into(),
            icon_set: self.icon_set.into(),
            icon_theme: self.icon_theme.as_deref()
                .map(ToSharedString::to_shared_string)
                .unwrap_or_default(),
            layout: self.layout.resolve(ctx),
            accessibility: self.accessibility.into(),
        }
    }
}
