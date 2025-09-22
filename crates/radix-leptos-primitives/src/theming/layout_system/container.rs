use serde::{Deserialize, Serialize};

/// Container system configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerSystem {
    pub max_widths: Vec<ContainerMaxWidth>,
    pub paddings: Vec<f64>,
    pub margins: Vec<f64>,
    pub centers: Vec<bool>,
}

impl Default for ContainerSystem {
    fn default() -> Self {
        Self {
            max_widths: [
                ContainerMaxWidth::Small,
                ContainerMaxWidth::Medium,
                ContainerMaxWidth::Large,
                ContainerMaxWidth::ExtraLarge,
                ContainerMaxWidth::Fluid,
            ]
            .to_vec(),
            paddings: [16.0, 24.0, 32.0, 48.0].to_vec(),
            margins: [0.0, 16.0, 24.0, 32.0, 48.0].to_vec(),
            centers: [true, false].to_vec(),
        }
    }
}

/// Container max width enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ContainerMaxWidth {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Fluid,
}

impl ContainerMaxWidth {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContainerMaxWidth::Small => "sm",
            ContainerMaxWidth::Medium => "md",
            ContainerMaxWidth::Large => "lg",
            ContainerMaxWidth::ExtraLarge => "xl",
            ContainerMaxWidth::Fluid => "fluid",
        }
    }

    pub fn max_width(&self) -> Option<f64> {
        match self {
            ContainerMaxWidth::Small => Some(640.0),
            ContainerMaxWidth::Medium => Some(768.0),
            ContainerMaxWidth::Large => Some(1024.0),
            ContainerMaxWidth::ExtraLarge => Some(1280.0),
            ContainerMaxWidth::Fluid => None,
        }
    }
}

/// Grid system configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridSystem {
    pub columns: u32,
    pub gutters: Vec<f64>,
    pub gaps: Vec<f64>,
    pub alignments: Vec<GridAlignment>,
}

impl Default for GridSystem {
    fn default() -> Self {
        Self {
            columns: 12,
            gutters: [16.0, 24.0, 32.0, 48.0].to_vec(),
            gaps: [8.0, 16.0, 24.0, 32.0, 48.0].to_vec(),
            alignments: [
                GridAlignment::Start,
                GridAlignment::Center,
                GridAlignment::End,
                GridAlignment::Stretch,
                GridAlignment::SpaceBetween,
                GridAlignment::SpaceAround,
                GridAlignment::SpaceEvenly,
            ]
            .to_vec(),
        }
    }
}

/// Grid alignment enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GridAlignment {
    Start,
    Center,
    End,
    Stretch,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl GridAlignment {
    pub fn as_str(&self) -> &'static str {
        match self {
            GridAlignment::Start => "start",
            GridAlignment::Center => "center",
            GridAlignment::End => "end",
            GridAlignment::Stretch => "stretch",
            GridAlignment::SpaceBetween => "space-between",
            GridAlignment::SpaceAround => "space-around",
            GridAlignment::SpaceEvenly => "space-evenly",
        }
    }
}

/// Flexbox system configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlexboxSystem {
    pub directions: Vec<FlexDirection>,
    pub wraps: Vec<FlexWrap>,
    pub justifications: Vec<JustifyContent>,
    pub alignments: Vec<AlignItems>,
    pub grows: Vec<f64>,
    pub shrinks: Vec<f64>,
}

impl Default for FlexboxSystem {
    fn default() -> Self {
        Self {
            directions: [
                FlexDirection::Row,
                FlexDirection::RowReverse,
                FlexDirection::Column,
                FlexDirection::ColumnReverse,
            ]
            .to_vec(),
            wraps: [FlexWrap::Nowrap, FlexWrap::Wrap, FlexWrap::WrapReverse].to_vec(),
            justifications: [
                JustifyContent::Start,
                JustifyContent::End,
                JustifyContent::Center,
                JustifyContent::SpaceBetween,
                JustifyContent::SpaceAround,
                JustifyContent::SpaceEvenly,
            ]
            .to_vec(),
            alignments: [
                AlignItems::Start,
                AlignItems::End,
                AlignItems::Center,
                AlignItems::Baseline,
                AlignItems::Stretch,
            ]
            .to_vec(),
            grows: [0.0, 1.0, 2.0, 3.0].to_vec(),
            shrinks: [0.0, 1.0, 2.0, 3.0].to_vec(),
        }
    }
}

