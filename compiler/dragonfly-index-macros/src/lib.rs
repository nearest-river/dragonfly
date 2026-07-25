
use quote::quote;
use proc_macro::TokenStream;
use proc_macro2::{
  Span,
  TokenStream as TokenStream2,
};

use syn::{
  *,
  parse::*,
};


struct NewType(TokenStream);

#[proc_macro]
pub fn new_type(input: TokenStream)-> TokenStream {
  let input=parse_macro_input!(input as NewType);
  input.0.into()
}


impl Parse for NewType {
  fn parse(input: ParseStream)-> syn::Result<Self> {
    let mut attrs=input.call(Attribute::parse_outer)?;
    let vis=input.parse::<Visibility>()?;
    input.parse::<Token![struct]>()?;
    let name=input.parse::<Ident>()?;

    let body;
    braced!(body in input);

    // Any additional `#[derive]` macro paths to apply
    let mut debug_format: Option<Lit>=None;
    let mut max=Option::<Lit>::None;
    let mut consts=Vec::<TokenStream2>::new();
    let mut _encodable=false;
    let mut ord=false;
    let mut stable_hash=false;

    attrs.retain(|attr| match attr.path().get_ident() {
      None=> true,
      Some(ident)=> match &*ident.to_string() {
        "orderable"=> {
          ord=true;
          false
        },
        "stable_hash"=> {
          stable_hash=true;
          false
        },
        "max"=> {
          let Meta::NameValue(MetaNameValue { value: Expr::Lit(lit), .. })=&attr.meta
          else {
            panic!("#[max = NUMBER] attribute requires max value");
          };

          if let Some(old)=max.replace(lit.lit.clone()) {
            panic!("Specified multiple max: {old:?}");
          }

          false
        },
        "debug_format"=> {
          let Meta::NameValue(MetaNameValue { value: Expr::Lit(lit), .. })=&attr.meta
          else {
            panic!("#[debug_format = FMT] attribute requires a format");
          };

          if let Some(old)=debug_format.replace(lit.lit.clone()) {
            panic!("Specified multiple debug format options: {old:?}");
          }

          false
        },
        _=> true,
      },
    });

    loop {
      // We've parsed everything that the user provided, so we're done
      if body.is_empty() {
        break;
      }

      // Otherwise, we are parsing a user-defined constant
      let const_attrs=body.call(Attribute::parse_outer)?;
      body.parse::<Token![const]>()?;
      let const_name=body.parse::<Ident>()?;

      body.parse::<Token![=]>()?;
      let const_val=body.parse::<Expr>()?;

      body.parse::<Token![;]>()?;
      consts.push(quote! { #(#const_attrs)* #vis const #const_name: #name = #name::from_u32(#const_val); });
    }


    let debug_format=debug_format.unwrap_or_else(|| Lit::Str(LitStr::new("{}",Span::call_site())));

    // shave off 256 indices at the end to allow space for packing these indices into enums
    let max=max.unwrap_or_else(|| Lit::Int(LitInt::new("0xFFFF_FF00",Span::call_site())));


    let step_impl=ord.then(|| step_impl(&name));
    let derive_ord=ord.then_some(quote! { #[derive(PartialOrd,Ord)] });

    let stable_hash_impl=stable_hash_impl(&name);
    let debug_impl=debug_impl(&name,debug_format);
    let ops_impl=ops_impl(&name);


    let token_stream=quote! {
      #(#attrs)*
      #derive_ord
      #[rustc_pass_by_value]
      #[derive(Clone,Copy,PartialEq,Eq,Hash)]
      #vis struct #name(u32);

      #(#consts)*

      impl #name {
        /// Maximum value the index can take, as a `u32`.
        #vis const MAX_AS_U32: u32=#max;
        /// Maximum value the index can take.
        #vis const MAX: Self=Self::from_u32(#max);
        /// Zero value of the index.
        #vis const ZERO: Self=Self::from_u32(0);

        /// Creates a new index from a given `usize`.
        ///
        /// # Panics
        ///
        /// Will panic if `value` exceeds `MAX`.
        #[inline(always)]
        #vis const fn from_usize(value: usize)-> Self {
          assert!(value <= (#max as usize));
          // SAFETY: We just checked that `value <= max`.
          unsafe {
            Self::from_u32_unchecked(value as u32)
          }
        }

        /// Creates a new index from a given `u32`.
        ///
        /// # Panics
        ///
        /// Will panic if `value` exceeds `MAX`.
        #[inline(always)]
        #vis const fn from_u32(value: u32)-> Self {
          assert!(value <= #max);
          // SAFETY: We just checked that `value <= max`.
          unsafe {
            Self::from_u32_unchecked(value)
          }
        }

        /// Creates a new index from a given `u16`.
        ///
        /// # Panics
        ///
        /// Will panic if `value` exceeds `MAX`.
        #[inline(always)]
        #vis const fn from_u16(value: u16)-> Self {
          let value = value as u32;
          assert!(value <= #max);
          // SAFETY: We just checked that `value <= max`.
          unsafe {
            Self::from_u32_unchecked(value)
          }
        }

        /// Creates a new index from a given `u32`.
        ///
        /// # Safety
        ///
        /// The provided value must be less than or equal to the maximum value for the newtype.
        /// Providing a value outside this range is undefined due to layout restrictions.
        ///
        /// Prefer using `from_u32`.
        #[inline(always)]
        #vis const unsafe fn from_u32_unchecked(value: u32)-> Self {
          Self(value)
        }

        /// Extracts the value of this index as a `usize`.
        #[inline(always)]
        #vis const fn index(self)-> usize {
          self.as_usize()
        }

        /// Extracts the value of this index as a `u32`.
        #[inline(always)]
        #vis const fn as_u32(self)-> u32 {
          self.0
        }

        /// Extracts the value of this index as a `usize`.
        #[inline(always)]
        #vis const fn as_usize(self)-> usize {
          self.as_u32() as usize
        }
      }

      impl From<u32> for #name {
        #[inline(always)]
        fn from(v: u32)-> Self {
          Self::from_u32(v)
        }
      }

      impl From<usize> for #name {
        #[inline(always)]
        fn from(v: usize)-> Self {
          Self::from_usize(v)
        }
      }

