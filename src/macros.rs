macro_rules! methods {
    ($($method:ident,)+) => {
        $(
            async fn $method<T: for<'de> serde::Deserialize<'de>>(&self, url: impl AsRef<str>) -> Result<T>
            {
                let url = url.as_ref();
                Ok(
                    self.client
                        .$method(url)
                        .send()
                        .await?
                        .error_for_status()?
                        .json()
                        .await?
                )
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
                let url = self.route(concat!("/api/v1/", $url));
                let response = self.client.$method(&url).send().await?;

                Page::new(self.clone(), response).await
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

                let response = self.client.get(&url).send().await?;

                Page::new(self.clone(), response).await
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

                let url = format!(concat!("/api/v2/", $url, "?{}"), &qs);

                self.get(self.route(&url)).await
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

                let form_data = Form::new()
                    $(
                        .part(stringify!($param), {
                            let mut file = std::fs::File::open($param.as_ref())?;
                            let mut data = if let Ok(metadata) = file.metadata() {
                                Vec::with_capacity(metadata.len().try_into()?)
                            } else {
                                vec![]
                            };
                            file.read_to_end(&mut data)?;
                            Part::bytes(data)
                        })
                     )*;

                let response = self.client
                    .post(&self.route(concat!("/api/v1/", $url)))
                    .multipart(form_data)
                    .send()
                    .await?;

                let status = response.status().clone();

                if status.is_client_error() {
                    return Err(Error::Client(status));
                } else if status.is_server_error() {
                    return Err(Error::Server(status));
                }

                Ok(response.json().await?)
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

                let url = format!(concat!("/api/v1/", $url, "?{}"), &qs);

                self.get(self.route(&url)).await
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

                let form_data = json!({
                    $(
                        stringify!($param): $param,
                    )*
                });

                let response = self.client
                    .$method(&self.route(concat!("/api/v1/", $url)))
                    .json(&form_data)
                    .send()
                    .await?;

                let status = response.status().clone();

                if status.is_client_error() {
                    return Err(Error::Client(status));
                } else if status.is_server_error() {
                    return Err(Error::Server(status));
                }

                Ok(response.json().await?)
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
                let url = self.route(&format!(concat!("/api/v1/", $url), id));
                let response = self.client.$method(&url).send().await?;

                Page::new(self.clone(), response).await
            }
        }

        paged_routes_with_id!{$($rest)*}
    };

    () => {}
}
