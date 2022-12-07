macro_rules! methods {
    ($($method:ident and $method_with_call_id:ident,)+) => {
        $(
            doc_comment! {
                concat!("Make a ", stringify!($method), " API request, and deserialize the result into T"),
                async fn $method<T: for<'de> serde::Deserialize<'de> + serde::Serialize>(&self, url: impl AsRef<str>) -> Result<T>
                {
                    let call_id = uuid::Uuid::new_v4();
                    self.$method_with_call_id(url, call_id).await
                }
            }

            doc_comment! {
                concat!(
                    "Make a ", stringify!($method), " API request, and deserialize the result into T.\n\n",
                    "Logging will use the provided UUID, rather than generating one before making the request.",
                ),
                async fn $method_with_call_id<T: for<'de> serde::Deserialize<'de> + serde::Serialize>(&self, url: impl AsRef<str>, call_id: Uuid) -> Result<T>
                {

                    use log::{debug, error, as_debug, as_serde};

                    let url = url.as_ref();
                    debug!(url = url, method = stringify!($method), call_id = as_debug!(call_id); "making API request");
                    let response = self.client
                        .$method(url)
                        .send()
                        .await?;
                    match response.error_for_status() {
                        Ok(response) => {
                            let response = response
                                .json()
                                .await?;
                            debug!(response = as_serde!(response), url = url, method = stringify!($method), call_id = as_debug!(call_id); "received API response");
                            Ok(response)
                        }
                        Err(err) => {
                            error!(err = as_debug!(err), url = url, method = stringify!($method), call_id = as_debug!(call_id); "error making API request");
                            Err(err.into())
                        }
                    }
                }
            }
         )+
    };
}

macro_rules! paged_routes {

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
            "Equivalent to `", stringify!($method), " /api/v1/",
            $url,
            "`\n# Errors\nIf `access_token` is not set.",
            "\n",
            "```no_run",
            "use elefren::prelude::*;\n",
            "let data = Data::default();\n",
            "let client = Mastodon::from(data);\n",
            "client.", stringify!($name), "();\n",
            "```"
            ),
            pub async fn $name(&self) -> Result<Page<$ret>> {
                use log::{debug, as_debug, error};
                let url = self.route(concat!("/api/v1/", $url));
                let call_id = uuid::Uuid::new_v4();
                debug!(url = url, method = stringify!($method), call_id = as_debug!(call_id); "making API request");
                let response = self.client.$method(&url).send().await?;

                match response.error_for_status() {
                    Ok(response) => {
                        Page::new(self.clone(), response, call_id).await
                    }
                    Err(err) => {
                        error!(err = as_debug!(err), url = url, method = stringify!($method), call_id = as_debug!(call_id); "error making API request");
                        Err(err.into())
                    }
                }
            }

        }

        paged_routes!{$($rest)*}
    };

    ((get ($($(#[$m:meta])* $param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `get /api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."
            ),
            pub async fn $name<'a>(&self, $($param: $typ,)*) -> Result<Page<$ret>> {
                use serde_urlencoded;
                use log::{debug, as_debug, error};

                let call_id = uuid::Uuid::new_v4();

                #[derive(Serialize)]
                struct Data<'a> {
                    $(
                        $(
                        #[$m]
                        )*
                        $param: $typ,
                    )*
                    #[serde(skip)]
                    _marker: ::std::marker::PhantomData<&'a ()>,
                }

                let qs_data = Data {
                    $(
                            $param: $param,
                    )*
                    _marker: ::std::marker::PhantomData,
                };

                let qs = serde_urlencoded::to_string(&qs_data)?;

                let url = format!(concat!("/api/v1/", $url, "?{}"), &qs);

                debug!(url = url, method = "get", call_id = as_debug!(call_id); "making API request");

                let response = self.client.get(&url).send().await?;

                match response.error_for_status() {
                    Ok(response) => {
                        Page::new(self.clone(), response, call_id).await
                    }
                    Err(err) => {
                        error!(err = as_debug!(err), url = url, method = stringify!($method), call_id = as_debug!(call_id); "error making API request");
                        Err(err.into())
                    }
                }
            }
        }

        paged_routes!{$($rest)*}
    };

    () => {}
}

