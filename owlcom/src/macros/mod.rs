#[macro_export]
macro_rules! impl_opt_param {
    ($($(#[$meta:meta])*$name:ident:$type:ty),+) => {
        impl Builder{
            /// Overwrite the current optional params.
            pub fn set_opt_param(mut self, param: String) -> Self {
                self.opt_params = Some(param);
                self
            }
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

#[macro_export]
macro_rules! simple_builder_impl {
    ($endpoint:ident:$path:expr) => {
        #[derive(Debug, Default)]
        pub struct Builder;
        impl<'a> Builder {
            /// Return the `Builder` of this endpoint.
            pub fn build(self, client: &'a Client, host: &String) -> $endpoint<'a> {
                $endpoint {
                    client,
                    request: client.post(host.clone() + $path).build().unwrap(),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! builder_impl_with_opt_params {
    ($endpoint:ident:$path:expr,$($(#[$meta:meta])*$name:ident:$type:ty),+) => {
        #[derive(Debug, Default)]
        pub struct Builder{
            opt_params:Option<String>
        }
        impl<'a> Builder {
            pub fn new()->Self{
                Self::default()
            }
            /// Return the `Builder` of this endpoint.
            pub fn build(self, client: &'a Client, host: &String) -> $endpoint<'a> {
                $endpoint {
                    client,
                    request: client.post(format!("{}{}{}",host,$path,self.opt_params.unwrap_or("".into()))).build().unwrap(),
                }
            }
            /// Overwrite the current optional params.
            pub fn set_opt_param(mut self, param: String) -> Self {
                self.opt_params = Some(param);
                self
            }
            $(
            $(#[$meta])*
            pub fn $name(self, arg: $type) -> Self {
                match self.opt_params {
                    None=>Self {
                        opt_params: Some(format!("?{}={}",stringify!($name), arg.to_string())),
                    },
                    Some(v)=> Self {
                        opt_params: Some(format!("{}&{}={}", v,stringify!($name) ,arg.to_string())),
                    }
                }
            })*
        }
    };
}