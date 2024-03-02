pub mod editor;
pub mod rope;
pub mod syntect_lib;

use cursive::traits::*;
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};

use std::rc::Rc;

use editor::Editor;

struct State {
    syntax_set: SyntaxSet,
    themes: ThemeSet,
    editor: Editor,
}

fn main() {
    let mut siv = cursive::default();

    let syntax_set = SyntaxSet::load_defaults_newlines();
    let themes = ThemeSet::load_defaults();

    siv.set_user_data(Rc::new(State {
        syntax_set,
        themes,
        editor: Editor::new(""),
    }));

    siv.add_global_callback('r', |s| {
        let file: String = std::fs::read_to_string("./src/main.rs").unwrap();

        s.set_user_data(Rc::new(State {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            themes: ThemeSet::load_defaults(),
            editor: Editor::new(&file),
        }));


        apply_theme(s, "InspiredGitHub");
    });

    siv.add_fullscreen_layer(
        cursive::views::TextView::new("")
            .with_name("content")
            .scrollable()
            .full_screen(),
    );

    siv.with_theme(|t| {
        t.shadow = false;
    });

    apply_theme(&mut siv, "InspiredGitHub");

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

fn apply_theme(siv: &mut cursive::Cursive, theme_name: &str) {
    let state = siv
        .with_user_data(|s: &mut Rc<State>| Rc::clone(s))
        .unwrap();

    let theme = &state.themes.themes[theme_name];
    let syntax = state.syntax_set.find_syntax_by_token("rs").unwrap();
    let mut highlighter = syntect::easy::HighlightLines::new(syntax, theme);

    siv.with_theme(|t| {
        if let Some(background) = theme.settings.background.map(syntect_lib::translate_color) {
            t.palette[cursive::theme::PaletteColor::Background] = background;
            t.palette[cursive::theme::PaletteColor::View] = background;
        }
        if let Some(foreground) = theme.settings.foreground.map(syntect_lib::translate_color) {
            t.palette[cursive::theme::PaletteColor::Primary] = foreground;
            t.palette[cursive::theme::PaletteColor::TitlePrimary] = foreground;
        }

        if let Some(highlight) = theme.settings.highlight.map(syntect_lib::translate_color) {
            t.palette[cursive::theme::PaletteColor::Highlight] = highlight;
        }
    });

    let content = state.editor.data.to_string();

    let styled = syntect_lib::parse(content, &mut highlighter, &state.syntax_set).unwrap();

    siv.call_on_name("content", |t: &mut cursive::views::TextView| {
        t.set_content(styled);
    })
    .unwrap();
}
