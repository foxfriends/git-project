use cursive::{align::*, theme::*, traits::*, views::*, utils::markup::StyledString};
use super::State;
use crate::model::*;

pub fn card(state: State, task: &Task) -> impl View {
    let mut description_text = StyledString::styled(task.name(), Effect::Bold);
    if !task.name().ends_with(|ch: char| ch.is_ascii_punctuation()) {
        // add a period if the last char was not punctuation already
        description_text.append_styled(".", Effect::Bold);
    }
    description_text.append_plain(" ");
    description_text.append_plain(task.short_description());

    let actions = LinearLayout::horizontal()
        .child(DummyView.full_width())
        .child(Button::new("Details", { let state = state.clone(); let task = task.clone(); move |s| { 
            state.show_task(task.clone(), s) 
        }}));
    let task_contents = LinearLayout::vertical()
        .child(PaddedView::new((0, 0, 1, 1), TextView::new(description_text)))
        .child(actions);
    Panel::new(PaddedView::new((1, 1, 0, 0), task_contents))
        .title(task.id())
        .title_position(HAlign::Left)
        .full_width()
        .with_id(task.id())
}
