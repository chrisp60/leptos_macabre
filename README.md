# Leptos Macabre

Scary simple server-side rendering. Powered by [Leptos](https://github.com/leptos-rs/leptos).

```rust
use leptos_macabre::*;

let result: Result<_, &str> = Ok("Done!");
let style = "color: green;";

section! {
    h1!{ "Uploads" },
    match result {
        Ok(msg) => div! {
            @htmx-get="/uploads";
            @style;
            p!{ "Status: ", msg },
        },
        Err(err) => strong! {
            @class="error";
            "Uh Oh: " err
        },
    }
};
```

## Syntax

- **Attributes** starts with `@` and end with `;`.
- **Attributes Values** are expressions returning `impl leptos::IntoAttribute`.
- **Attributes Values** can be shorthanded when the name equals a variable in scope.
  Raw identifiers (i.e. `r#type`) are not supported.
- **Children** are expressions returning `impl leptos::IntoView`.
- **Children** can be delimited with an _optional_ comma (helps rustfmt and auto-indenting, sometimes).

## Details & Conveniences

### Compile Time & Lsp
Due to everything being a minimally recursive `macro_rules!`, auto-complete and suggestions
are incredibly snappy.

### Auto `.into_view()`

Every macro in this crate returns a `View`, this intentionally diverts from
leptos to not require `.into_view()` on each arm (an understandable requirement).

```rust
use leptos_macabre::*;
let result = Some(true);
let href = "#section";

match result {
    Some(true) => a!(@href; "Go here!"),
    Some(false) => p!(@class="bad"; "Uh-oh!"),
    None => div!(),
};
```

### Every Element a Macro.

Each element is a separate macro. This saves many keystrokes. It is different
from most Rust HTML macros.

```rust, ignore
use leptos::*;
use leptos_macabre::*;

view! { <button>"27 keystrokes"</button> }
button! { "10 keystrokes" }
```

### Re-Export

This crate exports the `leptos_dom` items needed to work. If you just want
to make HTML on the server, you can skip adding `leptos` as its own dependency.

## Drawbacks

No reactivity that can't be implemented with a `script!` or normal html event
handlers. No signals. Performance will probably be worse when compared directly
to `leptos::view!`.

`leptos` components with props are minimally supported. The generated `Prop*`
struct would need to be provided as an argument, meaning you skip
many of the conveniences of using `#[component]` in the first place (optional
props etc).

```rust, ignore
use leptos_macabre::*;
use leptos::component;

#[component]
fn MyButton() -> impl leptos::IntoView {}

#[component]
fn MyA(href: &'static str) -> impl leptos::IntoView {}

p!{ MyButton };
p!{ MyButton() };
p!{ MyA(MyAProps { href: "#red" }) };

```

Since children are just expressions returning `impl IntoView`, you can swap out
components with boring old functions.

```rust, ignore
use leptos_macabre::*;

fn my_button() -> impl leptos::IntoView { }
fn my_a(href: &'static str) -> impl leptos::IntoView {}
fn my_b(inner: Option<bool>) -> impl leptos::IntoView {}

div! { 
    my_button,
    my_button(),
    my_a("#red"),
    my_b(Some(true)) ,
};

```

## Name

I always thought the `horrorshow` crate was neat, and `leptos` is close to the
disease *leptospirosis*. It is all just a little grim. Secondly, this crate is
created with nested `macro_rules!`, something truly horrendous.
