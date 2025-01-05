# protect-salvo

<p align="center">
    <img alt="protect-salvo" src="https://github.com/DDtKey/protect-endpoints/raw/main/protect-salvo/logo.png">
</p>

> Authorization extension for [`salvo`] to protect your endpoints.

[![Crates.io Downloads Badge](https://img.shields.io/crates/d/protect-salvo)](https://crates.io/crates/protect-salvo)
[![crates.io](https://img.shields.io/crates/v/protect-salvo)](https://crates.io/crates/protect-salvo)
[![Documentation](https://docs.rs/protect-salvo/badge.svg)](https://docs.rs/protect-salvo)
![Apache 2.0 or MIT licensed](https://img.shields.io/crates/l/protect-salvo)

To check user access to specific services, you can use built-in `proc-macro` or manual.

The library can also be integrated with third-party solutions (e.g. jwt-middlewares).

## How to use

1. Declare your
   own [authority extractor](https://docs.rs/protect-endpoints-core/latest/protect_endpoints_core/authorities/extractor/trait.AuthoritiesExtractor.html)

The easiest way is to declare a function with the following signature (trait is already implemented for such Fn):

```rust,ignore
use salvo::prelude::*;

// You can use custom type instead of String
// It requires to use hyper's `Request` & `Response` types, because integration is based on `tower`
pub async fn extract(req: &mut salvo::hyper::Request<ReqBody>) -> Result<HashSet<String>, salvo::hyper::Response<ResBody>>
```

2. Add middleware to your application using the extractor defined in step 1

```rust,ignore
// You can use [`salvo_extra`] directly for Tower compatibility or re-exported one
use protect_salvo::salvo_extra::TowerLayerCompat;

Router::with_path("/")
    .hoop(GrantsLayer::with_extractor(extract).compat())
    .push(Router::with_path("/endpoint").get(your_handler))
```

> Steps 2 and 3 can be replaced by custom middleware or integration with another libraries.

3. Protect your endpoints in any convenient way from the examples below:

### Example of `proc-macro` way protection

```rust,ignore
#[protect_salvo::protect("ROLE_ADMIN")]
#[handler]
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

```rust,ignore
use enums::Role::{self, ADMIN};
use dto::User;

#[post("/info/{user_id}")]
#[protect_salvo::protect(any("ADMIN", expr = "user.is_super_user()"), ty = "Role")]
async fn admin_or_super_user(user: User) -> &'static str {
    "some secured response"
}
```

</details>

### Example of manual way protection

```rust,ignore
use protect_salvo::authorities::{AuthDetails, AuthoritiesCheck};

async fn manual_secure(details: AuthDetails) -> &'static str {
    if details.has_authority(ROLE_ADMIN) {
        return "ADMIN_RESPONSE";
    }
    "OTHER_RESPONSE"
}
```

You can find more [`examples`] in the git repository folder and [`documentation`].

## Supported `salvo` versions

* For `protect-salvo: 0.1.*` supported version of `salvo` is `0.70.*`
* For `protect-salvo: 0.2.*` supported version of `salvo` is `0.75.*`

[`examples`]: https://github.com/DDtKey/protect-endpoints/tree/main/protect-salvo/examples
[`documentation`]: https://docs.rs/protect-salvo
[`salvo`]: https://github.com/salvo-rs/salvo
[`salvo_extra`]: https://crates.io/crates/salvo_extra
