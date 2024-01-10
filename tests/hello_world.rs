#[test]
fn hello_world() {
    let id = "affirmation"; // Shorthand attributes
    let text = leptos_macabre::p! {
        @class="very-pretty";
        "Hello World!",

        leptos_macabre::strong!{
            @id;
            "WOW THAT IS PRETTY"
        }
        leptos_macabre::for_each!{
            ref number in 0..3 => {
                number.to_string()
            }
        }
    };

    assert_eq!(
        text.render_to_string().to_string(),
        r#"<p class="very-pretty">Hello World!<strong id="affirmation">WOW THAT IS PRETTY</strong>012</p>"#
    );
}
