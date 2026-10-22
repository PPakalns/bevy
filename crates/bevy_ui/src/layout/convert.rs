use taffy::style_helpers;

use crate::{
    AlignContent, AlignItems, AlignSelf, BoxSizing, Display, FlexDirection, FlexWrap, GridAutoFlow,
    GridPlacement, GridTrack, InlineDirection, JustifyContent, JustifyItems, JustifySelf,
    MaxTrackSizingFunction, MinTrackSizingFunction, OverflowAxis, PositionType, UiRect, Val,
};

use super::LayoutContext;

impl Val {
    #[inline(always)]
    pub(super) fn into_length_percentage_auto(
        self,
        context: &LayoutContext,
    ) -> taffy::style::LengthPercentageAuto {
        match self {
            Val::Auto => style_helpers::auto(),
            Val::Percent(value) => style_helpers::percent(value / 100.),
            Val::Px(value) => style_helpers::length(context.scale_factor * value),
            Val::VMin(value) => {
                style_helpers::length(context.physical_size.min_element() * value / 100.)
            }
            Val::VMax(value) => {
                style_helpers::length(context.physical_size.max_element() * value / 100.)
            }
            Val::Vw(value) => style_helpers::length(context.physical_size.x * value / 100.),
            Val::Vh(value) => style_helpers::length(context.physical_size.y * value / 100.),
            Val::Em(value) => style_helpers::length(context.scale_factor * value * context.em_size),
            Val::Rem(value) => {
                style_helpers::length(context.scale_factor * value * context.rem_size)
            }
        }
    }

    #[inline(always)]
    pub(super) fn into_length_percentage(
        self,
        context: &LayoutContext,
    ) -> taffy::style::LengthPercentage {
        match self {
            Val::Auto => style_helpers::length(0.0_f32),
            Val::Percent(value) => style_helpers::percent(value / 100.),
            Val::Px(value) => style_helpers::length(context.scale_factor * value),
            Val::VMin(value) => {
                style_helpers::length(context.physical_size.min_element() * value / 100.)
            }
            Val::VMax(value) => {
                style_helpers::length(context.physical_size.max_element() * value / 100.)
            }
            Val::Vw(value) => style_helpers::length(context.physical_size.x * value / 100.),
            Val::Vh(value) => style_helpers::length(context.physical_size.y * value / 100.),
            Val::Em(value) => style_helpers::length(context.scale_factor * value * context.em_size),
            Val::Rem(value) => {
                style_helpers::length(context.scale_factor * value * context.rem_size)
            }
        }
    }

    // Only used for border values. Floors length based borders to a whole number of
    // physical pixels, following the CSS "snap a length as a border width" algorithm
    // used by browsers:
    // https://www.w3.org/TR/css-values-4/#snap-a-length-as-a-border-width
    //
    // * A non-zero border smaller than one physical pixel is rounded up to one pixel.
    // * A border larger than one physical pixel is rounded down to a whole pixel.
    #[inline(always)]
    pub(super) fn into_length_percentage_for_border(
        self,
        context: &LayoutContext,
    ) -> taffy::style::LengthPercentage {
        let length = self.into_length_percentage(context);
        let raw = length.into_raw();
        // we're only snapping if sized in non-zero px
        if raw.tag() == taffy::style::CompactLength::LENGTH_TAG && raw.value() > 0.0 {
            taffy::style::LengthPercentage::length(raw.value().floor().max(1.0))
        } else {
            length
        }
    }

    #[inline(always)]
    pub(super) fn into_dimension(self, context: &LayoutContext) -> taffy::style::Dimension {
        self.into_length_percentage_auto(context).into()
    }
}

impl UiRect {
    #[inline(always)]
    pub(super) fn map_to_taffy_rect<T>(
        self,
        map_fn: impl Fn(Val) -> T,
    ) -> taffy::geometry::Rect<T> {
        taffy::geometry::Rect {
            left: map_fn(self.left),
            right: map_fn(self.right),
            top: map_fn(self.top),
            bottom: map_fn(self.bottom),
        }
    }
}

