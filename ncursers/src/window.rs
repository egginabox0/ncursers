

pub enum ConstraintOptions {
    None,
    ClipDescendants,
    NearestValid,
}

pub struct Window {
    parent: Box<Window>,
    child: Box<Window>, //TODO: figure out how to allow Windows to not have children

    height: u16,
    width: u16,

    col: u16,
    lin: u16,

    render_order: u16,
    render_constraint: ConstraintOptions,
} impl Window {
    fn new() {
        
    }
}