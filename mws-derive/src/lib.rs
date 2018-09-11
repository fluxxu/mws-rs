#![recursion_limit = "128"]

#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;
use syn::*;

#[proc_macro_derive(SerializeMwsParams, attributes(mws_param))]
pub fn derive_params(input: TokenStream) -> TokenStream {
  let input: DeriveInput = syn::parse(input).unwrap();

  let name = input.ident;

  let expanded = match input.data {
    Data::Struct(meta) => {
      let item_push: Vec<_> = meta
        .fields
        .iter()
        .map(|f| {
          let ident = &f
            .ident
            .as_ref()
            .expect("only named struct field is supported.");
          let config_items: Vec<(String, Option<String>)> =
            get_config_items("mws_param", f, &["list_item_type_name"]);
          let ident_str = format!("{}", ident);

          let let_next_ctx = {
            let setters: Vec<_> = config_items
              .iter()
              .filter_map(|(k, v)| {
                if k == "list_item_type_name" && v.is_some() {
                  Some(quote! {
                    next_ctx.field_config.list_item_type_name = Some(#v);
                  })
                } else {
                  None
                }
              }).collect();
            quote! {
              let mut next_ctx = ctx.clone();
              next_ctx.field_config.list_item_type_name = Some("member");
              #(#setters)*
            }
          };

          let update_path = quote! {
            let next_path = match ctx.path.as_ref() {
              None => #ident_str.to_owned(),
              Some(ref path) => format!("{}.{}", path, #ident_str),
            };
            next_ctx.path = Some(next_path);
          };

          quote! {
            {
              #let_next_ctx
              #update_path

              self.#ident.serialize_mws_params(&next_ctx, pairs);
            }
          }
        }).collect();

      quote! {
        impl ::SerializeMwsParams for #name {
          fn serialize_mws_params(&self, ctx: &::SerializeMwsParamsContext, pairs: &mut Vec<(String, String)>) {
            #(#item_push)*
          }
        }
      }
    }
    Data::Enum(meta) => {
      let pat_item: Vec<_> = meta
        .variants
        .iter()
        .map(|v| {
          if v.fields != Fields::Unit {
            panic!("only unit enum variant is supported");
          }

          let variant_ident = &v.ident;
          let variant_ident_string = v.ident.to_string();

          quote! {
            #name::#variant_ident => #variant_ident_string,
          }
        }).collect();
      quote! {
        impl ::SerializeMwsParams for #name {
          fn serialize_mws_params(&self, ctx: &::SerializeMwsParamsContext, pairs: &mut Vec<(String, String)>) {
            let value = match *self {
              #(#pat_item)*
            };

            pairs.push((ctx.path.clone().expect("mws params type should be struct").to_owned(), value.to_owned()))
          }
        }
      }
    }
    _ => {
      panic!("union is not supported.");
    }
  };

  expanded.into()
}