impl From<AlignItems> for Option<taffy::style::AlignItems> {
    #[inline(always)]
    fn from(value: AlignItems) -> Self {
        match value {
            AlignItems::Default => None,
            AlignItems::Start => taffy::style::AlignItems::START.into(),
            AlignItems::StartSafe => taffy::style::AlignItems::SAFE_START.into(),
            AlignItems::End => taffy::style::AlignItems::END.into(),
            AlignItems::EndSafe => taffy::style::AlignItems::SAFE_END.into(),
            AlignItems::FlexStart => taffy::style::AlignItems::FLEX_START.into(),
            AlignItems::FlexStartSafe => taffy::style::AlignItems::SAFE_FLEX_START.into(),
            AlignItems::FlexEnd => taffy::style::AlignItems::FLEX_END.into(),
            AlignItems::FlexEndSafe => taffy::style::AlignItems::SAFE_FLEX_END.into(),
            AlignItems::Center => taffy::style::AlignItems::CENTER.into(),
            AlignItems::CenterSafe => taffy::style::AlignItems::SAFE_CENTER.into(),
            AlignItems::Baseline => taffy::style::AlignItems::BASELINE.into(),
            AlignItems::Stretch => taffy::style::AlignItems::STRETCH.into(),
        }
    }
}

impl From<JustifyItems> for Option<taffy::style::JustifyItems> {
    #[inline(always)]
    fn from(value: JustifyItems) -> Self {
        match value {
            JustifyItems::Default => None,
            JustifyItems::Start => taffy::style::JustifyItems::START.into(),
            JustifyItems::StartSafe => taffy::style::JustifyItems::SAFE_START.into(),
            JustifyItems::End => taffy::style::JustifyItems::END.into(),
            JustifyItems::EndSafe => taffy::style::JustifyItems::SAFE_END.into(),
            JustifyItems::Center => taffy::style::JustifyItems::CENTER.into(),
            JustifyItems::CenterSafe => taffy::style::JustifyItems::SAFE_CENTER.into(),
            JustifyItems::Baseline => taffy::style::JustifyItems::BASELINE.into(),
            JustifyItems::Stretch => taffy::style::JustifyItems::STRETCH.into(),
        }
    }
}

impl From<AlignSelf> for Option<taffy::style::AlignSelf> {
    #[inline(always)]
    fn from(value: AlignSelf) -> Self {
        match value {
            AlignSelf::Auto => None,
            AlignSelf::Start => taffy::style::AlignSelf::START.into(),
            AlignSelf::StartSafe => taffy::style::AlignSelf::SAFE_START.into(),
            AlignSelf::End => taffy::style::AlignSelf::END.into(),
            AlignSelf::EndSafe => taffy::style::AlignSelf::SAFE_END.into(),
            AlignSelf::FlexStart => taffy::style::AlignSelf::FLEX_START.into(),
            AlignSelf::FlexStartSafe => taffy::style::AlignSelf::SAFE_FLEX_START.into(),
            AlignSelf::FlexEnd => taffy::style::AlignSelf::FLEX_END.into(),
            AlignSelf::FlexEndSafe => taffy::style::AlignSelf::SAFE_FLEX_END.into(),
            AlignSelf::Center => taffy::style::AlignSelf::CENTER.into(),
            AlignSelf::CenterSafe => taffy::style::AlignSelf::SAFE_CENTER.into(),
            AlignSelf::Baseline => taffy::style::AlignSelf::BASELINE.into(),
            AlignSelf::Stretch => taffy::style::AlignSelf::STRETCH.into(),
        }
    }
}

impl From<JustifySelf> for Option<taffy::style::JustifySelf> {
    #[inline(always)]
    fn from(value: JustifySelf) -> Self {
        match value {
            JustifySelf::Auto => None,
            JustifySelf::Start => taffy::style::JustifySelf::START.into(),
            JustifySelf::StartSafe => taffy::style::JustifySelf::SAFE_START.into(),
            JustifySelf::End => taffy::style::JustifySelf::END.into(),
            JustifySelf::EndSafe => taffy::style::JustifySelf::SAFE_END.into(),
            JustifySelf::Center => taffy::style::JustifySelf::CENTER.into(),
            JustifySelf::CenterSafe => taffy::style::JustifySelf::SAFE_CENTER.into(),
            JustifySelf::Baseline => taffy::style::JustifySelf::BASELINE.into(),
            JustifySelf::Stretch => taffy::style::JustifySelf::STRETCH.into(),
        }
    }
}

