#![feature(macro_metavar_expr)]
#![doc = include_str!("../README.md")]

pub use leptos_dom::html::*;
pub use leptos_dom::{CollectView, IntoAttribute, IntoView};

pub fn render(v: impl IntoView) -> String {
    v.into_view().render_to_string().to_string()
}

/// Internal helper to parse attribute
#[doc(hidden)]
#[macro_export]
macro_rules! __parse_attribute {
    ($lhs:ident $(- $lhs_post:ident)* = $rhs:expr) => {
        (
            $crate::__parse_attribute! {
                @handle_dashes $lhs $(- $lhs_post)*
            },
            $rhs.into_attribute()
        )
    };
    ($lhs:ident) => {
        (stringify!($lhs), $lhs.into_attribute())
    };
    ($lhs:ident $(- $lhs_post:ident)+) => {
        compile_error! {
            "cannot use shortand attribute values with 
            invalid rust identifiers."
        }
    };
    (@handle_dashes $lhs:ident $(- $lhs_post:ident)*) => {
        concat!{ stringify!($lhs), $( "-", stringify!($lhs_post), )* }
    };
}

macro_rules! define_macro {
    ($(
        $(#[$meta:meta])*
        <$element:ident> $(,)*
    ),*) => {
        $(
            $(#[$meta])*
            #[macro_export]
            macro_rules! $element {
                (
                    $$(@
                        $$lhs:ident $$(- $$lhs_post:ident )*
                        $$(= $$rhs:expr )?
                    ;)*
                    $$( $$children:expr $$(,)? )*
                ) => {{
                    #[allow(unused_imports)]
                    use $crate::{IntoView, IntoAttribute, CollectView};
                    let __dummy_attr = ("", false.into_attribute());
                    let __dummy_child = ().into_view();
                    let __attrs = [
                        __dummy_attr,
                        $$(
                            $crate::__parse_attribute! {
                                $$lhs $$(- $$lhs_post )*
                                $$( = $$rhs )?
                            },
                        )*
                    ];
                    let __children = [
                        __dummy_child,
                        $$( $$children.into_view(), )*
                    ];
                    $crate::$element().attrs(__attrs).child(__children).into_view()
                }};
            }
        )*
    };
}

#[macro_export]
macro_rules! for_each {
    ($bind:pat in $iterator:expr => $expr:expr $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::IntoView;
        let mut __buffer = vec![];
        for $bind in $iterator {
            __buffer.push({ $expr });
        }
        __buffer.into_view()
    }};
}

define_macro! {
    ///	Sets a hyperlink to another page.
    <a>,
    ///	Represents an abbreviation or acronym and provides a full-text explanation to it (optional).
    <abbr>,
    ///	Defines the contact information of the document's author..
    <address>,
    ///	Defines a clickable area in an image map.
    <area>,
    ///	Defines independent content.
    <article>,
    ///	Describes content to be displayed aside other content.
    <aside>,
    ///	Includes sound in the web page (music, streams, etc.).
    <audio>,
    ///	Displays enclosed text in bold typeface without conveying any added importance.
    <b>,
    ///	Defines the base URI or URL for all the relative links in the HTML document.
    <base>,
    <basefont>,
    ///	Defines bidirectional text isolation.
    <bdi>,
    ///	Defines bidirectional text override.
    <bdo>,
    ///	Defines a quoted section.
    <blockquote>,
    ///	Defines the HTML container that contains visible contents of an HTML document.
    <body>,
    ///	Defines a line break.
    <br>,
    ///	Creates a button.
    <button>,
    ///	Defines an area of the webpage that becomes a space for rendering graphics.
    <canvas>,
    ///	Defines an HTML table caption.
    <caption>,
    ///	Defines a title for a work.
    <cite>,
    ///	Defines text to be displayed in the computer output style.
    <code>,
    ///	Specifies grouped attributes for columns.
    <col>,
    ///	Specifies a common formatting style for a group of columns.
    <colgroup>,
    ///	Specifies a comment in the document's source code.
    <comment>,
    ///	Specifies an autocomplete feature to be used with a form element.
    <datalist>,
    ///	Describes a term in a description list.
    <dd>,
    ///	Defines deleted text in a document and crosses it out.
    <del>,
    ///	Specifies additional details that can be shown or hidden manually.
    <details>,
    ///	Represents a term that needs an explanation.
    <dfn>,
    ///	Creates pop-up dialog models.
    <dialog>,
    ///	Defines a section or a division in HTML document by grouping elements.
    <div>,
    ///	Defines a description list.
    <dl>,
    ///	Defines a description term.
    <dt>,
    ///	Defines emphasized text.
    <em>,
    ///	Defines an embedded resource.
    <embed>,
    ///	Defines a group of <form> elements.
    <fieldset>,
    ///	Sets a caption for a <figure> element.
    <figcaption>,
    ///	Defines a self-contained piece of content.
    <figure>,
    ///	Defines footer context.
    <footer>,
    ///	Defines a group of submittable inputs.
    <form>,
    /// - <h6>	Defines headings in a document.
    <h1>,
    ///	Defines a space for declaring title, stylesheets, scripts and other document related information.
    <head>,
    ///	Specifies a container for one or more headings.
    <header>,
    ///	Defines a thematic division between content.
    <hr>,
    ///	Defines a document as an HTML document.
    <html>,
    ///	Defines an italic styled text.
    <i>,
    ///	Defines a frame of inline nature.
    <iframe>,
    ///	Sets an area for an image.
    <img>,
    ///	Specifies an element used to take values from the user.
    <input>,
    ///	Defines inserted text in a document.
    <ins>,
    ///	Identifies a user keyboard entry.
    <kbd>,
    ///	Specifies a label for a given input element.
    <label>,
    ///	Sets a caption for the fieldset.
    <legend>,
    ///	Defines a list item within a list.
    <li>,
    ///	Sets a link between a document and other external resources.
    <link>,
    ///	Defines the primary content of the document.
    <main>,
    ///	Defines a client-side image-map (an image with clickable places).
    <map>,
    ///	Defines marked text.
    <mark>,
    ///	Defines a list/menu of commands.
    <menu>,
    ///	Defines supplementary information about a website.
    <meta>,
    ///	Displays a scalar measurement within a known range.
    <meter>,
    ///	Defines a block of navigational links to the main sections of a website.
    <nav>,
    ///	Defines HTML to be inserted as replacement if the browser does not support scripting.
    <noscript>,
    ///	Defines an external source that is embedded into the HTML document.
    <object>,
    ///	Defines an ordered list of items in the HTML document.
    <ol>,
    ///	Groups the related options within a select element.
    <optgroup>,
    ///	Sets an HTML option in select element list.
    <option>,
    ///	Displays the result of a calculation performed.
    <output>,
    ///	Defines an HTML paragraph of text.
    <p>,
    ///	Defines parameters for an object element.
    <param>,
    ///	Defines preformatted text. Preserves spaces and line breaks.
    <pre>,
    ///	Displays an HTML progress bar representing the progress of a task.
    <progress>,
    ///	Indicates that the enclosed text is an inline quotation.
    <q>,
    ///	Defines a fallback parenthesis for a ruby annotation.
    <rp>,
    ///	Defines ruby text for Asian characters.
    <rt>,
    ///	Represents ruby annotation for Asian characters.
    <ruby>,
    ///	Draws a line across the text making a strikethrough.
    <s>,
    ///	Defines sample output from a computer program.
    <samp>,
    ///	Defines a client-side script (JavaScript).
    <script>,
    ///	Defines the primary content of the document.
    <section>,
    ///	Creates an HTML dropdown list with one or more options for a web form.
    <select>,
    ///	Reduces the HTML text size down by one size.
    <small>,
    ///	Defines sources for media elements like audio, video and picture.
    <source>,
    ///	Sets an inline container for formatting an HTML document.
    <span>,
    ///	Makes an important part of text appear bold.
    <strong>,
    ///	Defines styling properties for different HTML elements.
    <style>,
    ///	Defines subscript text.
    <sub>,
    ///	Sets the summary content for the <details> element.
    <summary>,
    ///	Defines superscript text.
    <sup>,
    ///	Includes a table with rows and columns in a web page.
    <table>,
    ///	Associates the body content in a table.
    <tbody>,
    ///	Defines table data.
    <td>,
    ///	Defines table footer content.
    <tfoot>,
    ///	Defines an HTML header table cell.
    <th>,
    ///	Defines the header of a table.
    <thead>,
    ///	Defines human and machine readable date and time.
    <time>,
    ///	Defines the title of the HTML document.
    <title>,
    ///	Defines a row in an HTML table.
    <tr>,
    ///	Defines text tracks for all the media elements.
    <track>,
    ///	Underlines the text.
    <u>,
    ///	Defines an unordered list.
    <ul>,
    ///	Defines a variable.
    <var>,
    ///	Includes a video in the web page (a video clip, streams, etc.).
    <video>,
    ///	Defines a possible line-break in a text
    <wbr>,
}
