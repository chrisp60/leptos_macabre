use leptos_macabre::*;

#[test]
fn dashes() {
    let view = p! {
        @hx-boost="true";
        @a-very-long-attribute-name={
            1 + 3 + 54
        };
        "hi"
    };
    assert_eq!(
        r#"<p hx-boost="true" a-very-long-attribute-name="58">hi</p>"#,
        view.render_to_string().to_string()
    );
}
