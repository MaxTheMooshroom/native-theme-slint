#![allow(unused_imports)]

use slint::Color;

use super::{nt, Coord, IntoColor as _, Resolve, ResolverContextBase};
use super::{WidgetBorderSpec};
use super::{
    ResolvedDefaults, ResolvedDefaultsBorder, ResolvedIconSizes,
};

/// Layout spacing constants shared between light and dark variants.
///
/// Unlike other widget themes, LayoutTheme lives on [`crate::Theme`] (top-level)
/// rather than [`crate::ThemeMode`] because spacing is variant-independent.
pub struct DefaultLayoutTheme {
    /// Space between adjacent widgets in logical pixels.
    pub widget_gap: Coord,
    /// Padding inside containers in logical pixels.
    pub container_margin: Coord,
    /// Padding inside the main window in logical pixels.
    pub window_margin: Coord,
    /// Space between major content sections in logical pixels.
    pub section_gap: Coord,
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

impl Resolve<ResolvedDefaults> for nt::ResolvedDefaults {
    #[cfg(not(feature = "nightly"))]
    type ResolverContext = ();

    fn resolve(self, ctx: &ResolverContextBase, (): &Self::ResolverContext) -> ResolvedDefaults {
        ResolvedDefaults {
            // ---- Base font ----
            font: self.font.resolve(ctx, &()),
            line_height: self.line_height,
            font_whole_pixels: self.font_whole_pixels.unwrap_or(ctx.font_whole_pixels),
            mono_font: self.mono_font.resolve(ctx, &()),

            // ---- Base colors ----
            background_color: self.background_color.into_color(),
            text_color: self.text_color.into_color(),
            accent_color: self.accent_color.into_color(),
            accent_text_color: self.accent_text_color.into_color(),
            surface_color: self.surface_color.into_color(),
            muted_color: self.muted_color.into_color(),
            shadow_color: self.shadow_color.into_color(),
            link_color: self.link_color.into_color(),
            selection_background: self.selection_background.into_color(),
            selection_text_color: self.selection_text_color.into_color(),
            selection_inactive_background: self.selection_inactive_background.into_color(),
            selection_inactive_text_color: self.selection_inactive_text_color.into_color(),
            text_selection_background: self.text_selection_background.into_color(),
            text_selection_color: self.text_selection_color.into_color(),
            disabled_text_color: self.disabled_text_color.into_color(),

            // ---- Status colors ----
            danger_color: self.danger_color.into_color(),
            danger_text_color: self.danger_text_color.into_color(),
            warning_color: self.warning_color.into_color(),
            warning_text_color: self.warning_text_color.into_color(),
            success_color: self.success_color.into_color(),
            success_text_color: self.success_text_color.into_color(),
            info_color: self.info_color.into_color(),
            info_text_color: self.info_text_color.into_color(),

            // ---- Global geometry ----
            border: self.border.into(),
            disabled_opacity: self.disabled_opacity,

            // ---- Focus ring ----
            focus_ring_color: self.focus_ring_color.into_color(),
            focus_ring_width: self.focus_ring_width,
            focus_ring_offset: self.focus_ring_offset,
            focus_ring_keyboard_only: self.focus_ring_keyboard_only
                .unwrap_or(ctx.focus_ring_keyboard_only),

            // ---- Icon sizes ----
            icon_sizes: self.icon_sizes.into(),
        }
    }
}
