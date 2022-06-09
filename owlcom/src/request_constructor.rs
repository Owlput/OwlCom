use hyper::http::request;

use crate::traits::url::ToParam;

pub fn construct_params<T>(params: &Option<Vec<T>>) -> String
where
    T: ToParam,
{
    match params{
        Some(params) => {let len = params.len();
                let mut iter = params.clone().into_iter();
    let mut param_string = String::new();
    for i in 1..len + 1 {
        if i == 1 {
            param_string = format!("{}", iter.next().unwrap().to_param());
        } else {
            param_string = format!("{}&{}", param_string, iter.next().unwrap().to_param());
        }
    }
    param_string
        },
        None => {
            "".to_string()
        },
    }
}

fn construct_headers<T>(builder:request::Builder,headers:Vec<T>)->request::Builder{
    todo!()
}