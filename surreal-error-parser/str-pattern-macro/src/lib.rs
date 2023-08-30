use proc_macro2::Ident;
use quote::quote;
use regex::Match;
use syn::{spanned::Spanned, Attribute, DeriveInput, FieldsNamed, FieldsUnnamed};

#[proc_macro_derive(StrPattern, attributes(str_pattern))]
pub fn derive_str_pattern(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let (match_arms, regexes) = match &input.data {
        syn::Data::Enum(d) => match impl_enum(d) {
            Ok(output) => output,
            Err(err) => return proc_macro::TokenStream::from(err.to_compile_error()),
        },
        _ => {
            return proc_macro::TokenStream::from(proc_macro2::TokenStream::from(
                syn::Error::new(input.span(), "StrPattern can only be derived for enums")
                    .to_compile_error(),
            ))
        }
    };

    let ident = &input.ident;

    let regexes_ident = Ident::new(&format!("__{}_REGEXES", ident).to_uppercase(), input.span());

    let output: proc_macro2::TokenStream = {
        quote! {
            static #regexes_ident:
                ::once_cell::sync::Lazy<::std::vec::Vec<::regex::Regex>> =
                ::once_cell::sync::Lazy::new(|| vec![ #regexes ]);

            impl #ident {
                pub fn from_string(string: &str) -> ::std::option::Option<Self> {
                    for (i, re) in #regexes_ident.iter().enumerate() {
                        if re.is_match(string) {
                            let caps = re.captures(string)?;

                            return match i {
                                #match_arms
                                _ => None
                            };
                        }
                    }

                    None
                }
            }
        }
    };

    return proc_macro::TokenStream::from(output);
}

