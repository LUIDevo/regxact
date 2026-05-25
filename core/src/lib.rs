mod builder;
mod test;
mod parser;
mod error;
mod analysis;
mod rx;
mod regex_tree;
mod allow;
use wasm_bindgen::prelude::*;

#[macro_export]
macro_rules! rx{
    ($pattern:expr)=>{
        crate::builder::RegxactBuilder::new($pattern).build()
    };
    ($pattern:expr, $($key: ident= $val:expr),*)=>{{
        let mut builder=crate::builder::RegxactBuilder::new($pattern);
        $(
            builder=match stringify!($key){
                "allow"=>builder.allow($val),
                "use"=>builder.contract($val),
                _=>builder,
            };
        )*
        builder.build()
    }}
}
