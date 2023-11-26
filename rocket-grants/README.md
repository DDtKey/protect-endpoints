# rocket-grants

<p align="center">
    <img alt="rocket-grants" src="https://github.com/DDtKey/rocket-grants/raw/main/logo.png">
</p>

> Extension for [`rocket`] to authorize requests.

[![CI](https://github.com/DDtKey/rocket-grants/workflows/CI/badge.svg)](https://github.com/DDtKey/rocket-grants/actions)
[![Crates.io Downloads Badge](https://img.shields.io/crates/d/rocket-grants)](https://crates.io/crates/rocket-grants)
[![crates.io](https://img.shields.io/crates/v/rocket-grants)](https://crates.io/crates/rocket-grants)
[![Documentation](https://docs.rs/rocket-grants/badge.svg)](https://docs.rs/rocket-grants)
[![dependency status](https://deps.rs/repo/github/DDtKey/rocket-grants/status.svg)](https://deps.rs/repo/github/DDtKey/rocket-grants)
![Apache 2.0 or MIT licensed](https://img.shields.io/crates/l/rocket-grants)

To check user access to specific endpoints, you can use built-in `proc-macro` or do it manually.

Provides a complete analogue of the [`actix-web-grants`] and [`poem-grants`].

## How to use


1. Declare your own authorities extraction function
   
The easiest way is to declare a function with the following signature:
```rust,ignore
// You can use custom type instead of String
async fn extract(req: &rocket::Request<'_>) -> Option<HashSet<String>>
```

2. Add fairing to your application using the extraction function defined in step 1
   
```rust,ignore
    rocket::build().mount("/api", rocket::routes![endpoint])
         .attach(GrantsFairing::with_extractor_fn(|req| {
             Box::pin(extract(req)) // example with a separate async function `extract`, but you can write a closure right here
         }))
```

> Steps 1 and 2 can be replaced by integration with your custom fairing.

3. Protect your endpoints in any convenient way from the examples below:

### Example of `proc-macro` way protection
```rust,no_run
#[rocket_grants::protect("OP_READ_SECURED_INFO")]
#[rocket::get("/")]
async fn macro_secured() -> &'static str {
   "ADMIN_RESPONSE"
}
```

<details>

<summary> <b><i> Example of ABAC-like protection and custom permission type </i></b></summary>
<br/>


Here is an example using the `ty` and `expr` attributes. But these are independent features.

`expr` allows you to include some checks in the macro based on function params.

`ty` allows you to use a custom type for the authority (then the fairing needs to be configured). 
Take a look at an [enum-role example](../examples/enum-role/src/main.rs)

```rust,ignore
use enums::Role::{self, ADMIN};
use dto::User;

#[rocket_grants::protect("USER", expr = "user_id == user.id")]
#[rocket::post("/secure/<user_id>", data = "<user>")]
async fn role_macro_secured_with_params(user_id: i32, user: Json<User>) -> &'static str {
   "some secured info with parameters"
}
```

</details>  

### Example of manual way protection
```rust,no_run
use rocket_grants::authorities::{AuthDetails, AuthoritiesCheck};

#[rocket::get("/")]
async fn manual_secure(details: AuthDetails) -> &'static str {
    if details.has_authority("ROLE_ADMIN") {
        return "ADMIN_RESPONSE"
    }
    "OTHER_RESPONSE"
}
```

You can find more [`examples`] in the git repository folder and [`documentation`].

# Error customization

Custom error responses can be specified using Rocket catchers. See the Rocket documentation for [catchers](https://doc.rust-lang.org/cargo/commands/cargo-doc.html).

You can set up custom responses for:

`401 Unauthorized` - when it wasn't possible to obtain authorization data from the request in your extractor.

`403 Forbidden` - when the permissions did not match the specified for the endpoint.


## Supported `rocket` versions
* For `rocket-grants: 0.1.*` supported version of `rocket` is `0.5.*`

[`rocket`]: https://github.com/SergioBenitez/Rocket
[`examples`]: https://github.com/DDtKey/rocket-grants/tree/main/examples
[`documentation`]: https://docs.rs/rocket-grants
[`poem-grants`]: https://github.com/DDtKey/poem-grants
[`actix-web-grants`]: https://github.com/DDtKey/actix-web-grants