fn impl_enum(
    data: &syn::DataEnum,
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let str_pattern_regex = regex::Regex::new(r"\\\{\w+\\\}").unwrap();

    let mut match_arms = vec![];
    let mut regexes = vec![];

    for (i, variant) in data.variants.iter().enumerate() {
        let Some(attribute) = variant.attrs.iter().find(|attr| {
            let ident = attr.meta.path().get_ident();

            match ident {
                Some(ident) => ident == "str_pattern",
                None => false,
            }
        }) else {
            return Err(syn::Error::new_spanned(
                variant,
                "missing `#[str_pattern(\"...\")]` attribute",
            ));
        };

        let str_value = regex::escape(&attribute.parse_args::<syn::LitStr>()?.value());

        let captures = str_pattern_regex
            .captures_iter(&str_value)
            .map(|c| c.iter().flatten().map(strip_brackets).last())
            .collect::<Vec<_>>();

        let variant_ident = &variant.ident;

        match &variant.fields {
            syn::Fields::Unit => {
                validate_unit(&captures, attribute)?;

                match_arms.push(quote! {
                    #i => Some(Self::#variant_ident),
                });

                regexes.push(quote! { ::regex::Regex::new(#str_value).unwrap(), });
            }
            syn::Fields::Unnamed(fields) => {
                validate_unnamed(&captures, attribute, fields)?;

                let str_value = str_pattern_regex
                    .replace_all(&str_value, |c: &regex::Captures| {
                        let ident = c.iter().flatten().map(strip_brackets).last().unwrap();

                        format!(r"(?<_{}>.*)", ident)
                    })
                    .to_string();

                let fields = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| syn::LitStr::new(&format!("_{}", i), variant.span()))
                    .collect::<Vec<syn::LitStr>>();

                match_arms.push(quote! {
                    #i =>
                    Some(Self::#variant_ident(#(caps[#fields].to_string()),*)),
                });

                regexes.push(quote! { ::regex::Regex::new(#str_value).unwrap(), });
            }
            syn::Fields::Named(fields) => {
                validate_named(&captures, attribute, fields)?;

                let mut capture_groups: Vec<String> = Vec::with_capacity(fields.named.len());

                let str_value = str_pattern_regex
                    .replace_all(&str_value, |c: &regex::Captures| {
                        let ident = c.iter().flatten().map(strip_brackets).last().unwrap();

                        if capture_groups.contains(&ident.to_string()) {
                            return r".*".to_string();
                        }

                        capture_groups.push(ident.to_string());
                        format!(r"(?<{}>.*)", ident)
                    })
                    .to_string();

                let field_idents = fields
                    .named
                    .iter()
                    .map(|f| &f.ident)
                    .flatten()
                    .collect::<Vec<&Ident>>();

                let field_literals = fields
                    .named
                    .iter()
                    .map(|f| syn::LitStr::new(&f.ident.as_ref().unwrap().to_string(), f.span()))
                    .collect::<Vec<syn::LitStr>>();

                let tokens = quote! {
                    #i =>
                         Some(Self::#variant_ident
                         {
                            #(#field_idents: caps[#field_literals].to_string()),*
                         }),
                };

                match_arms.push(tokens);

                regexes.push(quote! { ::regex::Regex::new(#str_value).unwrap(), });
            }
        };
    }

    Ok((
        match_arms.into_iter().collect::<proc_macro2::TokenStream>(),
        regexes.into_iter().collect::<proc_macro2::TokenStream>(),
    ))
}

fn strip_brackets<'a>(m: Match<'a>) -> &'a str {
    m.as_str().split_at(2).1.split_at(m.as_str().len() - 4).0
}

fn validate_unit(captures: &Vec<Option<&str>>, attribute: &Attribute) -> syn::Result<()> {
    if captures.len() != 0 {
        return Err(syn::Error::new_spanned(
            attribute,
            format!(
                "unit variant cannot have template vars. Remove {}",
                captures
                    .iter()
                    .map(|c| c.unwrap())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ));
    }

    Ok(())
}

fn validate_unnamed(
    captures: &Vec<Option<&str>>,
    attribute: &Attribute,
    fields: &FieldsUnnamed,
) -> syn::Result<()> {
    let matched = captures.len();

    if matched != fields.unnamed.len() {
        return Err(syn::Error::new_spanned(
            attribute,
            format!(
                "unnamed variant has {} template vars, but {} fields",
                matched,
                fields.unnamed.len()
            ),
        ));
    }

    if captures
        .iter()
        .any(|c| c.is_none() || c.unwrap().is_empty())
    {
        return Err(syn::Error::new_spanned(
            attribute,
            "template vars cannot be empty",
        ));
    }

    let indices_result = captures
        .iter()
        .map(|c| str::parse::<usize>(c.unwrap()))
        .collect::<Vec<_>>();

    for index in indices_result.into_iter() {
        match index {
            Err(_) => {
                return Err(syn::Error::new_spanned(
                    attribute,
                    "template vars in tuple variants must be valid indices",
                ))
            }
            Ok(index) => {
                if index >= fields.unnamed.len() {
                    return Err(syn::Error::new_spanned(
                        attribute,
                        format!(
                            "template var {} is out of bounds for tuple with {} fields",
                            index,
                            fields.unnamed.len()
                        ),
                    ));
                }
            }
        }
    }

    Ok(())
}

fn validate_named(
    captures: &Vec<Option<&str>>,
    attribute: &Attribute,
    fields: &FieldsNamed,
) -> syn::Result<()> {
    let missing = fields
        .named
        .iter()
        .filter(|f| {
            let ident = f.ident.as_ref().unwrap().to_string();
            !captures.iter().flatten().any(|c| *c == ident)
        })
        .map(|f| {
            format!(
                "named variant is missing template variable for field: `{}`",
                f.ident.as_ref().unwrap().to_string()
            )
        })
        .collect::<Vec<_>>();

    let invalid = captures
        .iter()
        .flatten()
        .filter(|c| {
            let ident = c.to_string();
            !fields
                .named
                .iter()
                .any(|f| f.ident.as_ref().unwrap().to_string() == ident)
        })
        .map(|f| format!("unknown field name: `{}`", f))
        .collect::<Vec<_>>();

    if invalid.len() + missing.len() > 0 {
        return Err(syn::Error::new_spanned(
            attribute,
            missing
                .iter()
                .chain(invalid.iter())
                .map(|e| e.as_str())
                .collect::<Vec<_>>()
                .join("\n"),
        ));
    }

    Ok(())
}