macro_rules! route_v2 {
    ((get ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `get /api/v2/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."
            ),
            pub async fn $name<'a>(&self, $($param: $typ,)*) -> Result<$ret> {
                use serde_urlencoded;
                use log::{debug, as_serde};
                use uuid::Uuid;

                let call_id = Uuid::new_v4();

                #[derive(Serialize)]
                struct Data<'a> {
                    $(
                        $param: $typ,
                    )*
                    #[serde(skip)]
                    _marker: ::std::marker::PhantomData<&'a ()>,
                }

                let qs_data = Data {
                    $(
                            $param: $param,
                    )*
                    _marker: ::std::marker::PhantomData,
                };

                let qs = serde_urlencoded::to_string(&qs_data)?;

                debug!(query_string_data = as_serde!(qs_data); "URL-encoded data to be sent in API request");

                let url = format!(concat!("/api/v2/", $url, "?{}"), &qs);

                self.get_with_call_id(self.route(&url), call_id).await
            }
        }

        route_v2!{$($rest)*}
    };

    () => {}
}

macro_rules! route {

    ((post multipart ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `post /api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."),
            pub async fn $name(&self, $($param: $typ,)*) -> Result<$ret> {
                use reqwest::multipart::{Form, Part};
                use std::io::Read;
                use log::{debug, error, as_debug, as_serde};
                use uuid::Uuid;

                let call_id = Uuid::new_v4();

                let form_data = Form::new()
                    $(
                        .part(stringify!($param), {
                            match std::fs::File::open($param.as_ref()) {
                                Ok(mut file) => {
                                    let mut data = if let Ok(metadata) = file.metadata() {
                                        Vec::with_capacity(metadata.len().try_into()?)
                                    } else {
                                        vec![]
                                    };
                                    file.read_to_end(&mut data)?;
                                    Part::bytes(data)
                                }
                                Err(err) => {
                                    error!(path = $param.as_ref(), error = as_debug!(err); "error reading file contents for multipart form");
                                    return Err(err.into());
                                }
                            }
                        })
                     )*;

                let url = &self.route(concat!("/api/v1/", $url));

                debug!(
                    url = url, method = stringify!($method),
                    multipart_form_data = as_debug!(form_data), call_id = as_debug!(call_id);
                    "making API request"
                );

                let response = self.client
                    .post(url)
                    .multipart(form_data)
                    .send()
                    .await?;

                match response.error_for_status() {
                    Ok(response) => {
                        let response = response.json().await?;
                        debug!(response = as_serde!(response), url = url, method = stringify!($method), call_id = as_debug!(call_id); "received API response");
                        Ok(response)
                    }
                    Err(err) => {
                        error!(err = as_debug!(err), url = url, method = stringify!($method), call_id = as_debug!(call_id); "error making API request");
                        Err(err.into())
                    }
                }
            }
        }

        route!{$($rest)*}
    };

    ((get ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `get /api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."
            ),
            pub async fn $name<'a>(&self, $($param: $typ,)*) -> Result<$ret> {
                use serde_urlencoded;
                use log::{debug, as_serde};
                use uuid::Uuid;

                let call_id = Uuid::new_v4();

                #[derive(Serialize)]
                struct Data<'a> {
                    $(
                        $param: $typ,
                    )*
                    #[serde(skip)]
                    _marker: ::std::marker::PhantomData<&'a ()>,
                }

                let qs_data = Data {
                    $(
                            $param: $param,
                    )*
                    _marker: ::std::marker::PhantomData,
                };


                let qs = serde_urlencoded::to_string(&qs_data)?;

                debug!(query_string_data = as_serde!(qs_data); "URL-encoded data to be sent in API request");

                let url = format!(concat!("/api/v1/", $url, "?{}"), &qs);

                self.get_with_call_id(self.route(&url), call_id).await
            }
        }

        route!{$($rest)*}
    };

    (($method:ident ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `", stringify!($method), " /api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set.",
            ),
            pub async fn $name(&self, $($param: $typ,)*) -> Result<$ret> {
                use log::{debug, error, as_debug, as_serde};
                use uuid::Uuid;

                let call_id = Uuid::new_v4();

                let form_data = json!({
                    $(
                        stringify!($param): $param,
                    )*
                });
                debug!(
                    url = $url, method = stringify!($method),
                    call_id = as_debug!(call_id),
                    form_data = as_serde!(&form_data);
                    "making API request"
                );

                let response = self.client
                    .$method(&self.route(concat!("/api/v1/", $url)))
                    .json(&form_data)
                    .send()
                    .await?;

                match response.error_for_status() {
                    Ok(response) => {
                        let response = response.json().await?;
                        debug!(response = as_serde!(response), url = $url, method = stringify!($method), call_id = as_debug!(call_id); "received API response");
                        Ok(response)
                    }
                    Err(err) => {
                        error!(err = as_debug!(err), url = $url, method = stringify!($method), call_id = as_debug!(call_id); "error making API request");
                        Err(err.into())
                    }
                }
            }
        }

        route!{$($rest)*}
    };

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `", stringify!($method), " /api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set.",
                "\n",
                "```no_run",
                "use elefren::prelude::*;\n",
                "let data = Data::default();\n",
                "let client = Mastodon::from(data);\n",
                "client.", stringify!($name), "();\n",
                "```"
            ),
            pub async fn $name(&self) -> Result<$ret> {
                self.$method(self.route(concat!("/api/v1/", $url))).await
            }
        }

        route!{$($rest)*}
    };

    () => {}
}