impl From<AlignContent> for Option<taffy::style::AlignContent> {
    #[inline(always)]
    fn from(value: AlignContent) -> Self {
        match value {
            AlignContent::Default => None,
            AlignContent::Start => taffy::style::AlignContent::START.into(),
            AlignContent::StartSafe => taffy::style::AlignContent::SAFE_START.into(),
            AlignContent::End => taffy::style::AlignContent::END.into(),
            AlignContent::EndSafe => taffy::style::AlignContent::SAFE_END.into(),
            AlignContent::FlexStart => taffy::style::AlignContent::FLEX_START.into(),
            AlignContent::FlexStartSafe => taffy::style::AlignContent::SAFE_FLEX_START.into(),
            AlignContent::FlexEnd => taffy::style::AlignContent::FLEX_END.into(),
            AlignContent::FlexEndSafe => taffy::style::AlignContent::SAFE_FLEX_END.into(),
            AlignContent::Center => taffy::style::AlignContent::CENTER.into(),
            AlignContent::CenterSafe => taffy::style::AlignContent::SAFE_CENTER.into(),
            AlignContent::Stretch => taffy::style::AlignContent::STRETCH.into(),
            AlignContent::SpaceBetween => taffy::style::AlignContent::SPACE_BETWEEN.into(),
            AlignContent::SpaceAround => taffy::style::AlignContent::SPACE_AROUND.into(),
            AlignContent::SpaceEvenly => taffy::style::AlignContent::SPACE_EVENLY.into(),
        }
    }
}

impl From<JustifyContent> for Option<taffy::style::JustifyContent> {
    #[inline(always)]
    fn from(value: JustifyContent) -> Self {
        match value {
            JustifyContent::Default => None,
            JustifyContent::Start => taffy::style::JustifyContent::START.into(),
            JustifyContent::StartSafe => taffy::style::JustifyContent::SAFE_START.into(),
            JustifyContent::End => taffy::style::JustifyContent::END.into(),
            JustifyContent::EndSafe => taffy::style::JustifyContent::SAFE_END.into(),
            JustifyContent::FlexStart => taffy::style::JustifyContent::FLEX_START.into(),
            JustifyContent::FlexStartSafe => taffy::style::JustifyContent::SAFE_FLEX_START.into(),
            JustifyContent::FlexEnd => taffy::style::JustifyContent::FLEX_END.into(),
            JustifyContent::FlexEndSafe => taffy::style::JustifyContent::SAFE_FLEX_END.into(),
            JustifyContent::Center => taffy::style::JustifyContent::CENTER.into(),
            JustifyContent::CenterSafe => taffy::style::JustifyContent::SAFE_CENTER.into(),
            JustifyContent::Stretch => taffy::style::JustifyContent::STRETCH.into(),
            JustifyContent::SpaceBetween => taffy::style::JustifyContent::SPACE_BETWEEN.into(),
            JustifyContent::SpaceAround => taffy::style::JustifyContent::SPACE_AROUND.into(),
            JustifyContent::SpaceEvenly => taffy::style::JustifyContent::SPACE_EVENLY.into(),
        }
    }
}

impl From<InlineDirection> for taffy::style::Direction {
    #[inline(always)]
    fn from(direction: InlineDirection) -> Self {
        match direction {
            InlineDirection::Ltr => taffy::style::Direction::Ltr,
            InlineDirection::Rtl => taffy::style::Direction::Rtl,
        }
    }
}

impl From<Display> for taffy::style::Display {
    #[inline(always)]
    fn from(value: Display) -> Self {
        match value {
            Display::Flex => taffy::style::Display::Flex,
            Display::Grid => taffy::style::Display::Grid,
            Display::Block => taffy::style::Display::Block,
            Display::None => taffy::style::Display::None,
        }
    }
}

impl From<BoxSizing> for taffy::style::BoxSizing {
    #[inline(always)]
    fn from(value: BoxSizing) -> Self {
        match value {
            BoxSizing::BorderBox => taffy::style::BoxSizing::BorderBox,
            BoxSizing::ContentBox => taffy::style::BoxSizing::ContentBox,
        }
    }
}

impl From<OverflowAxis> for taffy::style::Overflow {
    #[inline(always)]
    fn from(value: OverflowAxis) -> Self {
        match value {
            OverflowAxis::Visible => taffy::style::Overflow::Visible,
            OverflowAxis::Clip => taffy::style::Overflow::Clip,
            OverflowAxis::Hidden => taffy::style::Overflow::Hidden,
            OverflowAxis::Scroll => taffy::style::Overflow::Scroll,
        }
    }
}

