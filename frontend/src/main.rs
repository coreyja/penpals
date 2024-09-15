use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, HtmlTextAreaElement};

mod text_formatter;

fn main() {
    console_error_panic_hook::set_once();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");

    let format_button = document
        .get_element_by_id("format")
        .expect("Could not find format button");

    let format_on_click: Closure<dyn FnMut()> = Closure::new(move || {
        let original_message_el: HtmlTextAreaElement = document
            .get_element_by_id("original-message")
            .expect("Could not find original message")
            .dyn_into()
            .expect("Could not convert original message to HTMLTextAreaElement");

        let formatted_message_el: HtmlTextAreaElement = document
            .get_element_by_id("formatted-message")
            .expect("Could not find formatted message")
            .dyn_into()
            .expect("Could not convert formatted message to HTMLTextAreaElement");

        let original_message_text = original_message_el.value();

        let formatted_message_text = text_formatter::format(&original_message_text);

        formatted_message_el.set_value(&formatted_message_text);
    });

    format_button
        .add_event_listener_with_callback("click", format_on_click.as_ref().unchecked_ref())
        .expect("Could not add event listener");

    format_on_click.forget();
}
