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
                Box::new(<$model>::new(fields, id, parent_id, file_id))
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
        Some(Box::new(move |file_id, parent_id| {
            Box::pin(async move {
                let result = <$model>::get(file_id, parent_id).await?;
                Ok(result
                    .into_iter()
                    .map(|m| Box::new(m) as Box<dyn Model + Send>)
                    .collect::<Vec<_>>())
            })
                as Pin<
                    Box<
                        dyn std::future::Future<
                                Output = Result<
                                    Vec<Box<dyn Model + Send>>,
                                    Box<dyn std::error::Error + Send + Sync>,
                                >,
                            > + Send,
                    >,
                >
        }) as LoadModelFn)
    };
}
