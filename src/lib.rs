#![feature(specialization)]
#![feature(proc_macro_hygiene)]
#![feature(stmt_expr_attributes)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_deref;
#[macro_use]
extern crate pi_enum_default_macro;

extern crate paste;
#[macro_use]
extern crate lazy_static;

pub mod components;
pub mod resource;
pub mod system;
pub mod utils;
pub mod gui;
pub mod font;



pub mod prelude {
    // pub use crate::{
    //     system::world_matrix::cal_matrix,
    // };
}




