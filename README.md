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
```

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

### Supported `actix-web` versions
* For `actix-web-grants: 2.*` supported version of `actix-web` is `3.*`
* For `actix-web-grants: 3.*` supported version of `actix-web` is `4.*`

[`actix-web-httpauth`]: https://github.com/DDtKey/actix-web-grants/blob/main/examples/jwt-httpauth
[`examples`]: https://github.com/DDtKey/actix-web-grants/tree/main/examples
[`documentation`]: https://docs.rs/actix-web-grants

