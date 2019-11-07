use cursive::{align::*, views::*, view::*, theme::*, utils::markup::StyledString};
use super::super::State;
use crate::model::*;

pub fn task(state: State, task: Task) -> impl View {
    let git_project = state.git_project.borrow();
    let project = &git_project.projects()[state.selected_project.get()];
    let tags_list = task.tags().iter()
        .map(|tag| TextView::new(format!("#{}", tag)).no_wrap())
        .fold(LinearLayout::vertical(), LinearLayout::child);

    let mut assignee_text = StyledString::plain("Assigned to: ");
    match task.assignee() {
        Some(assignee) if assignee == state.current_user => {
            assignee_text.append_plain(assignee);
            assignee_text.append_styled(" (You)", Effect::Bold);
        }
        Some(assignee) => assignee_text.append_plain(assignee),
        None => assignee_text.append_styled("Nobody", PaletteColor::Secondary),
    }

    let task_description = LinearLayout::vertical()
        .child(TextView::new(task.name()).effect(Effect::Bold))
        .child(DummyView)
        .child(TextView::new(task.description()))
        .fixed_width(100);

    let task_info = LinearLayout::vertical()
        .child(TextView::new(assignee_text))
        .child(TextView::new(format!("Status:      {}", project.column_of_task(&task).map(|col| col.name()).unwrap_or("Unknown"))))
        .child(LinearLayout::horizontal()
            .child(TextView::new("Tags:        "))
            .child(tags_list));

    let task_contents = LinearLayout::horizontal()
        .child(PaddedView::new((1, 1, 0, 0), task_description))
        .child(PaddedView::new((4, 1, 0, 0), task_info));

    Dialog::around(PaddedView::new((0, 0, 1, 0), task_contents))
        .title(task.id())
        .title_position(HAlign::Left)
        .button("Close", |s| { s.pop_layer(); })
        .button("Edit", { let state = state.clone(); move |s| { state.edit_task(task.clone(), s) }})
}
