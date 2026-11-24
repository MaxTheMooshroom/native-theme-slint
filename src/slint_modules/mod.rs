use slint::ToSharedString;

use crate::slint_modules::nt::IntoColor;

macro_rules! native_theme_optional_enums {
    ($(Option<$from:ty> => $to:ty [ $($variant:ident),* $(,)? ];)*) => {
        $(
            #[cfg(feature = "nightly")]
            const _: [$from; std::mem::variant_count::<$from>()] = [ $(<$from>::$variant),* ];

            #[cfg(feature = "nightly")]
            const _: [$to; std::mem::variant_count::<$to>()] = [ <$to>::None, $(<$to>::$variant),* ];

            impl From<$to> for Option<$from> {
                fn from(other: $to) -> Self {
                    match other {
                        <$to>::None => None,
                        $(<$to>::$variant => Some(<$from>::$variant),)*
                        #[allow(unreachable_patterns)]
                        _ => panic!(concat!(
                            "Unknown `",
                            stringify!($to),
                            "` variant converted to `",
                            stringify!($from),
                            "`"
                        ))
                    }
                }
            }

            impl From<Option<$from>> for $to {
                fn from(other: Option<$from>) -> Self {
                    let Some(inner) = other else {
                        return Self::None;
                    };

                    match inner {
                        $(
                            <$from>::$variant => Self::$variant
                        ),+,
                        _ => panic!(concat!(
                            "Unknown `",
                            stringify!(($from)),
                            "` variant converted to `",
                            stringify!($to),
                            "`"
                        ))
                    }
                }
            }
        )*
    };
}

macro_rules! native_theme_enums {
    ($($from:ty => $to:ty [ $($variant:ident),+ $(,)? ];)*) => {
        $(
            #[cfg(feature = "nightly")]
            const _: [$from; std::mem::variant_count::<$from>()] = [ $(<$from>::$variant),* ];

            #[cfg(feature = "nightly")]
            const _: [$to; std::mem::variant_count::<$to>()] = [ $(<$to>::$variant),* ];

            impl From<$from> for $to {
                fn from(other: $from) -> Self {
                    match other {
                        $(
                            <$from>::$variant => Self::$variant
                        ),+,
                        #[allow(unreachable_patterns)]
                        _ => panic!(concat!(
                            "Unknown `",
                            stringify!(($from)),
                            "` variant converted to `",
                            stringify!($to),
                            "`"
                        ))
                    }
                }
            }
        )*
    };
}

macro_rules! string_enum {
    ($(enum $enum:ty { $( $variant:ident ),* $(,)?})*) => {        
        ::pastey::paste! {
            $(
                #[cfg(feature = "nightly")]
                const _: [$enum; std::mem::variant_count::<$enum>()] = [ $enum::None, $(<$enum>::$variant),* ];

                impl From<&str> for $enum {
                    fn from(value: &str) -> Self {
                        match value {
                            $([<@ $variant:kebab>] => Self::$variant),*,
                            _ => Self::None
                        }
                    }
                }

                impl std::fmt::Display for $enum {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", match self {
                            $(Self::$variant => [<@ $variant:kebab>]),*,
                            Self::None => "none",
                        })
                    }
                }
            )*
        }
    };
}

struct ResolveContext {
    dpi: f32,
    font_whole_pixels: bool,
    focus_ring_keyboard_only: bool,
}

trait Resolve<T> {
    fn resolve(ctx: &ResolveContext, other: T) -> Self;
}

trait TryResolve<T>: Sized {
    type Error;

    fn try_resolve(ctx: &ResolveContext, other: T) -> Result<Self, Self::Error>;
}

impl<T, R: Resolve<T>> TryResolve<T> for R {
    type Error = std::convert::Infallible;

    fn try_resolve(ctx: &ResolveContext, other: T) -> Result<Self, Self::Error> {
        Ok(Self::resolve(ctx, other))
    }
}

slint::include_modules!();

native_theme_enums! {
    nt::ColorMode => ColorMode [ Light, Dark ];
    nt::IconSet => IconSet [
        SfSymbols, SegoeFluent, Freedesktop, Material, Lucide,
    ];
    nt::FontStyle => FontStyle [ Normal, Italic, Oblique ];
}

native_theme_optional_enums! {
    Option<native_theme::VersionMatch> => VersionMatch [ Exact, PresetOlder, PresetNewer ];
}

string_enum! {
    enum ThemePreset {
        CatppuccinLatte, CatppuccinFrappe, CatppuccinMacchiato, CatppuccinMocha,
        Nord, Dracula, Gruvbox, Solarized, TokyoNight, OneDark, KdeBreeze,
        Adwaita, Windows11, MacosMonterey, MacosTahoe, Material3, Ios13,
    }
}

mod nt {
    pub(super) use ::native_theme::*;
    pub(super) use ::native_theme::theme::*;

    pub(super) trait IntoColor {
        fn into_color(self) -> slint::Color;
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

impl TryFrom<nt::LayoutTheme> for LayoutTheme {
    type Error = ();

