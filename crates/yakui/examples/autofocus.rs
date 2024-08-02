use bootstrap::ExampleState;
use yakui::widgets::{Pad, TextBox};
use yakui::{center, use_state};

pub fn run(example_state: &mut ExampleState) {
    let text = use_state(|| "".to_owned());
    let autofocus = use_state(|| false);
    let mut textbox_id = None;

    center(|| {
        let mut box1 = TextBox::new(text.borrow().as_str());
        box1.style.font_size = 60.0;
        box1.padding = Pad::all(50.0);
        box1.placeholder = "placeholder".into();

        let response = box1.show();
        textbox_id = Some(response.id);

        if let Some(new_text) = response.into_inner().text {
            text.set(new_text);
        }
    });

    if !autofocus.get() {
        autofocus.set(true);
        example_state.request_focus = Some(textbox_id.unwrap());
    }
}

fn main() {
    bootstrap::start(run as fn(&mut ExampleState));
}
