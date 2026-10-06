#[hotcode::hotreload]
pub fn standalone_get() -> i32 {
    10
}

pub struct Struct;

impl Struct {
    #[hotcode::hotreload]
    pub fn get(&self) -> i32 {
        20
    }
    #[hotcode::hotreload]
    pub fn mut_get(&mut self) -> i32 {
        30
    }
    #[hotcode::hotreload]
    pub fn consume_get(self) -> i32 {
        40
    }
    #[hotcode::hotreload]
    pub fn box_get(self: Box<Self>) -> i32 {
        50
    }
    #[hotcode::hotreload]
    pub fn static_get() -> i32 {
        60
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
        70
    }
    #[hotcode::hotreload]
    fn trait_mut_get(&mut self) -> i32 {
        80
    }
    #[hotcode::hotreload]
    fn trait_consume_get(self) -> i32 {
        90
    }
    #[hotcode::hotreload]
    fn trait_box_get(self: Box<Self>) -> i32 {
        100
    }
    #[hotcode::hotreload]
    fn trait_static_get() -> i32 {
        110
    }
}

pub enum Enum {
    Variant,
}

impl Enum {
    #[hotcode::hotreload]
    pub fn get(&self) -> i32 {
        120
    }
    #[hotcode::hotreload]
    pub fn mut_get(&mut self) -> i32 {
        130
    }
    #[hotcode::hotreload]
    pub fn consume_get(self) -> i32 {
        140
    }
    #[hotcode::hotreload]
    pub fn box_get(self: Box<Self>) -> i32 {
        150
    }
    #[hotcode::hotreload]
    pub fn static_get() -> i32 {
        160
    }
}

impl Trait for Enum {
    #[hotcode::hotreload]
    fn trait_get(&self) -> i32 {
        170
    }
    #[hotcode::hotreload]
    fn trait_mut_get(&mut self) -> i32 {
        180
    }
    #[hotcode::hotreload]
    fn trait_consume_get(self) -> i32 {
        190
    }
    #[hotcode::hotreload]
    fn trait_box_get(self: Box<Self>) -> i32 {
        200
    }
    #[hotcode::hotreload]
    fn trait_static_get() -> i32 {
        210
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
        220
    }
    #[hotcode::hotreload]
    pub fn mut_get(&mut self) -> i32 {
        230
    }
    #[hotcode::hotreload]
    pub fn consume_get(self) -> i32 {
        240
    }
    #[hotcode::hotreload]
    pub fn box_get(self: Box<Self>) -> i32 {
        250
    }
    #[hotcode::hotreload]
    pub fn static_get() -> i32 {
        260
    }
}

impl Trait for Union {
    #[hotcode::hotreload]
    fn trait_get(&self) -> i32 {
        270
    }
    #[hotcode::hotreload]
    fn trait_mut_get(&mut self) -> i32 {
        280
    }
    #[hotcode::hotreload]
    fn trait_consume_get(self) -> i32 {
        290
    }
    #[hotcode::hotreload]
    fn trait_box_get(self: Box<Self>) -> i32 {
        300
    }
    #[hotcode::hotreload]
    fn trait_static_get() -> i32 {
        310
    }
}