    fn try_from(other: nt::LayoutTheme) -> Result<Self, Self::Error> {
        Ok(Self {
            widget_gap: other.widget_gap.ok_or(())?,
            container_margin: other.container_margin.ok_or(())?,
            window_margin: other.window_margin.ok_or(())?,
            section_gap: other.section_gap.ok_or(())?,
        })
    }
}

impl nt::IntoColor for nt::color::Rgba {
    fn into_color(self) -> slint::Color {
        slint::Color::from_argb_u8(
            self.a,
            self.r,
            self.g,
            self.b
        )
    }
}



impl Resolve<nt::ResolvedFontSpec> for ResolvedFontSpec {
    fn resolve(ctx: &ResolveContext, other: nt::ResolvedFontSpec) -> Self {
        Self {
            family: other.family.to_shared_string(),
            size: other.size,
            defined_size: other.defined_size
                .unwrap_or(nt::FontSize::Px(0.))
                .to_logical_px(ctx.dpi),
            weight: other.weight as i32,
            style: other.style.into(),
            color: other.color.into_color(),
        }
    }
}

impl From<nt::ResolvedDefaultsBorder> for ResolvedDefaultsBorder {
    fn from(other: nt::ResolvedDefaultsBorder) -> Self {
        Self {
            color: other.color.into_color(),
            corner_radius: other.corner_radius,
            corner_radius_lg: other.corner_radius_lg,
            line_width: other.line_width,
            opacity: other.opacity,
            shadow_enabled: other.shadow_enabled,
        }
    }
}

impl From<nt::ResolvedIconSizes> for ResolvedIconSizes {
    fn from(other: nt::ResolvedIconSizes) -> Self {
        Self {
            toolbar: other.toolbar,
            small: other.small,
            large: other.large,
            dialog: other.dialog,
            panel: other.panel,
        }
    }
}

impl Resolve<nt::ResolvedDefaults> for ResolvedDefaults {
    fn resolve(ctx: &ResolveContext, other: nt::ResolvedDefaults) -> Self {
        Self {
            // ---- Base font ----
            font: ResolvedFontSpec::resolve(ctx, other.font),
            line_height: other.line_height,
            font_whole_pixels: other.font_whole_pixels.unwrap_or(ctx.font_whole_pixels),
            mono_font: ResolvedFontSpec::resolve(ctx, other.mono_font),

            // ---- Base colors ----
            background_color: other.background_color.into_color(),
            text_color: other.text_color.into_color(),
            accent_color: other.accent_color.into_color(),
            accent_text_color: other.accent_text_color.into_color(),
            surface_color: other.surface_color.into_color(),
            muted_color: other.muted_color.into_color(),
            shadow_color: other.shadow_color.into_color(),
            link_color: other.link_color.into_color(),
            selection_background: other.selection_background.into_color(),
            selection_text_color: other.selection_text_color.into_color(),
            selection_inactive_background: other.selection_inactive_background.into_color(),
            selection_inactive_text_color: other.selection_inactive_text_color.into_color(),
            text_selection_background: other.text_selection_background.into_color(),
            text_selection_color: other.text_selection_color.into_color(),
            disabled_text_color: other.disabled_text_color.into_color(),

            // ---- Status colors ----
            danger_color: other.danger_color.into_color(),
            danger_text_color: other.danger_text_color.into_color(),
            warning_color: other.warning_color.into_color(),
            warning_text_color: other.warning_text_color.into_color(),
            success_color: other.success_color.into_color(),
            success_text_color: other.success_text_color.into_color(),
            info_color: other.info_color.into_color(),
            info_text_color: other.info_text_color.into_color(),

            // ---- Global geometry ----
            border: other.border.into(),
            disabled_opacity: other.disabled_opacity,

            // ---- Focus ring ----
            focus_ring_color: other.focus_ring_color.into_color(),
            focus_ring_width: other.focus_ring_width,
            focus_ring_offset: other.focus_ring_offset,
            focus_ring_keyboard_only: other.focus_ring_keyboard_only
                .unwrap_or(ctx.focus_ring_keyboard_only),

            // ---- Icon sizes ----
            icon_sizes: other.icon_sizes.into(),
        }
    }
}

impl From<nt::ResolvedTextScaleEntry> for ResolvedTextScaleEntry {
    fn from(nt::ResolvedTextScaleEntry{ size, weight, line_height, .. }: nt::ResolvedTextScaleEntry) -> Self {
        let weight = weight as i32;
        Self { size, weight, line_height }
    }
}

impl From<nt::ResolvedTextScale> for ResolvedTextScale {
    fn from(other: nt::ResolvedTextScale) -> Self {
        Self {
            caption: other.caption.into(),
            section_heading: other.section_heading.into(),
            dialog_title: other.dialog_title.into(),
            display: other.display.into(),
        }
    }
}

impl Resolve<nt::ResolvedTheme> for ResolvedTheme {
    fn resolve(ctx: &ResolveContext, other: nt::ResolvedTheme) -> Self {
        Self {
            defaults: ResolvedDefaults::resolve(ctx, other.defaults),
            text_scale: other.text_scale.into(),

            // ---- Per-widget resolved structs ----
            
        }
    }
}

impl From<nt::SystemTheme> for SystemThemeInner {
    fn from(theme: nt::SystemTheme) -> Self {
        Self {
            name: theme.name.to_shared_string(),
            mode: theme.mode.into(),
            light: theme.light.into(),
            dark: theme.dark.into(),
            preset: theme.preset.as_str().try_into()
                .unwrap_or(ThemePreset::None),
            version_match: theme.version_match.into(),
            icon_set: theme.icon_set.into(),
            icon_theme: theme.icon_theme.as_deref()
                .map(ToSharedString::to_shared_string)
                .unwrap_or_default(),
            layout: theme.layout.try_into()
                .unwrap_or_default(),
            accessibility: theme.accessibility.into(),
        }
    }
}
