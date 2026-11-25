#![allow(unused_imports)]

use slint::ToSharedString;

use super::{nt, IntoColor, Resolve, ResolverContextBase};
use super::ResolvedDefaults;
use super::{ResolvedFontSpec, ResolvedTextScale, ResolvedTextScaleEntry};


impl Resolve<ResolvedFontSpec> for nt::FontSpec {
    type ResolverContext = ResolvedFontSpec;

    fn resolve(
        self,
        ctx: &ResolverContextBase,
        default_font: &Self::ResolverContext,
    ) -> ResolvedFontSpec {
        ResolvedFontSpec {
            family: self.family
                .as_deref()
                .map(ToSharedString::to_shared_string)
                .unwrap_or(default_font.family.clone()),
            size: self.size
                .map(|size| size.to_logical_px(ctx.dpi))
                .unwrap_or(default_font.size),
            defined_size: default_font.defined_size,
            weight: self.weight
                .map(i32::from)
                .unwrap_or(default_font.weight),
            style: self.style
                .map(Into::into)
                .unwrap_or(default_font.style),
            color: self.color
                .map(IntoColor::into_color)
                .unwrap_or(default_font.color),
        }
    }
}

impl Resolve<ResolvedFontSpec> for nt::ResolvedFontSpec {
    #[cfg(not(feature = "nightly"))]
    type ResolverContext = ();

    fn resolve(
        self,
        ctx: &ResolverContextBase,
        (): &Self::ResolverContext,
    ) -> ResolvedFontSpec {
        ResolvedFontSpec {
            family: self.family.to_shared_string(),
            size: self.size,
            defined_size: self.defined_size
                .unwrap_or(nt::FontSize::Px(self.size))
                .to_logical_px(ctx.dpi),
            weight: self.weight as i32,
            style: self.style.into(),
            color: self.color.into_color(),
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
