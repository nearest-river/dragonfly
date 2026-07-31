
use quote::quote;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use syn::{
  Data,
  Index,
  Fields,
  DataEnum,
  DataStruct,
  DeriveInput,
};



#[proc_macro_derive(StableHash)]
pub fn stable_hash_derive(item: TokenStream)-> TokenStream {
  let item=syn::parse::<DeriveInput>(item).unwrap();

  let impl_fn=match item.data {
    Data::Enum(item)=> _stable_hash_derive_enum(item),
    Data::Struct(item)=> _stable_hash_derive_struct(item),
    Data::Union(_)=> panic!("this trait cannot be derived for unions.")
  };

  let name=&item.ident;
  let (impl_generics,ty_generics,where_clause)=item.generics.split_for_impl();

  let token_stream=quote! {
    impl #impl_generics ::dragonfly_stable_hash::StableHash for #name #ty_generics
      #where_clause
    {
      #impl_fn
    }
  };

  token_stream.into()
}


fn _stable_hash_derive_struct(item: DataStruct)-> TokenStream2 {
  let stmts=match &item.fields {
    Fields::Unit=> todo!(),
    Fields::Named(fields)=> {
      fields.named
      .iter()
      .map(|field| {
        let ident=field.ident.as_ref().unwrap();

        quote! {
          self.#ident.stable_hash(hcx,hasher);
        }
      })
      .collect::<Vec<_>>()
    },
    Fields::Unnamed(fields)=> {
      fields.unnamed
      .iter()
      .enumerate()
      .map(|(i,_)| {
        let idx=Index::from(i);

        quote! {
          self.#idx.stable_hash(hcx,hasher);
        }
      })
      .collect::<Vec<_>>()
    },
  };



  quote! {
    #[inline]
    fn stable_hash<Hcx: dragonfly_stable_hash::StableHashCtxt>(
      &self,
      hcx: &mut Hcx,
      hasher: &mut dragonfly_stable_hash::StableHasher
    ) {
      #(#stmts)*
    }
  }
}

fn _stable_hash_derive_enum(item: DataEnum)-> TokenStream2 {
  todo!()
}








