extern crate regex_syntax;

// struct Foo;
//
// impl PartialEq<Foo> for u8 {
//     fn eq(&self, other: &Foo) -> bool {
//         false
//     }
// }

fn main() {
    let v: Vec<u8> = Vec::new();

    // error[E0282]: unable to infer enough type information about `T`
    //   --> src/main.rs:13:18
    //    |
    // 13 |     let b = v == Vec::new();
    //    |                  ^^^^^^^^ cannot infer type for `T`
    //    |
    //    = note: type annotations or generic parameter binding required

    let b = v == Vec::new();
}