/// Flex direction enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl FlexDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            FlexDirection::Row => "row",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::Column => "column",
            FlexDirection::ColumnReverse => "column-reverse",
        }
    }
}

/// Flex wrap enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FlexWrap {
    Nowrap,
    Wrap,
    WrapReverse,
}

impl FlexWrap {
    pub fn as_str(&self) -> &'static str {
        match self {
            FlexWrap::Nowrap => "nowrap",
            FlexWrap::Wrap => "wrap",
            FlexWrap::WrapReverse => "wrap-reverse",
        }
    }
}

/// Justify content enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl JustifyContent {
    pub fn as_str(&self) -> &'static str {
        match self {
            JustifyContent::Start => "start",
            JustifyContent::End => "end",
            JustifyContent::Center => "center",
            JustifyContent::SpaceBetween => "space-between",
            JustifyContent::SpaceAround => "space-around",
            JustifyContent::SpaceEvenly => "space-evenly",
        }
    }
}

/// Align items enum
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl AlignItems {
    pub fn as_str(&self) -> &'static str {
        match self {
            AlignItems::Start => "start",
            AlignItems::End => "end",
            AlignItems::Center => "center",
            AlignItems::Baseline => "baseline",
            AlignItems::Stretch => "stretch",
        }
    }
}

#[cfg(test)]
mod container_tests {
    use super::*;

    #[test]
    fn test_container_system_default() {
        let containers = ContainerSystem::default();
        assert!(containers.max_widths.contains(&ContainerMaxWidth::Small));
        assert!(containers.max_widths.contains(&ContainerMaxWidth::Medium));
        assert!(containers.max_widths.contains(&ContainerMaxWidth::Large));
        assert!(containers
            .max_widths
            .contains(&ContainerMaxWidth::ExtraLarge));
        assert!(containers.max_widths.contains(&ContainerMaxWidth::Fluid));
        assert!(containers.paddings.contains(&16.0));
        assert!(containers.paddings.contains(&24.0));
        assert!(containers.paddings.contains(&32.0));
        assert!(containers.paddings.contains(&48.0));
        assert!(containers.margins.contains(&0.0));
        assert!(containers.margins.contains(&16.0));
        assert!(containers.margins.contains(&24.0));
        assert!(containers.margins.contains(&32.0));
        assert!(containers.margins.contains(&48.0));
        assert!(containers.centers.contains(&true));
        assert!(containers.centers.contains(&false));
    }

    #[test]
    fn test_container_max_width_enum() {
        assert_eq!(ContainerMaxWidth::Small.as_str(), "sm");
        assert_eq!(ContainerMaxWidth::Medium.as_str(), "md");
        assert_eq!(ContainerMaxWidth::Large.as_str(), "lg");
        assert_eq!(ContainerMaxWidth::ExtraLarge.as_str(), "xl");
        assert_eq!(ContainerMaxWidth::Fluid.as_str(), "fluid");
    }

    #[test]
    fn test_container_max_width_values() {
        assert_eq!(ContainerMaxWidth::Small.max_width(), Some(640.0));
        assert_eq!(ContainerMaxWidth::Medium.max_width(), Some(768.0));
        assert_eq!(ContainerMaxWidth::Large.max_width(), Some(1024.0));
        assert_eq!(ContainerMaxWidth::ExtraLarge.max_width(), Some(1280.0));
        assert_eq!(ContainerMaxWidth::Fluid.max_width(), None);
    }

