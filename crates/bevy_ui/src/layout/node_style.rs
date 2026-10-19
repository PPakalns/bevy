use bevy_ecs::{
    change_detection::DetectChanges,
    entity::Entity,
    query::QueryEntityError,
    system::{Query, Res, SystemParam},
};
use bevy_text::{EmSize, RemSize};

use core::marker::PhantomData;

use crate::{ComputedUiRenderTargetInfo, GridTrack, GridTrackRepetition, Node, RepeatedGridTrack};

use super::LayoutContext;

/// UI [`Node`] effective style, compatible with taffy layout calculations.
#[derive(Copy, Clone)]
pub struct NodeStyle<'a> {
    pub(super) node: &'a Node,
    pub(super) context: LayoutContext,
}

impl<'a> NodeStyle<'a> {
    /// Builds a [`NodeStyle`] directly from a UI [`Node`] and a [`LayoutContext`].
    pub const fn new(node: &'a Node, context: LayoutContext) -> Self {
        Self { node, context }
    }

    /// Retrieve corresponding [`taffy::Display`]
    pub fn display(&self) -> taffy::Display {
        self.node.display.into()
    }
}

impl<'a> taffy::CoreStyle for NodeStyle<'a> {
    type CustomIdent = String;

    #[inline(always)]
    fn box_generation_mode(&self) -> taffy::BoxGenerationMode {
        match self.display() {
            taffy::Display::None => taffy::BoxGenerationMode::None,
            _ => taffy::BoxGenerationMode::Normal,
        }
    }

    #[inline(always)]
    fn is_block(&self) -> bool {
        matches!(self.display(), taffy::Display::Block)
    }

    #[inline(always)]
    fn box_sizing(&self) -> taffy::BoxSizing {
        self.node.box_sizing.into()
    }

    #[inline(always)]
    fn direction(&self) -> taffy::Direction {
        self.node.direction.into()
    }

    #[inline(always)]
    fn overflow(&self) -> taffy::Point<taffy::Overflow> {
        taffy::Point {
            x: self.node.overflow.x.into(),
            y: self.node.overflow.y.into(),
        }
    }

    #[inline(always)]
    fn scrollbar_width(&self) -> f32 {
        self.node.scrollbar_width * self.context.scale_factor
    }

    #[inline(always)]
    fn position(&self) -> taffy::Position {
        self.node.position_type.into()
    }

    #[inline(always)]
    fn inset(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        taffy::Rect {
            left: self.node.left.into_length_percentage_auto(&self.context),
            right: self.node.right.into_length_percentage_auto(&self.context),
            top: self.node.top.into_length_percentage_auto(&self.context),
            bottom: self.node.bottom.into_length_percentage_auto(&self.context),
        }
    }

    #[inline(always)]
    fn size(&self) -> taffy::Size<taffy::Dimension> {
        taffy::Size {
            width: self.node.width.into_dimension(&self.context),
            height: self.node.height.into_dimension(&self.context),
        }
    }

    #[inline(always)]
    fn min_size(&self) -> taffy::Size<taffy::LengthPercentageAuto> {
        taffy::Size {
            width: self
                .node
                .min_width
                .into_length_percentage_auto(&self.context),
            height: self
                .node
                .min_height
                .into_length_percentage_auto(&self.context),
        }
    }

    #[inline(always)]
    fn max_size(&self) -> taffy::Size<taffy::LengthPercentageAuto> {
        taffy::Size {
            width: self
                .node
                .max_width
                .into_length_percentage_auto(&self.context),
            height: self
                .node
                .max_height
                .into_length_percentage_auto(&self.context),
        }
    }

    #[inline(always)]
    fn aspect_ratio(&self) -> Option<f32> {
        self.node.aspect_ratio
    }

    #[inline(always)]
    fn margin(&self) -> taffy::Rect<taffy::LengthPercentageAuto> {
        self.node
            .margin
            .map_to_taffy_rect(|m| m.into_length_percentage_auto(&self.context))
    }

    #[inline(always)]
    fn padding(&self) -> taffy::Rect<taffy::LengthPercentage> {
        self.node
            .padding
            .map_to_taffy_rect(|m| m.into_length_percentage(&self.context))
    }

    #[inline(always)]
    fn border(&self) -> taffy::Rect<taffy::LengthPercentage> {
        self.node
            .border
            .map_to_taffy_rect(|m| m.into_length_percentage_for_border(&self.context))
    }

    #[inline(always)]
    fn is_compressible_replaced(&self) -> bool {
        false
    }

    #[inline(always)]
    fn contain(&self) -> taffy::Contain {
        taffy::Contain::NONE
    }
}

impl<'a> taffy::FlexboxContainerStyle for NodeStyle<'a> {
    #[inline(always)]
    fn flex_direction(&self) -> taffy::FlexDirection {
        self.node.flex_direction.into()
    }

    #[inline(always)]
    fn flex_wrap(&self) -> taffy::FlexWrap {
        self.node.flex_wrap.into()
    }

    #[inline(always)]
    fn gap(&self) -> taffy::Size<taffy::LengthPercentage> {
        taffy::Size {
            width: self.node.column_gap.into_length_percentage(&self.context),
            height: self.node.row_gap.into_length_percentage(&self.context),
        }
    }

