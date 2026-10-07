#[hotcode::hotreload]
fn tuple((a, b): (i32, i32)) {
    dbg!(a, b);
}

struct Struct {
    a: i32,
    b: i32,
    #[allow(unused)]
    c: i32,
}
#[hotcode::hotreload]
fn deconstruct(Struct { a, b, .. }: Struct) {
    dbg!(a, b);
}

mod tests {
    #[test]
    fn compile() {}
}
