#[macro_export]
macro_rules! sped_model {
    (
        $(#[$meta:meta])*
        $struct_name:ident,
        $reg_code:expr,
        $schema_mod:ident,
        [$($field:ident),* $(,)?]
    ) => {
        $crate::sped_model_inner! {
            $(#[$meta])*
            $struct_name, $reg_code, $crate::schemas::$schema_mod, [$($field),*]
        }
    };
}

#[macro_export]
macro_rules! sped_model_inner {
    (
        $(#[$meta:meta])*
        $struct_name:ident,
        $reg_code:expr,
        $schema_path:path,
        [$($field:ident),* $(,)?]
    ) => {
        #[derive(
            ::std::fmt::Debug,
            ::std::clone::Clone,
            ::serde::Serialize,
            ::serde::Deserialize,
            ::diesel::prelude::Queryable,
            ::diesel::Selectable,
        )]
        #[diesel(check_for_backend(::diesel::sqlite::Sqlite))]
        #[diesel(table_name = $schema_path::dsl)]
        $(#[$meta])*
        pub struct $struct_name {
            pub id: i32,
            pub file_id: ::std::option::Option<i32>,
            pub parent_id: ::std::option::Option<i32>,
            pub reg: ::std::option::Option<::std::string::String>,
            $(
                pub $field: ::std::option::Option<::std::string::String>,
            )*
        }

        #[::async_trait::async_trait]
        impl $crate::models::traits::Model for $struct_name {
            fn new(
                fields: ::std::vec::Vec<&str>,
                new_id: ::std::option::Option<i32>,
                new_parent_id: ::std::option::Option<i32>,
                new_file_id: i32,
            ) -> Self {
                $crate::sped_init!(
                    $struct_name, fields,
                    new_id, new_parent_id, new_file_id,
                    2, [$($field),*]
                )
            }

            fn get(
                file_id: i32,
                parent_id: ::std::option::Option<i32>,
                conn: &mut ::diesel::sqlite::SqliteConnection,
            ) -> ::std::result::Result<::std::vec::Vec<$struct_name>, ::diesel::result::Error> {
                use $schema_path::{dsl as schema, table};
                use ::diesel::ExpressionMethods;
                use ::diesel::QueryDsl;
                use ::diesel::SelectableHelper;
                use ::diesel::RunQueryDsl;

                if let ::std::option::Option::Some(id) = parent_id {
                    ::std::result::Result::Ok(
                        table
                            .filter(schema::file_id.eq(&file_id))
                            .filter(schema::parent_id.eq(&id))
                            .select($struct_name::as_select())
                            .load(conn)?
                    )
                } else {
                    ::std::result::Result::Ok(
                        table
                            .filter(schema::file_id.eq(&file_id))
                            .select($struct_name::as_select())
                            .load(conn)?
                    )
                }
            }

            fn save<'a>(&'a self) -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ::std::result::Result<i32, ::diesel::result::Error>> + ::std::marker::Send + 'a>> {
                ::std::boxed::Box::pin(async move {
                    use $schema_path::{dsl as schema, table};
                    use ::diesel::ExpressionMethods;
                    use ::diesel::RunQueryDsl;
                    use ::diesel::dsl::sql;
                    use ::diesel::sql_types::Integer;
                    use ::diesel::QueryDsl;

                    let mut conn = $crate::database::get_pool().lock().await.get().unwrap();

                    ::diesel::insert_into(table)
                        .values((
                            schema::file_id.eq(&self.file_id),
                            schema::parent_id.eq(&self.parent_id),
                            schema::reg.eq(&self.reg.clone()),
                            $(
                                schema::$field.eq(&self.$field),
                            )*
                        ))
                        .execute(&mut conn)?;

                    ::std::result::Result::Ok(
                        sql::<Integer>("SELECT last_insert_rowid()")
                            .get_result::<i32>(&mut conn)?
                    )
                })
            }

            fn get_id(&self) -> ::std::option::Option<i32> {
                ::std::option::Option::Some(self.id)
            }

            fn get_file_id(&self) -> ::std::option::Option<i32> {
                self.file_id
            }

            fn get_entity_name(&self) -> ::std::string::String {
                ::std::string::String::from(stringify!($struct_name))
            }

            fn get_display_fields(&self) -> ::std::vec::Vec<(::std::string::String, ::std::string::String)> {
                self.generate_display_fields()
            }
        }

        impl ::std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                $crate::models::traits::Model::display_format(self, f)
            }
        }

        $crate::impl_display_fields!(
            $struct_name,
            [reg, $($field),*]
        );

        $crate::register_model!(
            $struct_name, $reg_code
        );
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! sped_init {
    (
        $struct_name:ident, $fields:ident,
        $new_id:ident, $new_parent_id:ident, $new_file_id:ident,
        $idx:expr, [$($field:ident),*]
    ) => {
        $crate::sped_munch!(
            $struct_name, $fields, $new_id, $new_parent_id, $new_file_id,
            $idx, [$($field),*],
            {
                id: $new_id.unwrap_or(0),
                file_id: ::std::option::Option::Some($new_file_id),
                parent_id: $new_parent_id,
                reg: $fields.get(1).map(|s| s.to_string()),
            }
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! sped_munch {
    // Final: lista vazia — gera a struct literal completa
    (
        $struct_name:ident, $fields:ident,
        $new_id:ident, $new_parent_id:ident, $new_file_id:ident,
        $idx:expr, [],
        { $($acc:tt)* }
    ) => {
        $struct_name { $($acc)* }
    };
    // Iteração: head sozinho (último field)
    (
        $struct_name:ident, $fields:ident,
        $new_id:ident, $new_parent_id:ident, $new_file_id:ident,
        $idx:expr, [$head:ident],
        { $($acc:tt)* }
    ) => {
        $crate::sped_munch!(
            $struct_name, $fields,
            $new_id, $new_parent_id, $new_file_id,
            $idx + 1, [],
            { $($acc)* $head: $crate::models::utils::get_field(&$fields, $idx), }
        )
    };
    // Iteração: head + tail (pelo menos um tail com vírgula)
    (
        $struct_name:ident, $fields:ident,
        $new_id:ident, $new_parent_id:ident, $new_file_id:ident,
        $idx:expr, [$head:ident, $($tail:ident),+],
        { $($acc:tt)* }
    ) => {
        $crate::sped_munch!(
            $struct_name, $fields,
            $new_id, $new_parent_id, $new_file_id,
            $idx + 1, [$($tail),+],
            { $($acc)* $head: $crate::models::utils::get_field(&$fields, $idx), }
        )
    };
}

#[macro_export]
macro_rules! impl_display_fields {
    ($struct_name:ident, [$($field:ident),*]) => {
        impl $struct_name {
            pub fn generate_display_fields(&self) -> Vec<(String, String)> {
                vec![
                    $(
                        (
                            stringify!($field).to_string(),
                            match &self.$field {
                                Some(val) => val.to_string(),
                                None => "".to_string(),
                            }
                        ),
                    )*
                ]
            }
        }
    };
}

#[macro_export]
macro_rules! register_model {
    ($model:ty, $reg:expr) => {
        #[derive(Debug)]
        pub struct ModelFactoryImpl;

        impl $crate::models::traits::ModelFactory for ModelFactoryImpl {
            fn handle_reg(&self) -> &'static str {
                $reg
            }

            fn create_model(
                &self,
                fields: Vec<&str>,
                id: Option<i32>,
                parent_id: Option<i32>,
                file_id: i32,
            ) -> Box<dyn $crate::models::traits::Model> {
                Box::new(<$model as $crate::models::traits::Model>::new(fields, id, parent_id, file_id))
            }
        }

        pub fn register() {
            use $crate::models::registry;
            registry::register(Box::new(ModelFactoryImpl));
        }
    };
}

#[macro_export]
macro_rules! create_loader {
    ($model:ty) => {
        Some(Box::new(move |file_id, parent_id, conn: &mut _| {
            let result = <$model>::get(file_id, parent_id, conn)?;

            Ok(result
                .into_iter()
                .map(|m| Box::new(m) as Box<dyn Model>)
                .collect::<Vec<_>>())
        }) as LoadModelFn)
    };
}
