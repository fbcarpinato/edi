extern crate cursive;

use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::views::{DummyView, EditView, Layer, LinearLayout, TextArea};
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();

    let text_editor = TextArea::new()
        .disabled()
        .with_name("text_editor")
        .full_screen();

    let command_bar = EditView::new()
        .filler(" ")
        .on_submit(process_command)
        .with_name("command_bar")
        .min_width(20);

    let layout = LinearLayout::vertical()
        .child(Layer::new(text_editor))
        .child(DummyView.fixed_height(1))
        .child(command_bar);

    siv.add_layer(layout);

    siv.add_global_callback('i', |s| {
        s.focus_name("text_editor").unwrap();

        s.call_on_name("command_bar", |view: &mut EditView| {
            view.set_content("");
        });

        s.call_on_name("text_editor", |view: &mut TextArea| {
            view.enable();
        });
    });

    siv.add_global_callback(':', move |s| {
        s.call_on_name("command_bar", |view: &mut EditView| {
            view.set_content(":");
        });
        s.focus_name("command_bar").unwrap();
    });

    siv.add_global_callback(Event::Key(Key::Esc), |s| {
        s.call_on_name("text_editor", |view: &mut TextArea| {
            view.disable();
        });

        s.call_on_name("command_bar", |view: &mut EditView| {
            view.set_content("");
        });
        s.focus_name("text_editor").unwrap();
    });

    siv.run();
}

fn process_command(s: &mut Cursive, command: &str) {
    let command = command.trim_start_matches(':');

    match command {
        "q" => s.quit(),
        "clear" => {
            s.call_on_name("text_editor", |view: &mut TextArea| {
                view.set_content("");
            });
        }
        _ => {
            s.add_layer(cursive::views::Dialog::info("Unknown command"));
        }
    }

    s.call_on_name("command_bar", |view: &mut EditView| {
        view.set_content("");
    });
    s.focus_name("text_editor").unwrap();
}
