# actix-web-grants
> Extension for `actix-web` to validate user authorities.

[![CI](https://github.com/DDtKey/actix-web-grants/workflows/CI/badge.svg)](https://github.com/DDtKey/actix-web-grants/actions)
[![crates.io](https://img.shields.io/crates/v/actix-web-grants)](https://crates.io/crates/actix-web-grants)
[![Documentation](https://docs.rs/actix-web-grants/badge.svg)](https://docs.rs/actix-web-grants)
[![dependency status](https://deps.rs/repo/github/DDtKey/actix-web-grants/status.svg)](https://deps.rs/repo/github/DDtKey/actix-web-grants)
![Apache 2.0 or MIT licensed](https://img.shields.io/crates/l/actix-web-grants)

To check user access to specific services, you can use built-in `proc-macro`, `AuthorityGuard` or manual.

The library can also be integrated with third-party solutions (like [`actix-web-httpauth`]).

### Example of `proc-macro` way protection
```rust
use actix_web_grants::proc_macro::{has_authorities};

#[get("/admin")]
#[has_authorities("ROLE_ADMIN")]
async fn macro_secured() -> HttpResponse {
    HttpResponse::Ok().body("ADMIN_RESPONSE")
}
```

### Example of `Guard` way protection 
```rust
use actix_web_grants::{AuthorityGuard, GrantsMiddleware};

App::new()
    .wrap(GrantsMiddleware::with_extractor(extract))
    .service(web::resource("/admin")
            .to(|| async { HttpResponse::Ok().finish() })
            .guard(AuthorityGuard::new("ROLE_ADMIN".to_string())))
```

### Example of manual way protection
```rust
use actix_web_grants::authorities::{AuthDetails, AuthoritiesCheck};

async fn manual_secure(details: AuthDetails) -> HttpResponse {
    if details.has_authority(ROLE_ADMIN) {
        return HttpResponse::Ok().body("ADMIN_RESPONSE");
    }
    HttpResponse::Ok().body("OTHER_RESPONSE")
}
```

You can find more [`examples`] in the git repository folder and [`documentation`].

[`actix-web-httpauth`]: https://github.com/DDtKey/actix-web-grants/blob/main/examples/integration-httpauth.rs
[`examples`]: https://github.com/DDtKey/actix-web-grants/tree/main/examples
[`documentation`]: https://docs.rs/actix-web-grants

