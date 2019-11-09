use cursive::{Cursive, align::*, theme::*, traits::*, views::*, event, utils::markup::StyledString};
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

    let button = Button::new("Details", { let state = state.clone(); let task = task.clone(); move |s| { 
        state.show_task(task.clone(), s) 
    }});

    let delete_task = { let state = state.clone(); let task = task.clone(); move |s: &mut Cursive| {
        state.confirm(s, format!("Delete task {}?", task.id()), { let task = task.clone(); let state = state.clone(); move |s| {
            let mut git_project = state.git_project.borrow_mut();
            let current_project = &mut git_project.projects_mut()[state.selected_project.get()];
            current_project.delete_task(task.id());
            std::mem::drop(git_project);
            state.reload(s);
        }});
    }};
    let event_handler = OnEventView::new(button)
        .on_event(event::Key::Del, delete_task.clone())
        .on_event(event::Key::Backspace, delete_task.clone());

    let actions = LinearLayout::horizontal()
        .child(DummyView.full_width())
        .child(event_handler);
    let task_contents = LinearLayout::vertical()
        .child(PaddedView::new((0, 0, 1, 1), TextView::new(description_text)))
        .child(actions);
    Panel::new(PaddedView::new((1, 1, 0, 0), task_contents))
        .title(task.id())
        .title_position(HAlign::Left)
        .full_width()
        .with_id(task.id())
}