#[proc_macro_derive(FromXmlStream, attributes(from_xml_stream))]
pub fn derive_from_xml_stream(input: TokenStream) -> TokenStream {
  let input: DeriveInput = syn::parse(input).unwrap();

  let name = input.ident;

  let meta = if let Data::Struct(data) = input.data {
    get_struct_meta(data, "from_xml_stream", &["no_list_wrapper"])
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
                let quote_vec =
                  |ident: &Ident, optional: bool, cl: &Vec<(String, Option<String>)>| {
                    let ident_str = ident.to_string();
                    let no_list_wrapper = cl.iter().find(|(k, _)| k == "no_list_wrapper").is_some();
                    let expr = match (optional, no_list_wrapper) {
                      (true, true) => {
                        quote!{
                          record.#ident.get_or_insert_with(|| vec![])
                            .push(::xmlhelper::decode::FromXmlStream::from_xml(s)?)
                        }
                      }
                      (false, true) => {
                        quote!{
                          record.#ident
                            .push(::xmlhelper::decode::FromXmlStream::from_xml(s)?)
                        }
                      }
                      (true, false) => {
                        quote!{
                          record.#ident = ::xmlhelper::decode::fold_elements(s, vec![], |s, v| {
                            v.push(::xmlhelper::decode::FromXmlStream::from_xml(s)?);
                            Ok(())
                          }).map(Some)?
                        }
                      }
                      (false, false) => {
                        quote!{
                          record.#ident = ::xmlhelper::decode::fold_elements(s, vec![], |s, v| {
                            v.push(::xmlhelper::decode::FromXmlStream::from_xml(s)?);
                            Ok(())
                          })?
                        }
                      }
                    };

                    quote! {
                      #ident_str => #expr,
                    }
                  };

                if let Some(arg_ident) = segments.last().map(|node| node.value().ident.clone()) {
                  // `DateTime<_>` does not impl `Default`
                  // So generic impl FromXmlStream for Option<T> won't work
                  if last_node.ident == "Option" {
                    if arg_ident == "DateTime" {
                      return quote! {
                        #ident_str => record.#ident = ::xmlhelper::decode::characters(s).map(Some)?,
                      };
                    } else if arg_ident == "Vec" {
                      return quote_vec(ident, true, &f.config_list);
                    }
                  }

                  // error[E0477]: the type `mws::xmlhelper::decode::ElementScopedStream<'_, _S>` does not fulfill the required lifetime
                  if last_node.ident == "Vec" {
                    return quote_vec(ident, false, &f.config_list);
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

#[proc_macro_derive(FromTdffRow, attributes(from_tdff_row))]
pub fn derive_from_diff_row(input: TokenStream) -> TokenStream {
  let input: DeriveInput = syn::parse(input).unwrap();

  let name = input.ident;

  let meta = if let Data::Struct(data) = input.data {
    get_struct_meta(data, "from_tdff_row", &["key"])
  } else {
    panic!("only struct is supported.");
  };

  let fields: Vec<_> = meta
    .fields
    .iter()
    .map(|f| {
      let ident = &f.ident;
      match f.config_list.iter().find(|(k, _)| k == "key") {
        Some(&(_, Some(ref v))) => {
          if v.contains(',') {
            let keys: Vec<_> = v.split(',').map(|s| s.trim()).collect();
            quote! {
              #(#keys)|* => record.#ident = FromTdffField::parse_tdff_field(k, &v)?,
            }
          } else {
            quote! {
              #v => record.#ident = FromTdffField::parse_tdff_field(k, &v)?,
            }
          }
        }
        _ => {
          let ident_str = format!("{}", ident);
          let ident_underscore_str = ident_str.replace("-", "_");
          if ident_str == ident_underscore_str {
            quote! {
              #ident_str => record.#ident = FromTdffField::parse_tdff_field(k, v)?,
            }
          } else {
            quote! {
              #ident_str => record.#ident = FromTdffField::parse_tdff_field(k, v)?,
            }
          }
        }
      }
    }).collect();

  let expanded = quote! {
    impl ::mws::tdff::FromTdffRow for #name
    {
      fn from_tdff_row(pairs: &::mws::tdff::TdffRow) -> ::mws::result::MwsResult<Self> {
        use ::mws::tdff::FromTdffField;
        let mut record = #name::default();
        for (k, v) in pairs {
          let k = k as &str;
          match k {
            #(#fields)*
            _ => {},
          }
        }
        Ok(record)
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
  config_list: Vec<(String, Option<String>)>,
}

fn get_struct_meta(data: DataStruct, config_attr_name: &str, config_key_wl: &[&str]) -> StructMeta {
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
        config_list: get_config_items(config_attr_name, field, config_key_wl),
      }
    }).collect();

  StructMeta { fields }
}

fn get_config_items(attr_name: &str, field: &Field, wl: &[&str]) -> Vec<(String, Option<String>)> {
  field
    .attrs
    .iter()
    .filter_map(|a| {
      a.interpret_meta().and_then(|meta| match meta {
        Meta::List(list) => if list.ident == attr_name {
          Some(
            list
              .nested
              .iter()
              .filter_map(|m| match m {
                NestedMeta::Meta(meta) => {
                  let kv = match meta {
                    Meta::NameValue(MetaNameValue {
                      ident,
                      lit: Lit::Str(lit_str),
                      ..
                    }) => {
                      let k = ident.to_string();
                      let v = lit_str.value();
                      (k, Some(v))
                    }
                    Meta::Word(ident) => {
                      let k = ident.to_string();
                      (k, None)
                    }
                    _ => {
                      panic!("only `key = \"value\"` or `key` is allowed in config attrbute.");
                    }
                  };

                  if !wl.contains(&kv.0.as_ref()) {
                    panic!(
                      "unknown config key: `{}`. expecting {}",
                      kv.0,
                      wl.iter()
                        .map(|k| format!("`{}`", k))
                        .collect::<Vec<_>>()
                        .join(", ")
                    );
                  }

                  Some(kv)
                }
                _ => None,
              }).collect::<Vec<_>>(),
          )
        } else {
          None
        },
        _ => None,
      })
    }).flat_map(|i| i)
    .collect()
}
