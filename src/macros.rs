macro_rules! methods {
    ($($method:ident,)+) => {
        $(
            doc_comment! {
                concat!("Make a ", stringify!($method), " API request, and deserialize the result into T"),
                async fn $method<T>(&self, url: impl AsRef<str>) -> Result<T>
                where T: for<'de> serde::Deserialize<'de> + serde::Serialize + std::fmt::Debug
                {
                    let url = url.as_ref();
                    tracing::debug!(method = stringify!($method), url, "making API request");
                    let response = self.authenticated(self.client.$method(url))
                        .header("Accept", "application/json")
                        .send()
                        .await?;
                    read_response(response).await
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
            "use mastodon_async::prelude::*;\n",
            "let data = Data::default();\n",
            "let client = Mastodon::from(data);\n",
            "client.", stringify!($name), "();\n",
            "```"
            ),
            #[tracing::instrument(skip(self), fields(call_id = %uuid::Uuid::new_v4()))]
            pub async fn $name(&self) -> Result<Page<$ret>> {
                let url = self.route(concat!("/api/v1/", $url));
                tracing::debug!(method = stringify!($method), url, "making API request");

                let response = self
                    .authenticated(self.client.$method(&url))
                    .header("Accept", "application/json")
                    .send()
                    .await?;

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
            #[tracing::instrument(skip_all, fields(call_id = %uuid::Uuid::new_v4()))]
            pub async fn $name<'a>(&self, $($param: $typ,)*) -> Result<Page<$ret>> {
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

                #[allow(clippy::redundant_field_names)]
                let qs_data = Data {
                    $(
                            $param: $param,
                    )*
                    _marker: ::std::marker::PhantomData,
                };

                let qs = serde_urlencoded::to_string(&qs_data)?;
                let url = format!(concat!("/api/v1/", $url, "?{}"), &qs);
                tracing::debug!(method = "get", url, "making API request");
                let response = self.authenticated(self.client.get(&url))
                    .header("Accept", "application/json")
                    .send()
                    .await?;
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
            #[tracing::instrument(skip(self), fields(call_id = %Uuid::new_v4()))]
            pub async fn $name<'a>(&self, $($param: $typ,)*) -> Result<$ret> {
                #[derive(Serialize)]
                struct Data<'a> {
                    $(
                        $param: $typ,
                    )*
                    #[serde(skip)]
                    _marker: ::std::marker::PhantomData<&'a ()>,
                }

                #[allow(clippy::redundant_field_names)]
                let qs_data = Data {
                    $(
                            $param: $param,
                    )*
                    _marker: ::std::marker::PhantomData,
                };

                let qs = serde_urlencoded::to_string(&qs_data)?;
                tracing::debug!(query_string_data = ?qs, "URL-encoded data to be sent in API request");
                let url = format!(concat!("/api/v2/", $url, "?{}"), &qs);
                self.get(self.route(&url)).await
            }
        }

        route_v2!{$($rest)*}
    };

    ((post multipart with description ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `post /api/v2/",
                $url,
                "`, with a description/alt-text.",
                "\n# Errors\nIf `access_token` is not set."),
            #[tracing::instrument(skip_all, fields(call_id = %Uuid::new_v4()))]
            pub async fn $name(&self $(, $param: $typ)*, description: Option<String>) -> Result<$ret> {
                let form_data = reqwest::multipart::Form::new()
                    $(
                        .part(stringify!($param), Self::get_form_part($param)?)
                     )*;

                let form_data = if let Some(description) = description {
                    form_data.text("description", description)
                } else { form_data };

                let url = &self.route(concat!("/api/v2/", $url));
                tracing::debug!(method = "post", url, multipart_form_data = ?form_data, "making API request" );

                let response = self.authenticated(self.client.post(url))
                    .multipart(form_data)
                    .header("Accept", "application/json")
                    .send()
                    .await?;
                read_response(response).await
            }
        }
        route_v2! { $($rest)* }
    };


    ((post multipart ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `post /api/v2/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."),
            #[tracing::instrument(skip(self), fields(call_id = %uuid::Uuid::new_v4()))]
            pub async fn $name(&self, $($param: $typ,)*) -> Result<$ret> {
                let form_data = reqwest::multipart::Form::new()
                    $(
                        .part(stringify!($param), Self::get_form_part($param)?)
                     )*;

                let url = &self.route(concat!("/api/v2/", $url));
                tracing::debug!(method = "post", url, multipart_form_data = ?form_data, "making API request" );
                let response = self.authenticated(self.client.post(url))
                    .multipart(form_data)
                    .header("Accept", "application/json")
                    .send()
                    .await?;
                read_response(response).await
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
            #[tracing::instrument(skip(self), fields(call_id = %uuid::Uuid::new_v4()))]
            pub async fn $name(&self, $($param: $typ,)*) -> Result<$ret> {
                let url = &self.route(concat!("/api/v1/", $url));
                let form_data = reqwest::multipart::Form;Form::new()
                    $(
                        .part(stringify!($param), Self::get_form_part($param)?)
                     )*;

                tracing::debug!(method = "post", url, multipart_form_data = ?form_data, "making API request");
                let response = self.authenticated(self.client.post(url))
                    .multipart(form_data)
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                read_response(response).await
            }
        }

        route!{$($rest)*}
    };

    ((post multipart with description ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `post /api/v1/",
                $url,
                "`, with a description/alt-text.",
                "\n# Errors\nIf `access_token` is not set."),
            #[tracing::instrument(skip(self), fields(call_id = %uuid::Uuid::new_v4()))]
            pub async fn $name(&self $(, $param: $typ)*, description: Option<String>) -> Result<$ret> {
                let form_data = reqwest::multipart::Form::new()
                    $(
                        .part(stringify!($param), Self::get_form_part($param)?)
                     )*;

                let form_data = if let Some(description) = description {
                    form_data.text("description", description)
                } else { form_data };

                let url = &self.route(concat!("/api/v1/", $url));
                debug!(method = "post", url, multipart_form_data = ?form_data, "making API request");

                let response = self.authenticated(self.client.post(url))
                    .multipart(form_data)
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                read_response(response).await
            }
        }
        route! { $($rest)* }
    };

    ((get ($($param:ident: $typ:ty,)*)) $name:ident: $url:expr => $ret:ty, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                "Equivalent to `get /api/v1/",
                $url,
                "`\n# Errors\nIf `access_token` is not set."
            ),
            #[tracing::instrument(skip(self), fields(call_id = %Uuid::new_v4()))]
            pub async fn $name<'a>(&self, $($param: $typ,)*) -> Result<$ret> {
                #[derive(Serialize)]
                struct Data<'a> {
                    $(
                        $param: $typ,
                    )*
                    #[serde(skip)]
                    _marker: ::std::marker::PhantomData<&'a ()>,
                }

                #[allow(clippy::redundant_field_names)]
                let qs_data = Data {
                    $(
                            $param: $param,
                    )*
                    _marker: ::std::marker::PhantomData,
                };

                let qs = serde_urlencoded::to_string(&qs_data)?;
                tracing::debug!(query_string_data = ?qs, "URL-encoded data to be sent in API request");
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
            #[tracing::instrument(skip_all, fields(call_id = %Uuid::new_v4()))]
            pub async fn $name(&self, $($param: $typ,)*) -> Result<$ret> {
                let form_data = json!({
                    $(
                        stringify!($param): $param,
                    )*
                });
                let url = &self.route(concat!("/api/v1/", $url));
                debug!(method = stringify!($method), url = url.as_str(), ?form_data, "making API request");
                let response = self.authenticated(self.client.$method(url))
                    .json(&form_data)
                    .header("Accept", "application/json")
                    .send()
                    .await?;

                read_response(response).await
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
                "use mastodon_async::prelude::*;\n",
                "let data = Data::default();\n",
                "let client = Mastodon::from(data);\n",
                "client.", stringify!($name), "();\n",
                "```"
            ),
            #[tracing::instrument(skip_all, fields(call_id = %Uuid::new_v4()))]
            pub async fn $name(&self) -> Result<$ret> {
                self.$method(self.route(concat!("/api/v1/", $url))).await
            }
        }

        route!{$($rest)*}
    };

    () => {}
}

