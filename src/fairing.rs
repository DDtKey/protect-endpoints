use crate::permissions::AttachPermissions;
use futures_core::future::BoxFuture;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};

type Extractor<Type> = Box<
    dyn for<'a> Fn(&'a mut Request<'_>) -> BoxFuture<'a, Option<Vec<Type>>> + Send + Sync + 'static,
>;

/// Built-in fairing for extracting user permission.
///
///
/// # Examples
/// ```
/// use rocket::{get, Route, Response, http::Status};
///
/// use rocket_grants::permissions::{AuthDetails, PermissionsCheck};
/// use rocket_grants::GrantsFairing;
///
/// #[rocket::launch]
/// fn rocket() -> _ {
///     rocket::build().mount("/api", rocket::routes![endpoint])
///         .attach(GrantsFairing::with_extractor_fn(|req| {
///             Box::pin(extract(req)) // example with a separate async function, but you can write a closure right here
///         }))
/// }
///
/// // Furthermore, you can use you own type instead of `String` (e.g. Enum).
/// async fn extract(_req: &rocket::Request<'_>) -> Option<Vec<String>> {
///    // Here is a place for your code to get user permissions/grants/permissions from a request (e.g. from a token or database).
///
///    // Stub example
///    Some(vec!["ROLE_ADMIN".to_string()])
/// }
///
/// // `has_permissions` is one of options to validate permissions.
/// // `proc-macro` crate has additional features, like ABAC security and custom types. See examples and `proc-macro` crate docs.
/// #[rocket_grants::has_permissions("ROLE_ADMIN")]
/// #[rocket::get("/")]
/// async fn endpoint() -> Status {
///     Status::Ok
/// }
/// ```
pub struct GrantsFairing<Type> {
    extractor: Extractor<Type>,
}

impl<Type: PartialEq + Clone + Send + Sync + 'static> GrantsFairing<Type> {
    /// Creating fairing using your permission extraction function.
    ///
    /// You can declare `async fn` with a suitable signature or you can write a boxed closure in-place (see examples below).
    ///
    /// # Examples
    /// ```
    /// use rocket_grants::GrantsFairing;
    ///  async fn example() {
    ///     let string_extractor = GrantsFairing::with_extractor_fn(|req| Box::pin(extract(req)));
    ///     let enum_extractor = GrantsFairing::with_extractor_fn(|req| Box::pin(extract_enum(req)));
    ///
    ///     let closure_extractor = GrantsFairing::with_extractor_fn(|req| Box::pin(async move {
    ///         Some(vec!["WRITE_ACCESS".to_string()])
    ///     }));
    /// }
    ///
    /// async fn extract(_req: &rocket::Request<'_>) -> Option<Vec<String>> {
    ///     // Here is a place for your code to get user permissions/grants/permissions from a request
    ///      // For example from a token or database
    ///     Some(vec!["WRITE_ACCESS".to_string()])
    /// }
    ///
    /// // Or with you own type:
    /// #[derive(PartialEq, Clone)] // required bounds
    /// enum Permission { WRITE, READ }
    /// async fn extract_enum(_req: &rocket::Request<'_>) -> Option<Vec<Permission>> {
    ///     // Here is a place for your code to get user permissions/grants/permissions from a request
    ///      // For example from a token, database or external service
    ///     Some(vec![Permission::WRITE])
    /// }
    /// ```
    ///
    pub fn with_extractor_fn<F: Send + Sync + 'static>(extractor_fn: F) -> Self
    where
        F: for<'a> Fn(&'a mut Request<'_>) -> BoxFuture<'a, Option<Vec<Type>>>,
    {
        Self {
            extractor: Box::new(extractor_fn),
        }
    }
}

#[rocket::async_trait]
impl<Type: PartialEq + Clone + Send + Sync + 'static> Fairing for GrantsFairing<Type> {
    fn info(&self) -> Info {
        Info {
            name: "Rocket-Grants Extractor",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, mut req: &mut Request<'_>, _data: &mut Data<'_>) {
        let permissions: Option<Vec<Type>> = (self.extractor)(req).await;
        req.attach(permissions);
    }
}
