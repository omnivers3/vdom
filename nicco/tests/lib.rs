extern crate nicco;

use nicco::vdom;

vdom! {
    html 
}

#[test]
fn test() {
    let temp = blah();

    println!("Blah: {:?}", temp);
}

//     body [][
//         div [
//             onclick(Model.foo)
//         ][
//             text("Hiya"),
//             text(var),
//             text("and stuff"),
//         ]
//     ]
// ]