    #[inline(always)]
    fn align_content(&self) -> Option<taffy::AlignContent> {
        self.node.align_content.into()
    }

    #[inline(always)]
    fn align_items(&self) -> Option<taffy::AlignItems> {
        self.node.align_items.into()
    }

    #[inline(always)]
    fn justify_content(&self) -> Option<taffy::JustifyContent> {
        self.node.justify_content.into()
    }
}

impl<'a> taffy::FlexboxItemStyle for NodeStyle<'a> {
    #[inline(always)]
    fn flex_basis(&self) -> taffy::Dimension {
        self.node.flex_basis.into_dimension(&self.context)
    }

    #[inline(always)]
    fn flex_grow(&self) -> f32 {
        self.node.flex_grow
    }

    #[inline(always)]
    fn flex_shrink(&self) -> f32 {
        self.node.flex_shrink
    }

    #[inline(always)]
    fn align_self(&self) -> Option<taffy::AlignSelf> {
        self.node.align_self.into()
    }
}

impl<'a> taffy::BlockContainerStyle for NodeStyle<'a> {
    #[inline(always)]
    fn align_content(&self) -> Option<taffy::AlignContent> {
        self.node.align_content.into()
    }

    #[inline(always)]
    fn text_align(&self) -> taffy::TextAlign {
        taffy::prelude::Style::<Self::CustomIdent>::DEFAULT.text_align
    }
}

impl<'a> taffy::BlockItemStyle for NodeStyle<'a> {
    #[inline(always)]
    fn is_table(&self) -> bool {
        false
    }
}

/// Lazy iterator over [`RepeatedGridTrack`].
#[derive(Clone)]
pub struct GridTrackRepetitionRef<'a> {
    tracks: &'a [GridTrack],
    repetition: GridTrackRepetition,
    context: LayoutContext,
}

impl<'a> taffy::GenericRepetition for GridTrackRepetitionRef<'a> {
    type CustomIdent = String;

    type RepetitionTrackList<'b>
        = GridTrackIter<'b>
    where
        Self: 'b;

    type TemplateLineNames<'b>
        = EmptyLineNames<'b>
    where
        Self: 'b;

    #[inline(always)]
    fn count(&self) -> taffy::RepetitionCount {
        match self.repetition {
            GridTrackRepetition::Count(n) => taffy::RepetitionCount::Count(n),
            GridTrackRepetition::AutoFit => taffy::RepetitionCount::AutoFit,
            GridTrackRepetition::AutoFill => taffy::RepetitionCount::AutoFill,
        }
    }

    #[inline(always)]
    fn tracks(&self) -> Self::RepetitionTrackList<'_> {
        GridTrackIter {
            tracks: self.tracks.iter(),
            context: self.context,
        }
    }

    #[inline(always)]
    fn lines_names(&self) -> Self::TemplateLineNames<'_> {
        EmptyLineNames::new()
    }
}

/// Lazily converts a slice of [`GridTrack`]s into taffy `TrackSizingFunction`s.
#[derive(Clone)]
pub struct GridTrackIter<'a> {
    tracks: core::slice::Iter<'a, GridTrack>,
    context: LayoutContext,
}

impl<'a> Iterator for GridTrackIter<'a> {
    type Item = taffy::TrackSizingFunction;

    fn next(&mut self) -> Option<Self::Item> {
        self.tracks
            .next()
            .map(|track| track.into_taffy_track(&self.context))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.tracks.size_hint()
    }
}

impl<'a> ExactSizeIterator for GridTrackIter<'a> {
    fn len(&self) -> usize {
        self.tracks.len()
    }
}

/// Lazily converts a slice of [`RepeatedGridTrack`]s into taffy grid template components.
#[derive(Clone)]
pub struct GridTemplateIter<'a> {
    tracks: core::slice::Iter<'a, RepeatedGridTrack>,
    context: LayoutContext,
}

impl<'a> Iterator for GridTemplateIter<'a> {
    type Item = taffy::GenericGridTemplateComponent<String, GridTrackRepetitionRef<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tracks.next().map(|repeated| {
            if repeated.tracks.len() == 1
                && matches!(repeated.repetition, GridTrackRepetition::Count(1))
            {
                taffy::GenericGridTemplateComponent::Single(
                    repeated.tracks[0].into_taffy_track(&self.context),
                )
            } else {
                taffy::GenericGridTemplateComponent::Repeat(GridTrackRepetitionRef {
                    tracks: repeated.tracks.as_slice(),
                    repetition: repeated.repetition,
                    context: self.context,
                })
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.tracks.size_hint()
    }
}

impl<'a> ExactSizeIterator for GridTemplateIter<'a> {
    fn len(&self) -> usize {
        self.tracks.len()
    }
}

impl<'a> taffy::GridContainerStyle for NodeStyle<'a> {
    type Repetition<'b>
        = GridTrackRepetitionRef<'b>
    where
        Self: 'b;

    type TemplateTrackList<'b>
        = GridTemplateIter<'b>
    where
        Self: 'b;

    type AutoTrackList<'b>
        = GridTrackIter<'b>
    where
        Self: 'b;

    type TemplateLineNames<'b>
        = EmptyLineNames<'b>
    where
        Self: 'b;

    type GridTemplateAreas<'b>
        = core::iter::Empty<taffy::GridTemplateArea<String>>
    where
        Self: 'b;

    #[inline(always)]
    fn grid_template_rows(&self) -> Option<Self::TemplateTrackList<'_>> {
        Some(GridTemplateIter {
            tracks: self.node.grid_template_rows.iter(),
            context: self.context,
        })
    }

    #[inline(always)]
    fn grid_template_columns(&self) -> Option<Self::TemplateTrackList<'_>> {
        Some(GridTemplateIter {
            tracks: self.node.grid_template_columns.iter(),
            context: self.context,
        })
    }