impl From<FlexDirection> for taffy::style::FlexDirection {
    #[inline(always)]
    fn from(value: FlexDirection) -> Self {
        match value {
            FlexDirection::Row => taffy::style::FlexDirection::Row,
            FlexDirection::Column => taffy::style::FlexDirection::Column,
            FlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
            FlexDirection::ColumnReverse => taffy::style::FlexDirection::ColumnReverse,
        }
    }
}

impl From<PositionType> for taffy::style::Position {
    #[inline(always)]
    fn from(value: PositionType) -> Self {
        match value {
            PositionType::Relative => taffy::style::Position::Relative,
            PositionType::Absolute => taffy::style::Position::Absolute,
        }
    }
}

impl From<FlexWrap> for taffy::style::FlexWrap {
    #[inline(always)]
    fn from(value: FlexWrap) -> Self {
        match value {
            FlexWrap::NoWrap => taffy::style::FlexWrap::NoWrap,
            FlexWrap::Wrap => taffy::style::FlexWrap::Wrap,
            FlexWrap::WrapReverse => taffy::style::FlexWrap::WrapReverse,
        }
    }
}

impl From<GridAutoFlow> for taffy::style::GridAutoFlow {
    #[inline(always)]
    fn from(value: GridAutoFlow) -> Self {
        match value {
            GridAutoFlow::Row => taffy::style::GridAutoFlow::Row,
            GridAutoFlow::RowDense => taffy::style::GridAutoFlow::RowDense,
            GridAutoFlow::Column => taffy::style::GridAutoFlow::Column,
            GridAutoFlow::ColumnDense => taffy::style::GridAutoFlow::ColumnDense,
        }
    }
}

impl From<GridPlacement> for taffy::geometry::Line<taffy::style::GridPlacement<String>> {
    #[inline(always)]
    fn from(value: GridPlacement) -> Self {
        let span = value.get_span().unwrap_or(1);
        match (value.get_start(), value.get_end()) {
            (Some(start), Some(end)) => taffy::geometry::Line {
                start: style_helpers::line(start),
                end: style_helpers::line(end),
            },
            (Some(start), None) => taffy::geometry::Line {
                start: style_helpers::line(start),
                end: style_helpers::span(span),
            },
            (None, Some(end)) => taffy::geometry::Line {
                start: style_helpers::span(span),
                end: style_helpers::line(end),
            },
            (None, None) => style_helpers::span(span),
        }
    }
}

impl MinTrackSizingFunction {
    #[inline(always)]
    fn into_taffy(self, context: &LayoutContext) -> taffy::style::MinTrackSizingFunction {
        match self {
            MinTrackSizingFunction::Px(val) => Val::Px(val).into_length_percentage(context).into(),
            MinTrackSizingFunction::Percent(val) => {
                Val::Percent(val).into_length_percentage(context).into()
            }
            MinTrackSizingFunction::Em(val) => Val::Em(val).into_length_percentage(context).into(),
            MinTrackSizingFunction::Rem(val) => {
                Val::Rem(val).into_length_percentage(context).into()
            }
            MinTrackSizingFunction::Auto => taffy::style::MinTrackSizingFunction::auto(),
            MinTrackSizingFunction::MinContent => {
                taffy::style::MinTrackSizingFunction::min_content()
            }
            MinTrackSizingFunction::MaxContent => {
                taffy::style::MinTrackSizingFunction::max_content()
            }
            MinTrackSizingFunction::VMin(val) => {
                Val::VMin(val).into_length_percentage(context).into()
            }
            MinTrackSizingFunction::VMax(val) => {
                Val::VMax(val).into_length_percentage(context).into()
            }
            MinTrackSizingFunction::Vh(val) => Val::Vh(val).into_length_percentage(context).into(),
            MinTrackSizingFunction::Vw(val) => Val::Vw(val).into_length_percentage(context).into(),
        }
    }
}

