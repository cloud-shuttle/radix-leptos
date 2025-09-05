#[cfg(test)]
mod tests {
    use super::*;
    use crate::theming::{LayoutSystem, GridSystem, FlexboxSystem, ContainerSystem, SpacingSystem, BreakpointSystem};

    #[test]
    fn test_layout_system_creation() {
        let layout = LayoutSystem::default();
        assert_eq!(layout.spacing.base_unit, 4.0);
        assert_eq!(layout.spacing.scale.len(), 24); // 24 scale values
        assert_eq!(layout.spacing.directions.len(), 7); // All, Horizontal, Vertical, Top, Right, Bottom, Left
    }

    #[test]
    fn test_spacing_system_creation() {
        let spacing = SpacingSystem::default();
        assert_eq!(spacing.base_unit, 4.0);
        assert_eq!(spacing.scale.len(), 24);
        assert_eq!(spacing.directions.len(), 7);
    }

    #[test]
    fn test_breakpoint_system_creation() {
        let breakpoints = BreakpointSystem::default();
        assert_eq!(breakpoints.breakpoints.len(), 6); // ExtraSmall, Small, Medium, Large, ExtraLarge, ExtraExtraLarge
        assert_eq!(breakpoints.container_max_widths.len(), 5); // Small, Medium, Large, ExtraLarge, Fluid
    }

    #[test]
    fn test_grid_system_creation() {
        let grid = GridSystem::default();
        assert_eq!(grid.columns, 12);
        assert_eq!(grid.gutters.len(), 4); // 4 gutter values
        assert_eq!(grid.gaps.len(), 5); // 5 gap values
        assert_eq!(grid.alignments.len(), 4); // Start, Center, End, Stretch
    }

    #[test]
    fn test_flexbox_system_creation() {
        let flexbox = FlexboxSystem::default();
        assert_eq!(flexbox.directions.len(), 2); // Row, Column
        assert_eq!(flexbox.alignments.len(), 4); // Start, Center, End, Stretch
        assert_eq!(flexbox.justifications.len(), 5); // Start, Center, End, Between, Around
        assert_eq!(flexbox.wraps.len(), 3); // NoWrap, Wrap, WrapReverse
    }

    #[test]
    fn test_container_system_creation() {
        let container = ContainerSystem::default();
        assert_eq!(container.max_widths.len(), 5); // Small, Medium, Large, ExtraLarge, Fluid
        assert_eq!(container.paddings.len(), 4); // 4 padding values
        assert_eq!(container.margins.len(), 4); // 4 margin values
    }

    #[test]
    fn test_layout_system_default() {
        let layout = LayoutSystem::default();
        assert_eq!(layout.spacing.scale.len(), 24);
        assert_eq!(layout.breakpoints.breakpoints.len(), 6);
        assert_eq!(layout.grid.columns, 12);
        assert_eq!(layout.flexbox.directions.len(), 2);
        assert_eq!(layout.containers.max_widths.len(), 5);
    }
}
