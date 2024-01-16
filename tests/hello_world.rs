use leptos_macabre::*;

#[test]
fn main() {
    let id = "hello-world";
    let view = p! {
        @id;
        "Hello World!",
    };
    assert_eq!(
        view.render_to_string().to_string(),
        r#"<p id="hello-world">Hello World!</p>"#
    );
}
