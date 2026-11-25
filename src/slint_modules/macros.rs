#![cfg_attr(debug_assertions, allow(unused))]

macro_rules! stub {
    () => { assert!(false, "called macro stub") };
    ($name:ident) => { assert!(false, concat!("called macro stub `", stringify!($name), "()`")) };
}

#[cfg(feature = "nightly-checks")]
macro_rules! variant_count { ($enum:ty) => { ::core::mem::variant_count::<$enum>() }; }

#[allow(unused, reason = "stub for 'signature' to be available when not in nightly")]
#[cfg(not(feature = "nightly-checks"))]
macro_rules! variant_count { ($enum:ty) => { stub!(variant_count) }; }

#[cfg(not(feature = "nightly-checks"))]
macro_rules! count { ($($items:expr;)*) => { [ $({ let _ = &$items; }),* ].len() }; }

#[cfg(feature = "nightly-checks")]
macro_rules! count { ($($items:expr;)*) => { ${count($items)} }; }

macro_rules! native_theme_optional_enums {
    ($(Option<$from:ty> => $to:ty [ $($variant:ident),* $(,)? ];)*) => {
        $(
            #[cfg(feature = "nightly-checks")]
            const _: () = {
                let _: [$from; variant_count!($from)] = [ $(<$from>::$variant),* ];
                let _: [$to; variant_count!($to)] = [ <$to>::None, $(<$to>::$variant),* ];
            };

            impl From<$to> for Option<$from> {
                fn from(other: $to) -> Self {
                    match other {
                        <$to>::None => None,
                        $(<$to>::$variant => Some(<$from>::$variant),)*
                        #[allow(
                            unreachable_patterns,
                            reason = "future-proof against enums becoming non-exhaustive"
                        )]
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
            #[cfg(feature = "nightly-checks")]
            const _: () = {
                let _from: [$from; variant_count!($from)] = [ $(<$from>::$variant),* ];
                let _to: [$to; variant_count!($to)] = [ $(<$to>::$variant),* ];
            };

            impl From<$from> for $to {
                fn from(other: $from) -> Self {
                    match other {
                        $(
                            <$from>::$variant => Self::$variant
                        ),+,
                        #[allow(
                            unreachable_patterns,
                            reason = "future-proof against enums becoming non-exhaustive"
                        )]
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
                #[cfg(feature = "nightly-checks")]
                const _: () = {
                    let _: [$enum; std::mem::variant_count::<$enum>()] = [ $enum::None, $(<$enum>::$variant),* ];
                };

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

// macros do not have a semantic binding by default, re-exporting them
// like this is necessary to refer to them by path rather than `#[macro_use]`
// on a module item.
pub(super) use { native_theme_optional_enums, native_theme_enums, string_enum };
