#[macro_export]
macro_rules! impl_opt_param {
    ($($(#[$meta:meta])*$name:ident:$type:ty),+) => {
        impl Builder{
            $(
            $(#[$meta])*
            pub fn $name(self, arg: $type) -> Self {
                match self.opt_params {
                    None=>Self {
                        opt_params: Some(format!("{}={}",stringify!($name), arg.to_string())),
                    },
                    Some(v)=> Self {
                        opt_params: Some(format!("{}&{}={}", v,stringify!($name) ,arg.to_string())),
                    }
                }
            })*
        }
    };
}

#[macro_export]
macro_rules! endpoint_gen {
    ($(#[$meta:meta])*$endpoint:ident) => {
        use reqwest::{Client, Request};
        $(#[$meta])*
        #[derive(Debug)]
        pub struct $endpoint<'a> {
            client: &'a Client,
            request: Request,
        }
        impl<'a> $endpoint<'a>{
            /// Return the `Builder` of this endpoint.
            pub fn builder()->Builder{
                Builder::default()
            }
        }
    };
}
