#[cfg(not(procmacro2_semver_exempt))]
compile_error!("required: `cfg(procmacro2_semver_exempt)`");

use proc_macro as pm;
use proc_macro2 as pm2;
use quote::quote;

#[proc_macro_attribute]
pub fn my_macro(attr: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    my_macro2(attr.into(), item.into()).into()
}

fn my_macro2(attr: pm2::TokenStream, item: pm2::TokenStream) -> pm2::TokenStream {
    require_real_paths(attr);
    require_real_paths(item.clone());

    quote!(#[cfg(test)] #[test] #item)
}

fn require_real_paths(stream: pm2::TokenStream) {
    for tree in stream {
        let source = tree.span().source_file();
        assert!(source.is_real(), "{:#?}", source);
    }
}
