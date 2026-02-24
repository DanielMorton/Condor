/// Macro to define API endpoints with minimal boilerplate
///
/// Usage:
/// ```
/// define_endpoint! {
///     name: observations_histogram,
///     method: GET,
///     path: "observations/histogram",
///     response: HistogramResponse,
///     params: [place_id, taxon_id, interval, date_field],
/// }
/// ```
#[macro_export]
macro_rules! define_endpoint {
    (
        name: $name:ident,
        method: GET,
        path: $path:expr,
        response: $response:ty
        $(, params: [$($param:ident),* $(,)?])?
        $(, doc: $doc:expr)?
    ) => {
        paste::paste! {
            $(#[doc = $doc])?
            pub fn $name(&self) -> [<$name:camel Builder>] {
                [<$name:camel Builder>]::new(self.clone())
            }
        }
    };
}

/// Macro to generate endpoint builder structs
#[macro_export]
macro_rules! endpoint_builder {
    (
        name: $name:ident,
        path: $path:expr,
        response: $response:ty,
        params: {
            $(
                $param_name:ident: $param_type:ty
            ),* $(,)?
        }
    ) => {
        paste::paste! {
            pub struct [<$name:camel Builder>] {
                client: $crate::client::INaturalistClient,
                pub params: std::collections::HashMap<String, String>, // Made public
            }

            impl [<$name:camel Builder>] {
                pub fn new(client: $crate::client::INaturalistClient) -> Self {
                    Self {
                        client,
                        params: std::collections::HashMap::new(),
                    }
                }

                $(
                    pub fn $param_name(mut self, value: impl Into<$param_type>) -> Self {
                        self.params.insert(
                            stringify!($param_name).to_string(),
                            value.into().to_string()
                        );
                        self
                    }
                )*

                pub async fn fetch(self) -> $crate::error::Result<$response> {
                    self.client
                        .request_handler
                        .get_with_params($path, &self.params, &self.client.rate_limiter)
                        .await
                }
            }
        }
    };
}

/// Macro to generate batch processing methods
#[macro_export]
macro_rules! define_batch_endpoint {
    (
        name: $name:ident,
        single_method: $single:ident,
        response: $response:ty
    ) => {
        paste::paste! {
            pub async fn [<batch_ $name>](
                &self,
                items: Vec<String>,
                max_workers: usize,
            ) -> Vec<$response> {
                self.process_locations_parallel(
                    items,
                    {
                        let client = self.clone();
                        move |item| {
                            let client = client.clone();
                            async move {
                                client.$single()
                                    .place_id(&item)
                                    .fetch()
                                    .await
                            }
                        }
                    },
                    Some(max_workers),
                ).await
            }
        }
    };
}