fn main() {
    // let without_comma = macros::hashmap!(23 => 623, 34 => 21,);
    let _with_trailing = macros::hashmap!(23 => 623, 34 => 21,);
}

// ( $($key:expr => $value:expr $(,)?)* ) => {
//     ::std::collections::HashMap::from_iter([
//         $(
//             ($key, $value),
//         )*
//     ])
// };

// ( @build [ $($any:tt)* ];) => {
//     ::std::collections::HashMap::from_iter([
//         $(
//             $any,
//         )*
//     ])
// };

// ( @build [ $(($key:expr, $value:expr))* ];) => {
//     {
//         let mut hash = ::std::collections::HashMap::new();
//         $(
//             hash.insert($key, $value);
//         )*

//         hash
//     }

// };