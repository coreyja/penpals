use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, AddEventListenerOptions, HtmlTextAreaElement, KeyboardEvent, SubmitEvent};

mod text_formatter;

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

fn get_original_message_el() -> HtmlTextAreaElement {
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");

    document
        .get_element_by_id("original-message")
        .expect("Could not find original message")
        .dyn_into()
        .expect("Could not convert original message to HTMLTextAreaElement")
}

fn get_formatted_message_el() -> HtmlTextAreaElement {
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");

    document
        .get_element_by_id("formatted-message")
        .expect("Could not find formatted message")
        .dyn_into()
        .expect("Could not convert formatted message to HTMLTextAreaElement")
}

fn format_original_message() {
    let original_message_el = get_original_message_el();

    let formatted_message_el = get_formatted_message_el();

    let original_message_text = original_message_el.value();

    let formatted_message_text = text_formatter::format(&original_message_text);
    formatted_message_el.set_value(&formatted_message_text);
}

fn main() {
    console_error_panic_hook::set_once();

    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access the document");

    let form = document
        .get_element_by_id("form")
        .expect("Could not find form");

    let format_original_message_closure: Closure<dyn Fn()> = Closure::new(format_original_message);

    let on_submit: Closure<dyn FnMut(SubmitEvent)> = Closure::new(|event: SubmitEvent| {
        console_log!("submit");
        event.prevent_default();
        format_original_message()
    });

    let keydown_listener: Closure<dyn FnMut(KeyboardEvent)> =
        Closure::new(|event: KeyboardEvent| {
            console_log!("keydown");

            if event.key() == "Enter" && (event.ctrl_key() || event.meta_key()) {
                event.prevent_default();
                let el = event
                    .target()
                    .unwrap()
                    .dyn_into::<HtmlTextAreaElement>()
                    .expect("Could not convert event targer to an HTMLTextAreaElement");

                let form = el.form().expect("Could not find form");

                let submit_event =
                    web_sys::Event::new("submit").expect("Could not create submit event");
                form.dispatch_event(&submit_event)
                    .expect("Could not dispatch submit event");
            }
        });

    form.add_event_listener_with_callback(
        "submit",
        on_submit
            .as_ref()
            .dyn_ref()
            .expect("Could not turn closure to a function reference"),
    )
    .expect("Could not add event listener");

    get_original_message_el()
        .add_event_listener_with_callback(
            "keydown",
            keydown_listener
                .as_ref()
                .dyn_ref()
                .expect("Could not turn closure to a function reference"),
        )
        .expect("Could not add event listener");

    format_original_message_closure.forget();
    keydown_listener.forget();
    on_submit.forget();

    parse_initial_search_params();
}

fn parse_initial_search_params() {
    let location = window().expect("Could not get window").location();

    let protocol = location.protocol().expect("Could not get protocol");
    let host = location.host().expect("Could not get host");
    let pathname = location.pathname().expect("Could not get pathname");
    let url_without_search = format!("{}//{}{}", protocol, host, pathname);

    let search = location.search().expect("Could not get search");
    let url_search_params =
        web_sys::UrlSearchParams::new_with_str(&search).expect("Could not parse search");
    let original_message_query = url_search_params.get("original-message");
    if let Some(original_message_query) = original_message_query {
        get_original_message_el().set_value(&original_message_query);
        format_original_message();
        window()
            .expect("Could not get window")
            .history()
            .expect("Could not get history")
            .push_state_with_url(&JsValue::null(), "unused", Some(&url_without_search))
            .expect("Could not replace history");
    }
}