      impl Into<u32> for #name {
        #[inline(always)]
        fn into(self)-> u32 {
          self.as_u32()
        }
      }

      impl Into<usize> for #name {
        #[inline(always)]
        fn into(self)-> usize {
          self.as_usize()
        }
      }

      #step_impl
      #stable_hash_impl
      #debug_impl
      #ops_impl


    };

    Ok(Self(token_stream.into()))
  }
}







#[inline]
fn step_impl(name: &Ident)-> TokenStream2 {
  quote! {
    impl ::std::iter::Step for #name {
      #[inline]
      fn steps_between(start: &Self, end: &Self)-> (usize, Option<usize>) {
        ::std::iter::Step::steps_between(&start.index(),&end.index())
      }

      #[inline]
      fn forward_checked(start: Self, count: usize)-> Option<Self> {
        Self::index(start).checked_add(count).map(Self::from_usize)
      }

      #[inline]
      fn backward_checked(start: Self, count: usize)-> Option<Self> {
        Self::index(start).checked_sub(count).map(Self::from_usize)
      }
    }
  }
}

#[inline]
fn stable_hash_impl(name: &Ident)-> TokenStream2 {
  quote! {
    impl ::dragonfly_stable_hash::StableHash for #name {
      #[inline(always)]
      fn stable_hash<Hcx: dragonfly_stable_hash::StableHashCtxt>(
        &self,
        hcx: &mut Hcx,
        hasher: &mut dragonfly_stable_hash::StableHasher
      ) {
        self.as_u32().stable_hash(hcx,hasher);
      }
    }
  }
}

fn debug_impl(name: &Ident,fmt: Lit)-> TokenStream2 {
  quote! {
    impl ::std::fmt::Debug for #name {
      #[inline(always)]
      fn fmt(&self,f: &mut ::std::fmt::Formatter<'_>)-> ::std::fmt::Result {
        write!(f,#fmt,self.as_u32())
      }
    }
  }
}

// TODO(nate): impl dragonfly_index::Idx
fn ops_impl(name: &Ident)-> TokenStream2 {
  quote! {
    impl ::std::ops::Add<usize> for #name {
      type Output=Self;
      #[inline(always)]
      fn add(self,other: usize)-> Self {
        Self::from_usize(self.as_usize() + other)
      }
    }

    impl ::std::ops::AddAssign<usize> for #name {
      #[inline(always)]
      fn add_assign(&mut self,other: usize) {
        *self=*self+other;
      }
    }
  }
}






