use leftwm_core::Command;

pub trait WaylandConfig {
    fn transparancy_rules(&self) -> Vec<TransparancyRule>;

    // Other composting rules

    fn locking_clients(&self) -> Vec<String>;

    fn gestures(&self) -> Vec<Gesture>; //NOTE: Maybe
}

pub struct TransparancyRule {
    title: String,
    transparancy: usize,
    focused_transparancy: usize,
    floating_transparancy: usize,
}

pub enum Gesture {
    Swipe {
        fingers: u32,
        direction: Direction,
        command: Command,
    },
    Pinch {
        fingers: u32,
        action: PinchAction,
        command: Command,
    },
    Hold {
        fingers: u32,
        length: u32,
        command: Command,
    },
}

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub enum PinchAction {
    ScaleDown,
    ScaleUp,
    RotateLeft,
    RotateRight,
}
