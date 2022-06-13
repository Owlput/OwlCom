use crate::traits::url::ToParam;

pub fn construct_params<T>(params: &Option<Vec<T>>) -> String
where
    T: ToParam,
{
    if let Some(params) = params {
        let mut param_string = String::new();
        for param in params {
                param_string += &param.to_param();
                param_string.push_str("&")
        }
        param_string
    } else {
        "".into()
    }
}
