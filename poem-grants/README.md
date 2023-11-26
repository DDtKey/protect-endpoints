# poem-grants

<p align="center">
    <img alt="poem-grants" src="https://github.com/DDtKey/poem-grants/raw/main/logo.png">
</p>

> Extension for [`poem`] to validate user permissions.

[![CI](https://github.com/DDtKey/poem-grants/workflows/CI/badge.svg)](https://github.com/DDtKey/poem-grants/actions)
[![Crates.io Downloads Badge](https://img.shields.io/crates/d/poem-grants)](https://crates.io/crates/poem-grants)
[![crates.io](https://img.shields.io/crates/v/poem-grants)](https://crates.io/crates/poem-grants)
[![Documentation](https://docs.rs/poem-grants/badge.svg)](https://docs.rs/poem-grants)
[![dependency status](https://deps.rs/repo/github/DDtKey/poem-grants/status.svg)](https://deps.rs/repo/github/DDtKey/poem-grants)
![Apache 2.0 or MIT licensed](https://img.shields.io/crates/l/poem-grants)

To check user access to specific services, you can use built-in `proc-macro`, `PermissionGuard` or manual.

The library can also be integrated with third-party solutions or your custom middlewares (like [`jwt-auth`] example).

Provides a complete analogue of the [`actix-web-grants`].

**NOTE**: Even under `beta` flag it's ready-to-use library. However, I'm going to prepare large update of whole `*-grants` ecosystem with additional features soon. 


## How to use


1. Declare your own [permission extractor](srcermissions/extractors.rs)
   
The easiest way is to declare a function with the following signature (trait is already implemented for such Fn):
```rust,ignore
// You can use custom type instead of String
async fn extract(req: &poem::Request) -> poem::Result<Vec<String>>
```

2. Add middleware to your application using the extractor defined in step 1
   
```rust,ignore
Route::new()
    .at("/endpoint", your_endpoint)
    .with(GrantsMiddleware::with_extractor(extract))
```

> Steps 1 and 2 can be replaced by custom middleware or integration with another libraries. Take a look at an [jwt-auth example](../examples/poem/jwt-auth/src/main.rs)

3. Protect your endpoints in any convenient way from the examples below:

### Example of `proc-macro` way protection
```rust,no_run
use poem::{Response, http::StatusCode};

#[poem_grants::has_permissions("OP_READ_SECURED_INFO")]
#[poem::handler]
async fn macro_secured() -> Response {
    Response::builder().status(StatusCode::OK).body("ADMIN_RESPONSE")
}
```

Or for `poem-openapi`:
```rust,no_run
use poem_openapi::{OpenApi, payload::PlainText};

struct Api;

#[poem_grants::open_api] // It's important to keep above of `OpenApi`
#[OpenApi]
impl Api {
    #[has_permissions("OP_READ_ADMIN_INFO")]
    #[oai(path = "/admin", method = "get")]
    async fn macro_secured(&self) -> PlainText<String> {
        PlainText("ADMIN_RESPONSE".to_string())
    }
}
```

<details>

<summary> <b><i> Example of ABAC-like protection and custom permission type </i></b></summary>
<br/>


Here is an example using the `type` and `secure` attributes. But these are independent features.

`secure` allows you to include some checks in the macro based on function params.

`type` allows you to use a custom type for the roles and permissions (then the middleware needs to be configured). 
Take a look at an [enum-role example](../examples/poem/enum-role/src/main.rs)

```rust,ignore
use poem::{Response, http::StatusCode, web};
use enums::Role::{self, ADMIN};
use dto::User;

#[poem_grants::has_role("ADMIN", type = "Role", secure = "*user_id == user.id")]
#[poem::handler]
async fn macro_secured(user_id: web::Path<i32>, user: web::Data<User>) -> Response {
    Response::builder().status(StatusCode::OK).body("some secured response")
}
```

</details>  

### Example of manual way protection
```rust,no_run
use poem::{Response, http::StatusCode};
use poem_grants::permissions::{AuthDetails, PermissionsCheck};

#[poem::handler]
async fn manual_secure(details: AuthDetails) -> Response {
    if details.has_permission("ROLE_ADMIN") {
        return Response::builder().status(StatusCode::OK).body("ADMIN_RESPONSE");
    }
    Response::builder().status(StatusCode::OK).body("OTHER_RESPONSE")
}
```

You can find more [`examples`] in the git repository folder and [`documentation`].

## Supported `poem` versions
* For `poem-grants: 1.*` supported version of `poem` is `1.*`

[`jwt-auth`]: https://github.com/DDtKey/poem-grants/blob/main/examples/jwt-auth
[`examples`]: https://github.com/DDtKey/poem-grants/tree/main/examples
[`documentation`]: https://docs.rs/poem-grants
[`poem`]: https://github.com/poem-web/poem
[`poem-openapi`]: https://github.com/poem-web/poem/tree/master/poem-openapi
[`actix-web-grants`]: https://github.com/DDtKey/actix-web-grants
