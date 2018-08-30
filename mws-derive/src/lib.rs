#![recursion_limit = "128"]

#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use syn::*;

#[proc_macro_derive(SerializeMwsParams)]
pub fn derive_params(input: TokenStream) -> TokenStream {
  let input: DeriveInput = syn::parse(input).unwrap();

  let name = input.ident;

  let meta = if let Data::Struct(data) = input.data {
    get_struct_meta(data)
  } else {
    panic!("only struct is supported.");
  };

  let item_push: Vec<_> = meta
    .fields
    .iter()
    .map(|f| {
      let ident = &f.ident;
      let ident_str = format!("{}", ident);
      let ident_str_with_name = format!("{}.{}", name, ident);
      let bind_key = quote! {
        let part = if include_name {
          #ident_str_with_name
        } else {
          #ident_str
        };

        let key = if !path.is_empty() {
          format!("{}.{}", path, part)
        } else {
          part.to_string()
        };
      };

      quote! {
        #bind_key

        self.#ident.serialize_mws_params(&key, false, pairs);
      }
    }).collect();

  let expanded = quote! {
    impl ::SerializeMwsParams for #name {
      fn serialize_mws_params(&self, path: &str, include_name: bool, pairs: &mut Vec<(String, String)>) {
        #(#item_push)*
      }
    }
  };

  expanded.into()
}

#[proc_macro_derive(FromXmlStream)]
pub fn derive_from_xml_stream(input: TokenStream) -> TokenStream {
  let input: DeriveInput = syn::parse(input).unwrap();

  let name = input.ident;

  let meta = if let Data::Struct(data) = input.data {
    get_struct_meta(data)
  } else {
    panic!("only struct is supported.");
  };

  let fields: Vec<_> = meta
    .fields
    .iter()
    .map(|f| {
      let ident = &f.ident;
      let ident_str = format!("{}", ident);

      // special cases
      if let Type::Path(TypePath {
        path: Path { ref segments, .. },
        ..
      }) = f.ty
      {
        let last = segments.last().unwrap();
        let last_node = last.value();

        if last_node.ident == "Option" || last_node.ident == "Vec" {
          if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            ref args, ..
          }) = last_node.arguments
          {
            if args.len() == 1 {
              let first = args.first().unwrap();
              let first_node = first.value();
              if let &GenericArgument::Type(Type::Path(TypePath {
                path: Path { ref segments, .. },
                ..
              })) = first_node
              {
                if let Some(arg_ident) = segments.last().map(|node| node.value().ident.clone()) {
                  // `DateTime<_>` does not impl `Default`
                  // So generic impl FromXmlStream for Option<T> won't work
                  if last_node.ident == "Option" && arg_ident == "DateTime" {
                    return quote! {
                      #ident_str => record.#ident = ::xmlhelper::decode::characters(s).map(Some)?,
                    };
                  }

                  // error[E0477]: the type `mws::xmlhelper::decode::ElementScopedStream<'_, _S>` does not fulfill the required lifetime
                  if last_node.ident == "Vec" {
                    return quote! {
                      #ident_str => {
                        record.#ident = ::xmlhelper::decode::fold_elements(s, vec![], |s, v| {
                          v.push(::xmlhelper::decode::FromXmlStream::from_xml(s)?);
                          Ok(())
                        })?;
                      },
                    };
                  }
                }
              }
            }
          }
        }
      } else {
        use quote::ToTokens;
        panic!(
          "unsupported field type: '{}'",
          f.ty.clone().into_token_stream()
        );
      }

      // workaround: error[E0477]: the type `mws::xmlhelper::decode::ElementScopedStream<'_, _S>` does not fulfill the required lifetime

      quote! {
        #ident_str => record.#ident = ::xmlhelper::decode::FromXmlStream::from_xml(s)?,
      }
    }).collect();

  let expanded = quote! {
    impl<_S> ::xmlhelper::decode::FromXmlStream<_S> for #name
    where _S: ::xmlhelper::decode::XmlEventStream
    {
      fn from_xml(s: &mut _S) -> ::result::MwsResult<Self> {
        use ::xmlhelper::decode::fold_elements;
        fold_elements(s, Self::default(), |s, record| {
          match s.local_name() {
            #(#fields)*
            _ => {}
          }
          Ok(())
        })
      }
    }
  };

  expanded.into()
}

struct StructMeta {
  fields: Vec<StructFieldMeta>,
}

struct StructFieldMeta {
  ident: Ident,
  ty: Type,
}

fn get_struct_meta(data: DataStruct) -> StructMeta {
  let fields = data
    .fields
    .iter()
    .map(|field| {
      let ident = if let Some(ref ident) = field.ident {
        ident
      } else {
        panic!("unnamed field is not supported");
      };
      StructFieldMeta {
        ident: ident.clone(),
        ty: field.ty.clone(),
      }
    }).collect();

  StructMeta { fields }
}
