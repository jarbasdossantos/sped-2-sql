use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type, Ident, GenericArgument, PathArguments};
use syn::spanned::Spanned;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

/// Macro para derivar automaticamente a implementação da trait TableCreator.
///
/// Esta macro implementa a trait TableCreator para estruturas que já implementam a trait Model,
/// permitindo que elas criem suas próprias tabelas no banco de dados com base nos campos da struct.
#[proc_macro_derive(TableCreator)]
pub fn derive_table_creator(input: TokenStream) -> TokenStream {
    // Parse do input
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Extrai os campos da struct
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("TableCreator só pode ser derivado para structs com campos nomeados"),
        },
        _ => panic!("TableCreator só pode ser derivado para structs"),
    };
    
    // Gera os pares de campo e tipo SQL
    let field_types = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap().to_string().to_uppercase();
        let sql_type = match &field.ty {
            Type::Path(type_path) => {
                let type_name = type_path.path.segments.last().unwrap().ident.to_string();
                match type_name.as_str() {
                    "i32" => "INTEGER",
                    "i64" => "INTEGER",
                    "f32" => "REAL",
                    "f64" => "REAL",
                    "String" => "TEXT",
                    "bool" => "BOOLEAN",
                    _ => {
                        // Verifica se é um Option<T>
                        if type_name == "Option" {
                            if let Some(arg) = &type_path.path.segments.last().unwrap().arguments {
                                match arg {
                                    syn::PathArguments::AngleBracketed(args) => {
                                        if let Some(arg) = args.args.first() {
                                            if let syn::GenericArgument::Type(Type::Path(inner_type)) = arg {
                                                let inner_type_name = inner_type.path.segments.last().unwrap().ident.to_string();
                                                match inner_type_name.as_str() {
                                                    "i32" => "INTEGER",
                                                    "i64" => "INTEGER",
                                                    "f32" => "REAL",
                                                    "f64" => "REAL",
                                                    "String" => "TEXT",
                                                    "bool" => "BOOLEAN",
                                                    _ => "TEXT",
                                                }
                                            } else {
                                                "TEXT"
                                            }
                                        } else {
                                            "TEXT"
                                        }
                                    },
                                    _ => "TEXT",
                                }
                            } else {
                                "TEXT"
                            }
                        } else {
                            "TEXT"
                        }
                    }
                }
            },
            _ => "TEXT",
        };
        
        // Adiciona PRIMARY KEY para o campo id
        let sql_type = if field_name == "ID" {
            format!("{} PRIMARY KEY", sql_type)
        } else {
            sql_type.to_string()
        };
        
        // Adiciona DEFAULT NULL para campos Option<T>
        let mut token_stream = TokenStream2::new();
        field.ty.to_tokens(&mut token_stream);
        let type_str = token_stream.to_string();
        
        let sql_type = if type_str.starts_with("Option") && field_name != "ID" {
            format!("{} DEFAULT NULL", sql_type)
        } else {
            sql_type
        };
        
        quote! {
            (#field_name, #sql_type)
        }
    });

    // Gera a implementação da trait TableCreator
    let expanded = quote! {
        #[async_trait::async_trait]
        impl TableCreator for #name {
            fn table_fields() -> Vec<(&'static str, &'static str)> {
                vec![
                    #(#field_types),*
                ]
            }
        }
    };

    // Retorna o código gerado
    TokenStream::from(expanded)
}