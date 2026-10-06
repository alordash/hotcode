#[hotcode::hotreload]
pub fn standalone_get() -> i32 {
    1
}

pub struct Struct;

impl Struct {
    #[hotcode::hotreload]
    pub fn get(&self) -> i32 {
        2
    }
    #[hotcode::hotreload]
    pub fn mut_get(&mut self) -> i32 {
        3
    }
    #[hotcode::hotreload]
    pub fn consume_get(self) -> i32 {
        4
    }
    #[hotcode::hotreload]
    pub fn box_get(self: Box<Self>) -> i32 {
        5
    }
    #[hotcode::hotreload]
    pub fn static_get() -> i32 {
        6
    }
}

pub trait Trait {
    fn trait_get(&self) -> i32;
    fn trait_mut_get(&mut self) -> i32;
    fn trait_consume_get(self) -> i32;
    fn trait_box_get(self: Box<Self>) -> i32;
    fn trait_static_get() -> i32;
}

impl Trait for Struct {
    #[hotcode::hotreload]
    fn trait_get(&self) -> i32 {
        7
    }
    #[hotcode::hotreload]
    fn trait_mut_get(&mut self) -> i32 {
        8
    }
    #[hotcode::hotreload]
    fn trait_consume_get(self) -> i32 {
        9
    }
    #[hotcode::hotreload]
    fn trait_box_get(self: Box<Self>) -> i32 {
        10
    }
    #[hotcode::hotreload]
    fn trait_static_get() -> i32 {
        11
    }
}

pub enum Enum {
    Variant,
}

impl Enum {
    #[hotcode::hotreload]
    pub fn get(&self) -> i32 {
        12
    }
    #[hotcode::hotreload]
    pub fn mut_get(&mut self) -> i32 {
        13
    }
    #[hotcode::hotreload]
    pub fn consume_get(self) -> i32 {
        14
    }
    #[hotcode::hotreload]
    pub fn box_get(self: Box<Self>) -> i32 {
        15
    }
    #[hotcode::hotreload]
    pub fn static_get() -> i32 {
        16
    }
}

impl Trait for Enum {
    #[hotcode::hotreload]
    fn trait_get(&self) -> i32 {
        17
    }
    #[hotcode::hotreload]
    fn trait_mut_get(&mut self) -> i32 {
        18
    }
    #[hotcode::hotreload]
    fn trait_consume_get(self) -> i32 {
        19
    }
    #[hotcode::hotreload]
    fn trait_box_get(self: Box<Self>) -> i32 {
        20
    }
    #[hotcode::hotreload]
    fn trait_static_get() -> i32 {
        21
    }
}

pub union Union {
    _unused: bool,
}

impl Union {
    pub fn new() -> Self {
        Self { _unused: false }
    }
}

impl Union {
    #[hotcode::hotreload]
    pub fn get(&self) -> i32 {
        22
    }
    #[hotcode::hotreload]
    pub fn mut_get(&mut self) -> i32 {
        23
    }
    #[hotcode::hotreload]
    pub fn consume_get(self) -> i32 {
        24
    }
    #[hotcode::hotreload]
    pub fn box_get(self: Box<Self>) -> i32 {
        25
    }
    #[hotcode::hotreload]
    pub fn static_get() -> i32 {
        26
    }
}

impl Trait for Union {
    #[hotcode::hotreload]
    fn trait_get(&self) -> i32 {
        27
    }
    #[hotcode::hotreload]
    fn trait_mut_get(&mut self) -> i32 {
        28
    }
    #[hotcode::hotreload]
    fn trait_consume_get(self) -> i32 {
        29
    }
    #[hotcode::hotreload]
    fn trait_box_get(self: Box<Self>) -> i32 {
        30
    }
    #[hotcode::hotreload]
    fn trait_static_get() -> i32 {
        31
    }
}