    #[test]
    fn test_grid_system_default() {
        let grid = GridSystem::default();
        assert_eq!(grid.columns, 12);
        assert!(grid.gutters.contains(&16.0));
        assert!(grid.gutters.contains(&24.0));
        assert!(grid.gutters.contains(&32.0));
        assert!(grid.gutters.contains(&48.0));
        assert!(grid.gaps.contains(&8.0));
        assert!(grid.gaps.contains(&16.0));
        assert!(grid.gaps.contains(&24.0));
        assert!(grid.gaps.contains(&32.0));
        assert!(grid.gaps.contains(&48.0));
        assert!(grid.alignments.contains(&GridAlignment::Start));
        assert!(grid.alignments.contains(&GridAlignment::Center));
        assert!(grid.alignments.contains(&GridAlignment::End));
        assert!(grid.alignments.contains(&GridAlignment::Stretch));
    }

    #[test]
    fn test_grid_alignment_enum() {
        assert_eq!(GridAlignment::Start.as_str(), "start");
        assert_eq!(GridAlignment::Center.as_str(), "center");
        assert_eq!(GridAlignment::End.as_str(), "end");
        assert_eq!(GridAlignment::Stretch.as_str(), "stretch");
        assert_eq!(GridAlignment::SpaceBetween.as_str(), "space-between");
        assert_eq!(GridAlignment::SpaceAround.as_str(), "space-around");
        assert_eq!(GridAlignment::SpaceEvenly.as_str(), "space-evenly");
    }

    #[test]
    fn test_flexbox_system_default() {
        let flexbox = FlexboxSystem::default();
        assert!(flexbox.directions.contains(&FlexDirection::Row));
        assert!(flexbox.directions.contains(&FlexDirection::RowReverse));
        assert!(flexbox.directions.contains(&FlexDirection::Column));
        assert!(flexbox.directions.contains(&FlexDirection::ColumnReverse));
        assert!(flexbox.wraps.contains(&FlexWrap::Nowrap));
        assert!(flexbox.wraps.contains(&FlexWrap::Wrap));
        assert!(flexbox.wraps.contains(&FlexWrap::WrapReverse));
        assert!(flexbox.justifications.contains(&JustifyContent::Start));
        assert!(flexbox.justifications.contains(&JustifyContent::End));
        assert!(flexbox.justifications.contains(&JustifyContent::Center));
        assert!(flexbox.alignments.contains(&AlignItems::Start));
        assert!(flexbox.alignments.contains(&AlignItems::End));
        assert!(flexbox.alignments.contains(&AlignItems::Center));
        assert!(flexbox.grows.contains(&0.0));
        assert!(flexbox.grows.contains(&1.0));
        assert!(flexbox.shrinks.contains(&0.0));
        assert!(flexbox.shrinks.contains(&1.0));
    }

    #[test]
    fn test_flex_direction_enum() {
        assert_eq!(FlexDirection::Row.as_str(), "row");
        assert_eq!(FlexDirection::RowReverse.as_str(), "row-reverse");
        assert_eq!(FlexDirection::Column.as_str(), "column");
        assert_eq!(FlexDirection::ColumnReverse.as_str(), "column-reverse");
    }

    #[test]
    fn test_flex_wrap_enum() {
        assert_eq!(FlexWrap::Nowrap.as_str(), "nowrap");
        assert_eq!(FlexWrap::Wrap.as_str(), "wrap");
        assert_eq!(FlexWrap::WrapReverse.as_str(), "wrap-reverse");
    }

    #[test]
    fn test_justify_content_enum() {
        assert_eq!(JustifyContent::Start.as_str(), "start");
        assert_eq!(JustifyContent::End.as_str(), "end");
        assert_eq!(JustifyContent::Center.as_str(), "center");
        assert_eq!(JustifyContent::SpaceBetween.as_str(), "space-between");
        assert_eq!(JustifyContent::SpaceAround.as_str(), "space-around");
        assert_eq!(JustifyContent::SpaceEvenly.as_str(), "space-evenly");
    }

    #[test]
    fn test_align_items_enum() {
        assert_eq!(AlignItems::Start.as_str(), "start");
        assert_eq!(AlignItems::End.as_str(), "end");
        assert_eq!(AlignItems::Center.as_str(), "center");
        assert_eq!(AlignItems::Baseline.as_str(), "baseline");
        assert_eq!(AlignItems::Stretch.as_str(), "stretch");
    }
}
