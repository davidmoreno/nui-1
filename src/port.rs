#[derive(Debug, Copy, Clone)]
pub struct Port{
    pub nr: usize
}

impl Port{
    pub fn new(nr: usize) -> Port{
        Port{nr:nr}
    }
}
