use slint::{SharedString, ToSharedString};

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

slint::include_modules!();

native_theme_enums! {
    native_theme::ColorMode => ColorMode [ Light, Dark ];
    native_theme::theme::IconSet => IconSet [
        SfSymbols, SegoeFluent, Freedesktop, Material, Lucide,
    ];
}

native_theme_optional_enums! {
    Option<native_theme::VersionMatch> => VersionMatch [ Exact, PresetOlder, PresetNewer ];
}

impl From<native_theme::AccessibilityPreferences> for AccessibilityPreferences {
    fn from(other: native_theme::AccessibilityPreferences) -> Self {
        Self {
            text_scaling_factor: other.text_scaling_factor,
            reduce_motion: other.reduce_motion,
            high_contrast: other.high_contrast,
            reduce_transparency: other.reduce_transparency,
        }
    }
}

impl From<native_theme::SystemTheme> for SystemThemeInner {
    fn from(theme: native_theme::SystemTheme) -> Self {
        Self {
            name: theme.name.to_shared_string(),
            mode: theme.mode.into(),
            light: theme.light.into(),
            dark: theme.dark.into(),
            preset: theme.preset.to_shared_string(),
            version_match: theme.version_match.into(),
            icon_set: theme.icon_set.into(),
            icon_theme: theme.icon_theme.as_deref()
                .map(ToSharedString::to_shared_string)
                .unwrap_or_default(),
            layout: theme.layout.into(),
            accessibility: theme.accessibility.into()
        }
    }
}
