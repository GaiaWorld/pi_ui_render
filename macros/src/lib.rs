extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Data, DataStruct, DeriveInput, Field, Fields, GenericParam, Ident, Index, LitInt, Result,
    Token,
};

#[proc_macro_derive(Style, attributes(component))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);
	let value_ty = get_style_ty(&ast);
	
    

    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    TokenStream::from(quote! {
		impl #impl_generics pi_ui_render::utils::style::style_sheet::Style for #struct_name #type_generics #where_clause{
			fn get_type(&self) -> StyleType {
				todo!();
			}
			unsafe fn write(&self, ptr: *mut u8, buffer: &mut Vec<u8>) {
				todo!();
			}
			/// 安全： entity必须存在
			fn set(&self, buffer: &Vec<u8>, offset: usize, query: &mut StyleQuery, entity: Entity){
				todo!();
			}
			/// 安全： entity必须存在
			fn reset(&self, cur_style_mark: BitArray<[u32;3]>, query: &mut StyleQuery, entity: Entity){
				todo!();
			}
		}
    })
}

fn get_style_ty(ast: &DeriveInput) -> &Field {
	if let Data::Struct(s) = &ast.data {
		if let Fields::Unnamed(unnamed) = &s.fields {
			if unnamed.unnamed.len() == 0 {
				return &unnamed.unnamed[0] 
			}
		}
	}
	panic!("impl Style fail, struct must Tuples, and only one element, but cur struct is:{:?}", quote! {ast});
}

