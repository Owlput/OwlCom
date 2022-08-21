use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Endpoint)]
pub fn endpoint_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_endpoint(&ast)
}

fn impl_endpoint(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        use async_trait::async_trait;
        #[async_trait]
        impl<'a> Endpoint<Response,reqwest::Error> for #name<'a>
        {
            async fn exec(&self)->Result<Response,reqwest::Error>{
                self.client.execute(self.request.try_clone().unwrap()).await?.json::<Response>().await
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(EndpointResponse)]
pub fn endpoint_response_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_endpoint_response(&ast)
}

fn impl_endpoint_response(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl EndpointResponse for #name{}
    };
    gen.into()
}
