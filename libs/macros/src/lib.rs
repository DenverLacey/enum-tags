use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, DeriveInput};

/// Procedural macro that generates the companion tag-enum and implements [`TaggedEnum`] trait for the given enum
/// 
/// # Example
/// ```rust
/// use enum_tags::*;
/// #[derive(Tag)]
/// enum MyEnum {
/// 	A,
/// 	B,
/// 	C,
/// }
/// ```
#[proc_macro_derive(Tag)]
pub fn enum_tag(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let tokens = generate_tokens(input);
	tokens.into()
}

fn generate_tokens(input: DeriveInput) -> quote::__private::TokenStream {
	let input_ident = input.ident;
	let input_vis = input.vis;
	let input_generics = input.generics;
	let input_where_clause = input_generics.where_clause.clone();
	let data = if let syn::Data::Enum(data) = input.data {
		data
	} else {
		return quote_spanned! {
			get_decl_keyword_span(input.data) =>
			::std::compile_error!("Can only derive `Tag` for enums.");
		};
	};

	let ident = format_ident!("{}Tag", input_ident);
	let variants: Vec<_> = data.variants.into_iter().collect();
	let fieldless_variants: Vec<_> = variants.iter().map(to_fieldless_variant).collect();
	let ident_variants: Vec<_> = variants.iter().map(to_ident_variant).collect();
	let arm_variants: Vec<_> = variants.iter().map(to_arm_variant).collect();

	let stream = quote! {
		#[derive(Clone, Copy, Debug, PartialEq, Eq)]
		#input_vis enum #ident {
			#(#fieldless_variants),*
		}

		impl #input_generics ::enum_tags_traits::TaggedEnum for #input_ident #input_generics 
			#input_where_clause
		{
			type Tag = #ident;

			fn tag(&self) -> Self::Tag {
				match *self {
					#(Self :: #arm_variants => Self::Tag :: #ident_variants),*
				}
			}
		}
	};

	stream
}

fn get_decl_keyword_span(data: syn::Data) -> quote::__private::Span {
	match data {
		syn::Data::Struct(decl) => decl.struct_token.span,
		syn::Data::Union(decl) => decl.union_token.span,
		syn::Data::Enum(_) => unreachable!(),
	}
}

fn to_fieldless_variant(var: &syn::Variant) -> syn::Variant {
	syn::Variant {
		attrs: var.attrs.clone(),
		ident: var.ident.clone(),
		fields: syn::Fields::Unit,
		discriminant: var.discriminant.clone(),
	}
}

fn to_ident_variant(var: &syn::Variant) -> syn::Variant {
	syn::Variant {
		attrs: var.attrs.clone(),
		ident: var.ident.clone(),
		fields: syn::Fields::Unit,
		discriminant: None,
	}
}

fn to_arm_variant(var: &syn::Variant) -> quote::__private::TokenStream {
	match &var.fields {
		syn::Fields::Named(fields) => {
			let ident = &var.ident;
			let names = fields.named.iter().map(|f| if let Some(ident) = &f.ident { ident } else { unreachable!() });
			quote! { #ident { #(#names : _),* } }
		}
		syn::Fields::Unnamed(fields) => {
			let ident = &var.ident;
			let underscores = fields.unnamed.iter().map(|_| quote! { _ });
			quote! { #ident (#(#underscores),*) }
		}
		syn::Fields::Unit => {
			let ident = &var.ident;
			quote!{ #ident }
		}
	}
}
