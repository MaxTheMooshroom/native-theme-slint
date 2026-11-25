mod defaults;
mod font;
mod macros;
mod system;
mod theme;

mod nt {
    pub(super) use ::native_theme::*;
    pub(super) use ::native_theme::theme::*;
}

trait IntoColor {
    fn into_color(self) -> slint::Color;
}

impl IntoColor for nt::color::Rgba {
    fn into_color(self) -> slint::Color {
        slint::Color::from_argb_u8(
            self.a,
            self.r,
            self.g,
            self.b
        )
    }
}

pub(crate) type Coord = f32;

#[non_exhaustive]
pub struct ResolverContextBase {
    /// System-provided value required to resolve logical size (length) to physical
    /// size (physical-length): `length = physical-length * dpi / 72`
    dpi: f32,
    font_whole_pixels: bool,
    focus_ring_keyboard_only: bool,

    default_layout: defaults::DefaultLayoutTheme,
}

pub(crate) trait Resolve<T> {
    #[cfg(feature = "nightly")]
    type ResolverContext = ();

    #[cfg(not(feature = "nightly"))]
    type ResolverContext;

    fn resolve(self, ctx: &ResolverContextBase, extra_ctx: &Self::ResolverContext) -> T;
}

slint::include_modules!();
