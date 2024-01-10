# Leptos Macabre

```rust
use leptos_macabre::*;
fn main() {
    let class = "grim";
    let hecks = vec!["(~'o')~", "(~'_')~", "(◟ᅇ)◜"];
    let mut n = 0;

    let view = form! {
        @hx-boost="true";
        @action="/ghastly";
        @method="post";
        @class; // Shorthand attributes

        // Children are expressions -> impl IntoView.
        // Separate them with an optional comma.
        label! {
            "What scares you the most?",
            input! {
                @type="text";
                @name="fear";
                @value="Complexity";
            },
        }

        // Loops
        for_each! {
            heck in hecks => {
                p! { 
                    @id=format!("heck-{n}"); 
                    n += 1 // -> (): impl IntoView
                    { println!("this is also Ok") } // -> (): impl IntoView
                    heck
                }
            },
        },
        input! { @type="submit"; }
    };

    let expected = r#"
    <form hx-boost="true" action="/ghastly" method="post" class="grim">
        <label>
            What scares you the most?
            <input type="text" name="fear" value="Complexity"/>
        </label>
        <p id="heck-0">(~&#x27;o&#x27;)~</p>
        <p id="heck-1">(~&#x27;_&#x27;)~</p>
        <p id="heck-2">(◟ᅇ)◜</p>
        <input type="submit"/>
    </form>"# 
    .replace(['\n', '\t'], "")
    .replace("    ", "");

    assert_eq!(view.render_to_string().to_string(), expected);
}
```

## Syntax

### Attributes
Attributes start with "@" and end with ";". Shorthands omit the rhs.

```rust
use leptos_macabre::*;

let class = "spooky";

let a = div! { @class="spooky"; }.render_to_string().to_string();
let b = div! { @class; }.render_to_string().to_string(); 

assert_eq!(a, b, "<div class=\"spooky\"></div>");

```

### Child Elements
Children **must** be placed after any attributes. Any expression returning
`T: IntoView` is valid.

The match statement below does not need to call `.into_view()`
on each arm. The macro calls `into_view()` on each element after it is built.

```rust
use leptos_macabre::*;

enum Pet { Dog, Cat, }
let pet = Pet::Dog;

let view = p! { 
    "Free control flow!" 
    if false { println!("The compiler decides my fate"); }
    match pet {
        Pet::Dog => strong! { "Perfect" },
        Pet::Cat => p! { "Also Perfect" },
    }
};

assert_eq!(
    "<p>Free control flow!<strong>Perfect</strong></p>",
    view.render_to_string().to_string()
);

```

### Loops
The only extra "component" is the `for_each!` macro.

```rust 
use leptos_macabre::*;
let output = for_each! { n in 0..2 => { p! { n } } };
assert_eq!(
    "<p>0</p><p>1</p>", 
    output.render_to_string().to_string()
);

// Patterns are good!
for_each! {
    ref mut binding in [1, 2 ,3] => { }
};
```

## Dependencies
This crate re-exports the `leptos_dom` items it relies on. You do not need to
rely on any `leptos` crate directly unless you need something specific.

I am new to publishing. Feel free to PR if there is a more idiomatic way
to source items from withing a macro.

```toml
[dependencies]
leptos_macabre = "*"
```

## Goals
* Minimal syntax.
* Reduce the needed keystrokes for writing simple HTML on the server.
* Do not be fancy or complex.

## Non-Goals
* Supporting anything besides the bare minimum for simple HTML on the server.
* Reactivty.
* Rendering speed or efficiency.
* Component support beyond being able to call functions as children.

## Why a macro for each element?
I just want to type less. `macro_rules!` definitions are pretty negligible
as far as compile times go (AFAIK).

```rust, ignore
// a hypothetical macro
fview! { button { "Click me!" } }.into_view() // -> View
button! { "Click me!" } // -> View
```
It probably seems stupid but, a difference of 22 keystrokes means a lot.