macro_rules! route_id {

    ($(($method:ident) $name:ident[$id_type:ty]: $url:expr => $ret:ty,)*) => {
        $(
            doc_comment! {
                concat!(
                    "Equivalent to `", stringify!($method), " /api/v1/",
                    $url,
                    "`\n# Errors\nIf `access_token` is not set.",
                    "\n",
                    "```no_run",
                    "use mastodon_async::prelude::*;\n",
                    "let data = Data::default();\n",
                    "let client = Mastodon::from(data);\n",
                    "client.", stringify!($name), "(\"42\");\n",
                    "#   Ok(())\n",
                    "# }\n",
                    "```"
                ),
                #[tracing::instrument(skip_all, fields(call_id = %Uuid::new_v4()))]
                pub async fn $name(&self, id: &$id_type) -> Result<$ret> {
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
                "use mastodon_async::prelude::*;\n",
                "let data = Data::default();",
                "let client = Mastodon::from(data);\n",
                "client.", stringify!($name), "(\"some-id\");\n",
                "```"
            ),
            #[tracing::instrument(skip_all, fields(call_id = %Uuid::new_v4()))]
            pub async fn $name(&self, id: impl AsRef<str>) -> Result<Page<$ret>> {
                let url = self.route(&format!(concat!("/api/v1/", $url), id.as_ref()));
                tracing::debug!(method = stringify!($method), url, "making API request");
                let response = self.authenticated(self.client.$method(&url)).header("Accept", "application/json").send().await?;
                Page::new(self.clone(), response).await
            }
        }

        paged_routes_with_id!{$($rest)*}
    };

    () => {}
}

macro_rules! streaming {
    ($desc:tt $fn_name:ident@$stream:literal, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                $desc,
                "\n\nExample:\n\n",
                "
use mastodon_async::prelude::*;
use mastodon_async::entities::event::Event;
use futures_util::{pin_mut, StreamExt, TryStreamExt};

tokio_test::block_on(async {
    let data = Data::default();
    let client = Mastodon::from(data);
    let stream = client.",
                    stringify!($fn_name),
                    "().await.unwrap();
    stream.try_for_each(|event| async move {
        match event {
            Event::Update(ref status) => { /* .. */ },
            Event::Notification(ref notification) => { /* .. */ },
            Event::Delete(ref id) => { /* .. */ },
            Event::FiltersChanged => { /* .. */ },
        }
        Ok(())
    }).await.unwrap();
});"
            ),
            #[tracing::instrument(skip(self), fields(call_id = %Uuid::new_v4()))]
            pub async fn $fn_name(&self) -> Result<impl TryStream<Ok=(Event, Mastodon), Error=Error> + '_> {
                let url = self.route(&format!("/api/v1/streaming/{}", $stream));
                let response = self.authenticated(self.client.get(&url))
                    .header("Accept", "application/json")
                    .send()
                    .await?;
                debug!(response = as_value!(response, Response), "received API response");
                let status = response.status();
                if status.is_success() {
                     Ok($crate::event_stream::event_stream(response, url, self))
                } else {
                    let response = response.json().await?;
                    Err(Error::Api{ status, response })
                }
            }
        }
        streaming! { $($rest)* }
    };
    ($desc:tt $fn_name:ident($param:ident: $param_type:ty, like $param_doc_val:literal)@$stream:literal, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                $desc,
                "\n\nExample:\n\n",
                "
use mastodon_async::prelude::*;
use mastodon_async::entities::event::Event;
use futures_util::{pin_mut, StreamExt, TryStreamExt};

tokio_test::block_on(async {
    let data = Data::default();
    let client = Mastodon::from(data);
    let stream = client.",
                    stringify!($fn_name),
                    "(",
                    $param_doc_val,
                    ").await.unwrap();
    stream.try_for_each(|event| async move {
        match event {
            Event::Update(ref status) => { /* .. */ },
            Event::Notification(ref notification) => { /* .. */ },
            Event::Delete(ref id) => { /* .. */ },
            Event::FiltersChanged => { /* .. */ },
        }
        Ok(())
    }).await.unwrap();
});"
            ),
            #[tracing::instrument(skip_all, fields(call_id = %Uuid::new_v4()))]
            pub async fn $fn_name(&self, $param: $param_type) -> Result<impl TryStream<Ok=(Event, Mastodon), Error=Error> + '_> {
                let mut url: Url = self.route(concat!("/api/v1/streaming/", $stream)).parse()?;
                url.query_pairs_mut().append_pair(stringify!($param), $param.as_ref());
                let url = url.to_string();
                let response = self.authenticated(self.client.get(url.as_str())).header("Accept", "application/json").send().await?;
                debug!(response = as_value!(response, Response), "received API response");
                let status = response.status();
                if status.is_success() {
                     Ok($crate::event_stream::event_stream(response, url, self))
                } else {
                    let response = response.json().await?;
                    Err(Error::Api{ status, response })
                }
            }
        }
        streaming! { $($rest)* }
    };
    ($desc:tt $fn_name:ident(flag $param:ident)@$stream:literal, $($rest:tt)*) => {
        doc_comment! {
            concat!(
                $desc,
                "\n\nExample:\n\n",
                "
use mastodon_async::prelude::*;
use mastodon_async::entities::event::Event;
use futures_util::{pin_mut, StreamExt, TryStreamExt};

tokio_test::block_on(async {
    let data = Data::default();
    let client = Mastodon::from(data);
    let stream = client.",
                    stringify!($fn_name),
                    "(false).await.unwrap();
    stream.try_for_each(|event| async move {
        match event {
            Event::Update(ref status) => { /* .. */ },
            Event::Notification(ref notification) => { /* .. */ },
            Event::Delete(ref id) => { /* .. */ },
            Event::FiltersChanged => { /* .. */ },
        }
        Ok(())
    }).await.unwrap();
});"
            ),
            #[tracing::instrument(skip(self), fields(call_id = %Uuid::new_v4()))]
            pub async fn $fn_name(&self, $param: bool) -> Result<impl TryStream<Ok=(Event, Mastodon), Error=Error> + '_> {
                let mut url: Url = self.route(concat!("/api/v1/streaming/", $stream)).parse()?;
                if $param {
                    url.query_pairs_mut().append_key_only(stringify!($param));
                }
                let url = url.to_string();
                let response = self.authenticated(self.client.get(url.as_str())).header("Accept", "application/json").send().await?;
                debug!(response = as_value!(response, Response), "received API response");
                let status = response.status();
                if status.is_success() {
                     Ok(crate::event_stream::event_stream(response, url, self))
                } else {
                    let response = response.json().await?;
                    Err(Error::Api{ status, response })
                }
            }
        }
        streaming! { $($rest)* }
    };
    () => {}
}