macro_rules! route_id {

    ($(($method:ident) $name:ident: $url:expr => $ret:ty,)*) => {
        $(
            doc_comment! {
                concat!(
                    "Equivalent to `", stringify!($method), " /api/v1/",
                    $url,
                    "`\n# Errors\nIf `access_token` is not set.",
                    "\n",
                    "```no_run",
                    "use elefren::prelude::*;\n",
                    "let data = Data::default();\n",
                    "let client = Mastodon::from(data);\n",
                    "client.", stringify!($name), "(\"42\");\n",
                    "#   Ok(())\n",
                    "# }\n",
                    "```"
                ),
                pub async fn $name(&self, id: &str) -> Result<$ret> {
                    self.$method(self.route(&format!(concat!("/api/v1/", $url), id))).await
                }
            }
         )*
    }

}
macro_rules! paged_routes_with_id {

    (($method:ident) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `", stringify!($method), " /api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set.",
                "\n",
                "```no_run",
                "use elefren::prelude::*;\n",
                "let data = Data::default();",
                "let client = Mastodon::from(data);\n",
                "client.", stringify!($name), "(\"some-id\");\n",
                "```"
            ),
            pub async fn $name(&self, id: &str) -> Result<Page<$ret>> {
                use log::{debug, error, as_debug};
                use uuid::Uuid;

                let call_id = Uuid::new_v4();
                let url = self.route(&format!(concat!("/api/v1/", $url), id));

                debug!(url = url, method = stringify!($method), call_id = as_debug!(call_id); "making API request");
                let response = self.client.$method(&url).send().await?;
                match response.error_for_status() {
                    Ok(response) => {
                        Page::new(self.clone(), response, call_id).await
                    }
                    Err(err) => {
                        error!(err = as_debug!(err), url = url, method = stringify!($method), call_id = as_debug!(call_id); "error making API request");
                        Err(err.into())
                    }
                }
            }
        }

        paged_routes_with_id!{$($rest)*}
    };

    () => {}
}