impl MaxTrackSizingFunction {
    #[inline(always)]
    fn into_taffy(self, context: &LayoutContext) -> taffy::style::MaxTrackSizingFunction {
        match self {
            MaxTrackSizingFunction::Px(val) => Val::Px(val).into_length_percentage(context).into(),
            MaxTrackSizingFunction::Percent(val) => {
                Val::Percent(val).into_length_percentage(context).into()
            }
            MaxTrackSizingFunction::Em(val) => Val::Em(val).into_length_percentage(context).into(),
            MaxTrackSizingFunction::Rem(val) => {
                Val::Rem(val).into_length_percentage(context).into()
            }
            MaxTrackSizingFunction::Auto => taffy::style::MaxTrackSizingFunction::auto(),
            MaxTrackSizingFunction::MinContent => {
                taffy::style::MaxTrackSizingFunction::min_content()
            }
            MaxTrackSizingFunction::MaxContent => {
                taffy::style::MaxTrackSizingFunction::max_content()
            }
            MaxTrackSizingFunction::FitContentPx(val) => {
                taffy::style::MaxTrackSizingFunction::fit_content_px(
                    Val::Px(val)
                        .into_length_percentage(context)
                        .into_raw()
                        .value(),
                )
            }
            MaxTrackSizingFunction::FitContentPercent(val) => {
                taffy::style::MaxTrackSizingFunction::fit_content_percent(
                    Val::Percent(val)
                        .into_length_percentage(context)
                        .into_raw()
                        .value(),
                )
            }
            MaxTrackSizingFunction::Fraction(fraction) => {
                taffy::style::MaxTrackSizingFunction::fr(fraction)
            }
            MaxTrackSizingFunction::VMin(val) => {
                Val::VMin(val).into_length_percentage(context).into()
            }
            MaxTrackSizingFunction::VMax(val) => {
                Val::VMax(val).into_length_percentage(context).into()
            }
            MaxTrackSizingFunction::Vh(val) => Val::Vh(val).into_length_percentage(context).into(),
            MaxTrackSizingFunction::Vw(val) => Val::Vw(val).into_length_percentage(context).into(),
        }
    }
}

impl GridTrack {
    #[inline(always)]
    pub(super) fn into_taffy_track(
        self,
        context: &LayoutContext,
    ) -> taffy::style::TrackSizingFunction {
        let min = self.min_sizing_function.into_taffy(context);
        let max = self.max_sizing_function.into_taffy(context);
        style_helpers::minmax(min, max)
    }
}

#[cfg(test)]
mod tests {
    use bevy_math::Vec2;
    use bevy_text::{EmSize, RemSize, DEFAULT_REM_SIZE_PX};
    use taffy::{
        CoreStyle, FlexboxContainerStyle, FlexboxItemStyle, GenericGridTemplateComponent,
        GenericRepetition, GridContainerStyle, GridItemStyle,
    };

    use crate::{layout::node_style::NodeStyle, BorderRadius, Node, RepeatedGridTrack};

    use super::*;

