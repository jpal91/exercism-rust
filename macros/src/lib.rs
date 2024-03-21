#[macro_export]
macro_rules! hashmap {

    ( @build [ $($any:tt)* ]; $key:expr => $value:expr, $( $rest:tt )* ) => {
        $crate::hashmap!( @build [ $($any)* ($key, $value) ]; $($rest)*)
    };

    ( @build [ $($any:tt)* ]; $key:expr => $value:expr ) => {
        $crate::hashmap!( @build [ $($any)* ($key, $value) ]; )
    };

    ( @build [ $(($key:expr, $value:expr))* ];) => {
        {
            let mut hash = ::std::collections::HashMap::new();
            
            $(
                hash.insert($key, $value);
            )*

            hash
        }

    };

    ( $( $any:tt )* ) => {
        $crate::hashmap!(@build []; $($any)*)
    };
}
