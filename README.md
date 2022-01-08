# actix-web-grants

<p align="center">
    <img alt="actix-web-grants" src="https://github.com/DDtKey/actix-web-grants/raw/main/logo.png">
</p>

> Extension for `actix-web` to validate user permissions.

[![CI](https://github.com/DDtKey/actix-web-grants/workflows/CI/badge.svg)](https://github.com/DDtKey/actix-web-grants/actions)
[![Crates.io Downloads Badge](https://img.shields.io/crates/d/actix-web-grants)](https://crates.io/crates/actix-web-grants)
[![crates.io](https://img.shields.io/crates/v/actix-web-grants)](https://crates.io/crates/actix-web-grants)
[![Documentation](https://docs.rs/actix-web-grants/badge.svg)](https://docs.rs/actix-web-grants)
[![dependency status](https://deps.rs/repo/github/DDtKey/actix-web-grants/status.svg)](https://deps.rs/repo/github/DDtKey/actix-web-grants)
![Apache 2.0 or MIT licensed](https://img.shields.io/crates/l/actix-web-grants)

To check user access to specific services, you can use built-in `proc-macro`, `PermissionGuard` or manual.

The library can also be integrated with third-party solutions (like [`actix-web-httpauth`]).


## How to use


1. Declare your own [permission extractor](./src/permissions/extractors.rs)
   
The easiest way is to declare a function with the following signature (trait is already implemented for such Fn):
```rust,ignore
use actix_web::{dev::ServiceRequest, Error};

// You can use custom type instead of String
async fn extract(req: &ServiceRequest) -> Result<Vec<String>, Error>
```

2. Add middleware to your application using the extractor defined in step 1
   
```rust,ignore
App::new()
    .wrap(GrantsMiddleware::with_extractor(extract))
```

> Steps 1 and 2 can be replaced by custom middleware or integration with another libraries. Take a look at an [jwt-httpauth example](./examples/jwt-httpauth/src/main.rs)

3. Protect your endpoints in any convenient way from the examples below:

### Example of `proc-macro` way protection
```rust,ignore
use actix_web_grants::proc_macro::{has_permissions};

#[get("/secure")]
#[has_permissions("OP_READ_SECURED_INFO")]
async fn macro_secured() -> HttpResponse {
    HttpResponse::Ok().body("ADMIN_RESPONSE")
}
```

<details>

<summary> <b><i> Example of ABAC-like protection and custom permission type </i></b></summary>
<br/>


Here is an example using the `type` and `secure` attributes. But these are independent features.

`secure` allows you to include some checks in the macro based on function params.

`type` allows you to use a custom type for the roles and permissions (then the middleware needs to be configured). 
Take a look at an [enum-role example](./examples/enum-role/src/main.rs)

```rust,ignore
use actix_web_grants::proc_macro::{has_role};
use enums::Role::{self, ADMIN};
use dto::User;

#[get("/info/{user_id}")]
#[has_role("ADMIN", type = "Role", secure = "user_id.into_inner() == user.id")]
async fn macro_secured(user_id: web::Path<i32>, user: web::Data<User>) -> HttpResponse {
    HttpResponse::Ok().body("some secured response")
}
```

</details>  


### Example of `Guard` way protection 
```rust,ignore
use actix_web_grants::{PermissionGuard, GrantsMiddleware};

App::new()
    .wrap(GrantsMiddleware::with_extractor(extract))
    .service(web::resource("/admin")
            .to(|| async { HttpResponse::Ok().finish() })
            .guard(PermissionGuard::new("ROLE_ADMIN".to_string())))
    .service(web::resource("/admin") // fallback endpoint if you want to return a 403 HTTP code 
            .to(|| async { HttpResponse::Forbidden().finish() }))
```

<details>

<summary> <b><i> Example of custom fallback endpoint for `Scope` with `Guard` </i></b></summary>
<br/>


Since `Guard` is intended only for routing, if the user doesn't have permissions, it returns a `404` HTTP code. But you can override the behavior like this:

```rust,ignore
use actix_web_grants::{PermissionGuard, GrantsMiddleware};
use actix_web::http::header;

App::new()
    .wrap(GrantsMiddleware::with_extractor(extract))
    .service(web::scope("/admin")
        .guard(PermissionGuard::new("ROLE_ADMIN_ACCESS".to_string()))
        .service(web::resource("/users")
            .to(|| async { HttpResponse::Ok().finish() }))
    ).service(
        web::resource("/admin{regex:$|/.*?}").to(|| async { 
            HttpResponse::TemporaryRedirect().append_header((header::LOCATION, "/login")).finish()
        }))
```
When `Guard` lets you in the `Scope` (meaning you have `"ROLE_ADMIN_ACCESS"`), the redirect will be unreachable for you. Even if you will request `/admin/some_undefined_page`.

Note: `regex` is a `Path` variable containing passed link.

</details>    

### Example of manual way protection
```rust,ignore
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};

async fn manual_secure(details: AuthDetails) -> HttpResponse {
    if details.has_permission(ROLE_ADMIN) {
        return HttpResponse::Ok().body("ADMIN_RESPONSE");
    }
    HttpResponse::Ok().body("OTHER_RESPONSE")
}
```

You can find more [`examples`] in the git repository folder and [`documentation`].

## Supported `actix-web` versions
* For `actix-web-grants: 2.*` supported version of `actix-web` is `3.*`
* For `actix-web-grants: 3.*` supported version of `actix-web` is `4.*`

[`actix-web-httpauth`]: https://github.com/DDtKey/actix-web-grants/blob/main/examples/jwt-httpauth
[`examples`]: https://github.com/DDtKey/actix-web-grants/tree/main/examples
[`documentation`]: https://docs.rs/actix-web-grants