    #[test]
    fn test_convert_from() {
        use sh::TaffyZero;
        use taffy::style_helpers as sh;

        let node = Node {
            display: Display::Flex,
            box_sizing: BoxSizing::ContentBox,
            position_type: PositionType::Absolute,
            left: Val::ZERO,
            right: Val::Percent(50.),
            top: Val::Px(12.),
            bottom: Val::Auto,
            flex_direction: FlexDirection::ColumnReverse,
            flex_wrap: FlexWrap::WrapReverse,
            align_items: AlignItems::Baseline,
            align_self: AlignSelf::Start,
            align_content: AlignContent::SpaceAround,
            justify_items: JustifyItems::Default,
            justify_self: JustifySelf::Center,
            justify_content: JustifyContent::SpaceEvenly,
            direction: InlineDirection::Ltr,
            margin: UiRect {
                left: Val::ZERO,
                right: Val::Px(10.),
                top: Val::Percent(15.),
                bottom: Val::Auto,
            },
            padding: UiRect {
                left: Val::Percent(13.),
                right: Val::Px(21.),
                top: Val::Auto,
                bottom: Val::ZERO,
            },
            border: UiRect {
                left: Val::Px(14.),
                right: Val::ZERO,
                top: Val::Auto,
                bottom: Val::Percent(31.),
            },
            border_radius: BorderRadius::DEFAULT,
            flex_grow: 1.,
            flex_shrink: 0.,
            flex_basis: Val::ZERO,
            width: Val::ZERO,
            height: Val::Auto,
            min_width: Val::ZERO,
            min_height: Val::ZERO,
            max_width: Val::Auto,
            max_height: Val::ZERO,
            aspect_ratio: None,
            overflow: crate::Overflow::clip(),
            overflow_clip_margin: crate::OverflowClipMargin::default(),
            scrollbar_width: 7.,
            column_gap: Val::ZERO,
            row_gap: Val::ZERO,
            grid_auto_flow: GridAutoFlow::ColumnDense,
            grid_template_rows: vec![
                GridTrack::px(10.0),
                GridTrack::percent(50.0),
                GridTrack::fr(1.0),
            ],
            grid_template_columns: RepeatedGridTrack::px(5, 10.0),
            grid_auto_rows: vec![
                GridTrack::fit_content_px(10.0),
                GridTrack::fit_content_percent(25.0),
                GridTrack::flex(2.0),
            ],
            grid_auto_columns: vec![
                GridTrack::auto(),
                GridTrack::min_content(),
                GridTrack::max_content(),
            ],
            grid_column: GridPlacement::start(4),
            grid_row: GridPlacement::span(3),
        };
        let viewport_values = LayoutContext::new(
            1.0,
            Vec2::new(800., 600.),
            EmSize(DEFAULT_REM_SIZE_PX),
            RemSize(DEFAULT_REM_SIZE_PX),
        );
        let taffy_sty = NodeStyle::new(&node, viewport_values);
        assert_eq!(taffy_sty.display(), taffy::style::Display::Flex);
        assert_eq!(taffy_sty.box_sizing(), taffy::style::BoxSizing::ContentBox);
        assert_eq!(taffy_sty.position(), taffy::style::Position::Absolute);
        assert_eq!(taffy_sty.direction(), taffy::style::Direction::Ltr);
        let inset = taffy_sty.inset();
        assert_eq!(inset.left, taffy::style::LengthPercentageAuto::ZERO);
        assert_eq!(
            inset.right,
            taffy::style::LengthPercentageAuto::percent(0.5)
        );
        assert_eq!(inset.top, taffy::style::LengthPercentageAuto::length(12.));
        assert_eq!(inset.bottom, taffy::style::LengthPercentageAuto::auto());
        assert_eq!(
            taffy_sty.flex_direction(),
            taffy::style::FlexDirection::ColumnReverse
        );
        assert_eq!(taffy_sty.flex_wrap(), taffy::style::FlexWrap::WrapReverse);
        assert_eq!(
            FlexboxContainerStyle::align_items(&taffy_sty),
            Some(taffy::style::AlignItems::BASELINE)
        );
        assert_eq!(
            FlexboxItemStyle::align_self(&taffy_sty),
            Some(taffy::style::AlignSelf::START)
        );
        assert_eq!(
            FlexboxContainerStyle::align_content(&taffy_sty),
            Some(taffy::style::AlignContent::SPACE_AROUND)
        );
        assert_eq!(
            FlexboxContainerStyle::justify_content(&taffy_sty),
            Some(taffy::style::JustifyContent::SPACE_EVENLY)
        );
        assert_eq!(taffy_sty.justify_items(), None);
        assert_eq!(
            taffy_sty.justify_self(),
            Some(taffy::style::JustifySelf::CENTER)
        );
        let margin = taffy_sty.margin();
        assert_eq!(margin.left, taffy::style::LengthPercentageAuto::ZERO);
        assert_eq!(
            margin.right,
            taffy::style::LengthPercentageAuto::length(10.)
        );
        assert_eq!(
            margin.top,
            taffy::style::LengthPercentageAuto::percent(0.15)
        );
        assert_eq!(margin.bottom, taffy::style::LengthPercentageAuto::auto());
        let padding = taffy_sty.padding();
        assert_eq!(padding.left, taffy::style::LengthPercentage::percent(0.13));
        assert_eq!(padding.right, taffy::style::LengthPercentage::length(21.));
        assert_eq!(padding.top, taffy::style::LengthPercentage::ZERO);
        assert_eq!(padding.bottom, taffy::style::LengthPercentage::ZERO);
        let border = taffy_sty.border();
        assert_eq!(border.left, taffy::style::LengthPercentage::length(14.));
        assert_eq!(border.right, taffy::style::LengthPercentage::ZERO);
        assert_eq!(border.top, taffy::style::LengthPercentage::ZERO);
        assert_eq!(border.bottom, taffy::style::LengthPercentage::percent(0.31));
        assert_eq!(taffy_sty.flex_grow(), 1.);
        assert_eq!(taffy_sty.flex_shrink(), 0.);
        assert_eq!(taffy_sty.flex_basis(), taffy::style::Dimension::ZERO);
        let size = taffy_sty.size();
        assert_eq!(size.width, taffy::style::Dimension::ZERO);
        assert_eq!(size.height, taffy::style::Dimension::auto());
        let min_size = taffy_sty.min_size();
        assert_eq!(min_size.width, taffy::style::LengthPercentageAuto::ZERO);
        assert_eq!(min_size.height, taffy::style::LengthPercentageAuto::ZERO);
        let max_size = taffy_sty.max_size();
        assert_eq!(max_size.width, taffy::style::LengthPercentageAuto::auto());
        assert_eq!(max_size.height, taffy::style::LengthPercentageAuto::ZERO);
        assert_eq!(taffy_sty.aspect_ratio(), None);
        assert_eq!(taffy_sty.scrollbar_width(), 7.);
        let gap = FlexboxContainerStyle::gap(&taffy_sty);
        assert_eq!(gap.width, taffy::style::LengthPercentage::ZERO);
        assert_eq!(gap.height, taffy::style::LengthPercentage::ZERO);
        assert_eq!(
            taffy_sty.grid_auto_flow(),
            taffy::style::GridAutoFlow::ColumnDense
        );

        let mut rows = taffy_sty.grid_template_rows().unwrap();
        assert!(matches!(
            rows.next(),
            Some(GenericGridTemplateComponent::Single(t)) if t == sh::length(10_f32)
        ));
        assert!(matches!(
            rows.next(),
            Some(GenericGridTemplateComponent::Single(t)) if t == sh::percent(0.5_f32)
        ));
        assert!(matches!(
            rows.next(),
            Some(GenericGridTemplateComponent::Single(t)) if t == sh::fr(1_f32)
        ));
        assert!(rows.next().is_none());

        let mut columns = taffy_sty.grid_template_columns().unwrap();
        assert!(matches!(
            columns.next(),
            Some(GenericGridTemplateComponent::Repeat(ref repeat))
                if GenericRepetition::count(repeat) == taffy::style::RepetitionCount::Count(5)
                    && GenericRepetition::tracks(repeat).collect::<Vec<_>>()
                        == vec![sh::length(10_f32)]
        ));
        assert!(columns.next().is_none());

        assert_eq!(
            taffy_sty.grid_auto_rows().collect::<Vec<_>>(),
            vec![
                sh::fit_content(taffy::style::LengthPercentage::length(10.0)),
                sh::fit_content(taffy::style::LengthPercentage::percent(0.25)),
                sh::minmax(sh::length(0_f32), sh::fr(2_f32)),
            ]
        );
        assert_eq!(
            taffy_sty.grid_auto_columns().collect::<Vec<_>>(),
            vec![sh::auto(), sh::min_content(), sh::max_content()]
        );
        assert_eq!(
            taffy_sty.grid_column(),
            taffy::geometry::Line {
                start: sh::line(4),
                end: sh::span(1)
            }
        );
        assert_eq!(taffy_sty.grid_row(), sh::span(3));
    }

    #[test]
    fn test_into_length_percentage() {
        use taffy::style::LengthPercentage;
        let context = LayoutContext::new(2.0, Vec2::new(800., 600.), EmSize(10.), RemSize(20.));
        let cases = [
            (Val::Auto, LengthPercentage::length(0.)),
            (Val::Percent(1.), LengthPercentage::percent(0.01)),
            (Val::Px(1.), LengthPercentage::length(2.)),
            (Val::Vw(1.), LengthPercentage::length(8.)),
            (Val::Vh(1.), LengthPercentage::length(6.)),
            (Val::VMin(2.), LengthPercentage::length(12.)),
            (Val::VMax(2.), LengthPercentage::length(16.)),
            (Val::Em(2.), LengthPercentage::length(40.)),
            (Val::Rem(3.), LengthPercentage::length(120.)),
        ];
        for (val, length) in cases {
            assert!({
                let lhs = val.into_length_percentage(&context).into_raw().value();
                let rhs = length.into_raw().value();
                (lhs - rhs).abs() < 0.0001
            });
        }
    }
}
