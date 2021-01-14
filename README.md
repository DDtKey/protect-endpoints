# actix-web-grants
> A crate for validate user authorities in `actix-web`.


[![crates.io](https://img.shields.io/crates/v/actix-web-grants)](https://crates.io/crates/actix-web-grants)
[![Documentation](https://docs.rs/actix-web-grants/badge.svg)](https://docs.rs/actix-web-grants)
![Apache 2.0 or MIT licensed](https://img.shields.io/crates/l/actix-web-grants)

To check user access to specific services, you can use built-in `proc-macro`, `AuthorityGuard` or manual.

The library can also be integrated with third-party solutions (like [`actix-web-httpauth`]).

### Example of protection via `actix_web_grants::proc-macro`
```rust
#[get("/admin")]
#[has_authorities("ROLE_ADMIN")]
async fn macro_secured() -> HttpResponse {
    HttpResponse::Ok().body(ADMIN_RESPONSE)
}
```

### Example of protection via trait `Guard`
```rust
App::new()
    .wrap(GrantsMiddleware::fn_extractor(extract))
    .service(web::resource("/admin")
            .to(|| async { HttpResponse::Ok().finish() })
            .guard(AuthorityGuard::new("ROLE_ADMIN".to_string())))
```

You can find more [`examples`] in the git repository folder and [`documentation`].

[`actix-web-httpauth`]: https://github.com/DDtKey/actix-web-grants/blob/main/examples/integration-httpauth.rs
[`examples`]: https://github.com/DDtKey/actix-web-grants/tree/main/examples
[`documentation`]: https://docs.rs/actix-web-grants

