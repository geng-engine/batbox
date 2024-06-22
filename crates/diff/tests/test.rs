use batbox_diff::*;

#[derive(Diff)]
struct Inner {
    a: i32,
}

#[derive(Diff)]
struct Foo {
    a: i32,
    #[diff(mode = "eq")]
    b: Vec<i32>,
    #[diff(mode = "clone")]
    c: String,
    d: Inner,
}

#[test]
fn main() {
    let a = Foo {
        a: 123,
        b: vec![1, 2, 3],
        c: "hello".to_owned(),
        d: Inner { a: 0 },
    };
    let b = Foo {
        a: 124,
        b: vec![1, 2, 3],
        c: "world".to_owned(),
        d: Inner { a: 0 },
    };
    let diff = a.diff(&b);
    assert!(diff.b.is_none());
}
