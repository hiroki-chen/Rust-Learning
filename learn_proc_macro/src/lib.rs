use my_proc_macros::{attribute_play, Reflection};

#[derive(Reflection, Default)]
struct Foo {
    foo: u32,
    bar: String,
    baz: String,
    harry_potter: Vec<u8>,
    albus_dumbledore: Option<String>,
}

struct Bar {}

#[attribute_play(arg1, arg2)]
fn disappear() {
    println!("ok");
}

#[cfg(test)]
mod test {
    use super::*;

    /// Test reflection at compilation time using procedural macro.
    #[test]
    fn reflection_works() {
        let foo = Foo::default();
        foo.traverse_fields();
    }

    #[test]
    fn attribute_works() {
        // The function signature is changed.
        disappear(1, 2);
    }
}
