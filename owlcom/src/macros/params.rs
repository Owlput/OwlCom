#[macro_export]
macro_rules! generate_optional_params_enum {
    ($($x:tt:$y:ty),+) => {
        use crate::traits::url::ToParam;
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

#[macro_export]
macro_rules! impl_opt_param {
    ($($name:tt::$type:ty),+) => {
        impl Builder {
           $( pub fn $name(self, arg: $type) -> Self {
                if self.optional_params == String::new() {
                    Self {
                        optional_params: format!("{}={}",$name, arg.to_string()),
                    }
                } else {
                    Self {
                        optional_params: format!("{}&{}={}", self.optional_params,$name ,arg.to_string()),
                    }
                }
            } )*
        }
    };
}
