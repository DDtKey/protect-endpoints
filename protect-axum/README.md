# protect-axum

<p align="center">
    <img alt="protect-axum" src="https://github.com/DDtKey/protect-endpoints/raw/main/protect-axum/logo.png">
</p>

> Authorization extension for `axum` to protect your endpoints.

[![Crates.io Downloads Badge](https://img.shields.io/crates/d/protect-axum)](https://crates.io/crates/protect-axum)
[![crates.io](https://img.shields.io/crates/v/protect-axum)](https://crates.io/crates/protect-axum)
[![Documentation](https://docs.rs/protect-axum/badge.svg)](https://docs.rs/protect-axum)
![Apache 2.0 or MIT licensed](https://img.shields.io/crates/l/protect-axum)

To check user access to specific services, you can use built-in `proc-macro` or manual.

The library can also be integrated with third-party solutions (e.g. jwt-middlewares).

## How to use

1. Declare your
   own [authority extractor](https://docs.rs/protect-endpoints-core/latest/protect_endpoints_core/authorities/extractor/trait.AuthoritiesExtractor.html)

The easiest way is to declare a function with the following signature (trait is already implemented for such Fn):

```rust,ignore
use axum::extract::Request;
use axum::response::Response;

// You can use custom type instead of String
pub async fn extract(req: &mut Request) -> Result<HashSet<String>, Response>
```

2. Add middleware to your application using the extractor defined in step 1

```rust,ignore
Router::new()
    .route("/endpoint", get(your_handler))
    .layer(GrantsLayer::with_extractor(extract));
```

> Steps 1 and 2 can be replaced by custom middleware or integration with another libraries.

3. Protect your endpoints in any convenient way from the examples below:

### Example of `proc-macro` way protection

```rust,ignore
#[get("/secure")]
#[protect_axum::protect("OP_READ_SECURED_INFO")]
async fn macro_secured() -> &'static str {
    return "Hello, World!";
}
```

<details>

<summary> <b><i> Example of ABAC-like protection and custom authority type </i></b></summary>
<br/>


Here is an example using the `ty` and `expr` attributes. But these are independent features.

`expr` allows you to include some checks in the macro based on function params, it can be combined with authorities by
using `all`/`any`.

`ty` allows you to use a custom type for th authorities (then the middleware needs to be configured).
Take a look at an [enum-role example](examples/enum-role/main.rs)

```rust,ignore
use enums::Role::{self, ADMIN};
use dto::User;

#[get("/info/{user_id}")]
#[protect_axum::protect("ADMIN", expr = "user_id.into_inner() == user.id", ty = "Role")]
async fn macro_secured(Path(user_id): Path<i32>, Json(user): Json<User>) -> &'static str {
    "some secured response"
}

#[post("/info/{user_id}")]
#[protect_axum::protect(any("ADMIN", expr = "user.is_super_user()"), ty = "Role")]
async fn admin_or_super_user(Path(user_id): Path<i32>, Json(user): Json<User>) -> &'static str {
    "some secured response"
}
```

</details>

### Example of manual way protection

```rust,ignore
use protect_axum::authorities::{AuthDetails, AuthoritiesCheck};

async fn manual_secure(details: AuthDetails) -> &'static str {
    if details.has_authority(ROLE_ADMIN) {
        return "ADMIN_RESPONSE";
    }
    "OTHER_RESPONSE"
}
```

You can find more [`examples`] in the git repository folder and [`documentation`].

## Supported `axum` versions

* For `protect-axum: 0.1` supported version of `axum` is `0.7.*`
* For `protect-axum: 0.2.*` supported version of `axum` is `0.8.*`

[`examples`]: https://github.com/DDtKey/protect-endpoints/tree/main/protect-axum/examples
[`documentation`]: https://docs.rs/protect-axum
