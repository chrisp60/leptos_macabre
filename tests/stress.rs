use leptos_macabre::*;

#[test]
fn stress_test() {
    let view = p! {
        {
            #[derive(Debug)]
            struct _ImInsideYourHtml {
                _name: String,
                _other_name: usize,
            }
        }
        1
    };
    assert_eq!(view.render_to_string().to_string(), r#"<p>1</p>"#);
}
