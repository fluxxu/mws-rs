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
              })
              .collect();
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
        })
        .collect();

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
        })
        .collect();
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
    get_struct_meta(
      data,
      "from_xml_stream",
      &["no_list_wrapper", "from_attr", "from_content"],
    )
  } else {
    panic!("only struct is supported.");
  };

  let (attr_fields, rest): (Vec<StructFieldMeta>, Vec<StructFieldMeta>) =
    meta.fields.into_iter().partition(|f| {
      f.config_list
        .iter()
        .find(|(k, _)| k == "from_attr")
        .is_some()
    });

  let (content_fields, rest): (Vec<StructFieldMeta>, Vec<StructFieldMeta>) =
    rest.into_iter().partition(|f| {
      f.config_list
        .iter()
        .find(|(k, _)| k == "from_content")
        .is_some()
    });

  let tag_fields = rest;

  if content_fields.len() > 1 {
    panic!("cannot have more than 1 `from_content` field.");
  }

  if content_fields.len() > 0 && tag_fields.len() > 0 {
    panic!("`from_content` field found, other fields must be `from_attr`.");
  }

  let tag_fields: Vec<_> = tag_fields
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
        let last_node = segments.last().unwrap();

        if last_node.ident == "Option" || last_node.ident == "Vec" {
          if let PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            ref args, ..
          }) = last_node.arguments
          {
            if args.len() == 1 {
              let first_node = args.first().unwrap();
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
                        quote! {
                          record.#ident.get_or_insert_with(|| vec![])
                            .push(::xmlhelper::decode::FromXmlStream::from_xml(s)?)
                        }
                      }
                      (false, true) => {
                        quote! {
                          record.#ident
                            .push(::xmlhelper::decode::FromXmlStream::from_xml(s)?)
                        }
                      }
                      (true, false) => {
                        quote! {
                          record.#ident = ::xmlhelper::decode::fold_elements(s, vec![], |s, v| {
                            v.push(::xmlhelper::decode::FromXmlStream::from_xml(s)?);
                            Ok(())
                          }).map(Some)?
                        }
                      }
                      (false, false) => {
                        quote! {
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

                if let Some(arg_ident) = segments.last().map(|node| node.ident.clone()) {
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
    })
    .collect();

  let attr_fields: Vec<_> = attr_fields
    .iter()
    .map(|f| {
      let ident = &f.ident;
      let ident_str = format!("{}", ident);
      let attr_name = f
        .config_list
        .iter()
        .find(|(k, _)| k == "from_attr")
        .and_then(|(_, v)| v.clone())
        .unwrap_or_else(|| ident_str.clone());
      if let Type::Path(TypePath {
        path: Path { ref segments, .. },
        ..
      }) = f.ty
      {
        let last_node = segments.last().unwrap();

        if last_node.ident == "Option" {
          quote! {
            if let Some(value) = elem.attributes.value(#attr_name) {
              record.#ident = Some(::xmlhelper::decode::parse_str(&value)?);
            }
          }
        } else {
          quote! {
            if let Some(value) = elem.attributes.value(#attr_name) {
              record.#ident = ::xmlhelper::decode::parse_str(&value)?;
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
    })
    .collect();

  let assign_attr_fields = if attr_fields.len() > 0 {
    quote! {
      if let Some(elem) = s.container_elem() {
        #(#attr_fields)*
      }
    }
  } else {
    quote! {}
  };

  let content_field = if let Some(f) = content_fields.get(0) {
    let ident = &f.ident;
    quote! {
      record.#ident = ::xmlhelper::decode::FromXmlStream::from_xml(s)?;
    }
  } else {
    quote! {}
  };

  let assign_tag_fields = if tag_fields.len() > 0 {
    quote! {
      use ::xmlhelper::decode::fold_elements;

      record = fold_elements(s, record, |s, record| {
        match s.local_name() {
          #(#tag_fields)*
          _ => {}
        }
        Ok(())
      })?;
    }
  } else {
    quote! {}
  };

  let expanded = quote! {
    impl<_S> ::xmlhelper::decode::FromXmlStream<_S> for #name
    where _S: ::xmlhelper::decode::XmlEventStream
    {
      fn from_xml(s: &mut _S) -> ::result::MwsResult<Self> {
        let mut record = Self::default();

        #assign_attr_fields
        #content_field
        #assign_tag_fields

        Ok(record)
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
    })
    .collect();

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
    })
    .collect();

  StructMeta { fields }
}

fn get_config_items(attr_name: &str, field: &Field, wl: &[&str]) -> Vec<(String, Option<String>)> {
  field
    .attrs
    .iter()
    .filter_map(|a| {
      a.parse_meta().ok().and_then(|meta| match meta {
        Meta::List(list) => {
          if list.path.is_ident(attr_name) {
            Some(
              list
                .nested
                .iter()
                .filter_map(|m| match m {
                  NestedMeta::Meta(meta) => {
                    let kv = match meta {
                      Meta::NameValue(MetaNameValue {
                        path,
                        lit: Lit::Str(lit_str),
                        ..
                      }) => {
                        let k = path.get_ident().unwrap().to_string();
                        let v = lit_str.value();
                        (k, Some(v))
                      }
                      Meta::Path(path) => {
                        let k = path.get_ident().unwrap().to_string();
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
                })
                .collect::<Vec<_>>(),
            )
          } else {
            None
          }
        }
        _ => None,
      })
    })
    .flat_map(|i| i)
    .collect()
}
