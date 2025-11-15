use super::*;
use std::{marker::PhantomData, ops::RangeInclusive};

use egui::Rect;
#[cfg(feature = "persistence")]
use serde::{Deserialize, Serialize};

#[derive(Clone)]
#[cfg_attr(feature = "persistence", derive(Serialize, Deserialize))]
pub struct GraphEditorState<NodeData, DataType, ValueType, NodeTemplate, UserState> {
    pub graph: Graph<NodeData, DataType, ValueType>,
    /// Nodes are drawn in this order. Draw order is important because nodes
    /// that are drawn last are on top.
    pub node_order: Vec<NodeId>,
    /// An ongoing connection interaction: The mouse has dragged away from a
    /// port and the user is holding the click
    pub connection_in_progress: Option<(NodeId, AnyParameterId)>,
    /// The currently selected node. Some interface actions depend on the
    /// currently selected node.
    pub selected_nodes: Vec<NodeId>,
    /// The mouse drag start position for an ongoing box selection.
    pub ongoing_box_selection: Option<egui::Pos2>,
    /// The position of each node.
    pub node_positions: SecondaryMap<NodeId, egui::Pos2>,
    /// The node finder is used to create new nodes.
    pub node_finder: Option<NodeFinder<NodeTemplate>>,
    /// Internal Rect for Scene state
    pub scene_rect: egui::Rect,
    /// Allowed Zoom Range
    pub zoom_range: RangeInclusive<f32>,
    pub _user_state: PhantomData<fn() -> UserState>,
}

impl<NodeData, DataType, ValueType, NodeKind, UserState> Default
    for GraphEditorState<NodeData, DataType, ValueType, NodeKind, UserState>
{
    fn default() -> Self {
        Self {
            graph: Default::default(),
            node_order: Default::default(),
            connection_in_progress: Default::default(),
            selected_nodes: Default::default(),
            ongoing_box_selection: Default::default(),
            node_positions: Default::default(),
            node_finder: Default::default(),
            scene_rect: Rect::ZERO,
            zoom_range: 0.2..=2.0,
            _user_state: Default::default(),
        }
    }
}

impl<NodeData, DataType, ValueType, NodeKind, UserState>
    GraphEditorState<NodeData, DataType, ValueType, NodeKind, UserState>
{
    pub fn with_zoom_range(zoom_range: RangeInclusive<f32>) -> Self {
        assert!(0f32.lt(zoom_range.start()), "Range minimum must be greater than zero");
        Self { zoom_range, ..Default::default() }
    }
}