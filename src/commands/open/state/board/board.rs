use cursive::{views::*, view::*, theme::*};
use super::State;
use super::column;

pub fn board(state: State) -> impl View {
    let git_project = state.git_project.borrow();
    let project = &git_project.projects()[state.selected_project.get()];

    let header = LinearLayout::vertical()
        .child(TextView::new(project.name()).effect(Effect::Bold))
        .child(TextView::new(project.description()));

    let project_board = project.columns().iter()
        .map(|col| column::column(state.clone(), project, col))
        .fold(LinearLayout::horizontal(), LinearLayout::child);

    LinearLayout::vertical()
        .child(Panel::new(PaddedView::new((1, 1, 0, 0), header)).full_width())
        .child(ScrollView::new(project_board).full_screen())
}