    #[inline(always)]
    fn grid_auto_rows(&self) -> Self::AutoTrackList<'_> {
        GridTrackIter {
            tracks: self.node.grid_auto_rows.iter(),
            context: self.context,
        }
    }

    #[inline(always)]
    fn grid_auto_columns(&self) -> Self::AutoTrackList<'_> {
        GridTrackIter {
            tracks: self.node.grid_auto_columns.iter(),
            context: self.context,
        }
    }

    #[inline(always)]
    fn grid_template_areas(&self) -> Option<Self::GridTemplateAreas<'_>> {
        None
    }

    #[inline(always)]
    fn grid_template_column_names(&self) -> Option<Self::TemplateLineNames<'_>> {
        None
    }

    #[inline(always)]
    fn grid_template_row_names(&self) -> Option<Self::TemplateLineNames<'_>> {
        None
    }

    #[inline(always)]
    fn grid_auto_flow(&self) -> taffy::GridAutoFlow {
        self.node.grid_auto_flow.into()
    }

    #[inline(always)]
    fn gap(&self) -> taffy::Size<taffy::LengthPercentage> {
        taffy::Size {
            width: self.node.column_gap.into_length_percentage(&self.context),
            height: self.node.row_gap.into_length_percentage(&self.context),
        }
    }

    #[inline(always)]
    fn align_content(&self) -> Option<taffy::AlignContent> {
        self.node.align_content.into()
    }

    #[inline(always)]
    fn justify_content(&self) -> Option<taffy::JustifyContent> {
        self.node.justify_content.into()
    }

    #[inline(always)]
    fn align_items(&self) -> Option<taffy::AlignItems> {
        self.node.align_items.into()
    }

    #[inline(always)]
    fn justify_items(&self) -> Option<taffy::AlignItems> {
        self.node.justify_items.into()
    }
}

impl<'a> taffy::GridItemStyle for NodeStyle<'a> {
    #[inline(always)]
    fn grid_row(&self) -> taffy::Line<taffy::GridPlacement<String>> {
        self.node.grid_row.into()
    }

    #[inline(always)]
    fn grid_column(&self) -> taffy::Line<taffy::GridPlacement<String>> {
        self.node.grid_column.into()
    }

    #[inline(always)]
    fn align_self(&self) -> Option<taffy::AlignSelf> {
        self.node.align_self.into()
    }

    #[inline(always)]
    fn justify_self(&self) -> Option<taffy::AlignSelf> {
        self.node.justify_self.into()
    }
}

/// Bundles the ECS inputs needed to build a [`NodeStyle`] for UI entities.
#[derive(SystemParam)]
pub struct NodeStyleQuery<'w, 's> {
    rem_size: Res<'w, RemSize>,
    node_query: Query<
        'w,
        's,
        (
            &'static Node,
            &'static ComputedUiRenderTargetInfo,
            &'static EmSize,
        ),
    >,
}

impl<'w, 's> NodeStyleQuery<'w, 's> {
    /// Returns the effective [`NodeStyle`] for `entity`.
    pub fn get(&self, entity: Entity) -> Result<NodeStyle<'_>, QueryEntityError> {
        let (node, target, em_size) = self.node_query.get(entity)?;
        Ok(NodeStyle::new(
            node,
            LayoutContext::new(
                target.scale_factor(),
                target.physical_size().as_vec2(),
                *em_size,
                *self.rem_size,
            ),
        ))
    }

    /// Whether the global [`RemSize`] resource changed.
    pub fn rem_size_changed(&self) -> bool {
        self.rem_size.is_changed()
    }
}

/// An iterator over an empty set of named grid lines
///
/// [`Node`] has no concept of named lines.
#[derive(Clone)]
pub struct EmptyLineNames<'a>(PhantomData<&'a ()>);

impl<'a> EmptyLineNames<'a> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'a> Iterator for EmptyLineNames<'a> {
    type Item = core::iter::Empty<&'a String>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

impl<'a> ExactSizeIterator for EmptyLineNames<'a> {
    fn len(&self) -> usize {
        0
    }
}

impl<'a> taffy::TemplateLineNames<'a, String> for EmptyLineNames<'a> {
    type LineNameSet<'b>
        = core::iter::Empty<&'b String>
    where
        Self: 'b;
}
