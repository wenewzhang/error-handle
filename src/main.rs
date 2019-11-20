// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";
//
//     let result = longest(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

// fn main() {
//     let string1 = String::from("long string is long");
//
//     {
//         let string2 = String::from("xyz");
//         let result = longest(string1.as_str(), string2.as_str());
//         println!("The longest string is {}", result);
//     }
// }

//error happen example
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
//
// error[E0597]: `string2` does not live long enough
//   --> src/main.rs:26:44
//    |
// 26 |         result = longest(string1.as_str(), string2.as_str());
//    |                                            ^^^^^^^ borrowed value does not live long enough
// 27 |     }
//    |     - `string2` dropped here while still borrowed
// 28 |     println!("The longest string is {}", result);
//    |                                          ------ borrow later used here

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
