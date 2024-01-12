# Leptos Macabre

```rust, ignore
section! {
    h1!{"Id number: " id},
    details! {"Uploaded at: " upload_time}
    match upload.msg {
        Some(msg) => div! {
            p!{strong!{"Done"}}
            p!{"Report Type: " div!{@style="color: green"; msg}},
            p!{"Records Processed: " upload.records}
        },
        None => p!("Still processing, refresh to update"),
    }
}
```
# Syntax
* An **attribute** starts with `@` and ends with `;`.
* A **child** is any expression that returns `impl IntoView`.
* Children can be delimited with an *optional* comma `,`.
* Attribute values can be *ommited* when the identifier and names are the same.
* The macro will **not** handle shorthand attributes using raw identifiers.

That's it! You are ready to create some `leptos::View`s.

## Details & Conveniences

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
}
```

Each element is implemented as a separate macro. Kind of strange, yes. But,
it saves quite a bit on keystrokes and allows for much more modular snippets
of html.

```rust, ignore
use leptos::*;
use leptos_macabre::*;

view! { <button>"27 keystrokes"</button> }
button! { "10 keystrokes" }
```

This crate reexports the `leptos_dom` items that it relies on. If you do not
need anything specific from `leptos` you can rely on this crate alone. Although,
they have a ton of great things over there.

`leptos` components can be used with some caveats. If your component takes props
you will need to supply the generated `*Prop` struct as arguments when calling
your component. Otherwise, you can call the component like any other function.

Prop level attributes have not been tested but, I would not expect them to work.

```rust, ignore
use leptos_macabre::*;

#[leptos::component]
fn MyButton() -> impl leptos::IntoView { }

#[leptos::component]
fn MyA(href: &'static str) -> impl leptos::IntoView {}

p!{@id="cool-button"; MyButton};
p!{@id="cool-button"; MyButton()};
p!{@style="color: red"; MyA(MyAProps { href: "#red" })};
```

Since children are always just expressions, you can swap out components and
instead use regular functions in their place.

```rust, ignore
use leptos_macabre::*;

fn my_button() -> impl leptos_macabre::IntoView { }

fn my_a(href: &'static str) -> impl leptos_macabre::IntoView {}

p!(my_button, "nice button!"); // fn() pointers impl IntoView
p!(my_button(), div!("Nice!") ); // () impl IntoView
p!{@style="color: green"; my_a("#red")};
```

## Name
I always though the `horrorshow` crate was neat, and `leptos` is close to the
disease `leptospirosis`. It is just a little grim. Also, the macro that makes
this crate is truly horrendous.
