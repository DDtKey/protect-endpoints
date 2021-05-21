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

### Example of `proc-macro` way protection
```rust
use actix_web_grants::proc_macro::{has_permissions};

#[get("/secure")]
#[has_permissions("OP_READ_SECURED_INFO")]
async fn macro_secured() -> HttpResponse {
    HttpResponse::Ok().body("ADMIN_RESPONSE")
}
```

### Example of `Guard` way protection 
```rust
use actix_web_grants::{PermissionGuard, GrantsMiddleware};

App::new()
    .wrap(GrantsMiddleware::with_extractor(extract))
    .service(web::resource("/admin")
            .to(|| async { HttpResponse::Ok().finish() })
            .guard(PermissionGuard::new("ROLE_ADMIN".to_string())))
    .service(web::resource("/admin") // in cases where you want to return 403 HTTP code
            .to(|| async { HttpResponse::Forbidden().finish() }))
```

### Example of `Guard` way protection for `Scope`
```rust
use actix_web_grants::{PermissionGuard, GrantsMiddleware};
use actix_web::http::header;

App::new()
    .wrap(GrantsMiddleware::with_extractor(extract))
    .service(
        web::scope("/admin")
            .service(web::resource("/users")
                .to(|| async { HttpResponse::Ok().finish() })
            ).guard(PermissionGuard::new("ROLE_ADMIN_ACCESS".to_string()))
    ).service(
        web::resource("/admin{regex:$|/.*?}")
        .to(|| async { 
            HttpResponse::TemporaryRedirect().append_header((header::LOCATION, "/login")).finish()
        }))
```
When `Guard` lets you in the `Scope` (meaning you have `"ROLE_ADMIN_ACCESS"`), the redirect will be unreachable for you. Even if you will request `/admin/some_undefined_page`.

Note: `regex` is a `Path` variable containing passed link.

### Example of manual way protection
```rust
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};

async fn manual_secure(details: AuthDetails) -> HttpResponse {
    if details.has_permission(ROLE_ADMIN) {
        return HttpResponse::Ok().body("ADMIN_RESPONSE");
    }
    HttpResponse::Ok().body("OTHER_RESPONSE")
}
```

You can find more [`examples`] in the git repository folder and [`documentation`].

[`actix-web-httpauth`]: https://github.com/DDtKey/actix-web-grants/blob/main/examples/jwt-httpauth
[`examples`]: https://github.com/DDtKey/actix-web-grants/tree/main/examples
[`documentation`]: https://docs.rs/actix-web-grants
