#[macro_export]
macro_rules! generate_required_params_enum {
    ($(($x:ident:$y:ty)),+) => {
        pub enum RequiredParams{
            $(
                $x($y)
            )*
        }
    };
}
#[macro_export]
macro_rules! generate_optional_params_enum {
    ($($x:tt:$y:ty),+) => {
        pub enum OptionalParams{
            $(
                $x($y),
            )*
        }
        impl ToParam for OptionalParams{
            fn to_param(&self)->String{
                match self{
                    $(
                        OptionalParams::$x(val)=>format!("{}={}",stringify!($x),val),
                    )*
                }
            }
        }
    };
}