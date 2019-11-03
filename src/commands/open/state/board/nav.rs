use cursive::{views::*, view::*};
use super::State;

pub fn new(state: State) -> impl View {
    let git_project = state.git_project.borrow();

    let left_nav = git_project.projects().iter()
        .enumerate()
        .fold(SelectView::new(), |sel, (i, project)| sel.item(project.name(), i))
        .on_select({ let state = state.clone(); move |s, i| {
            state.selected_project.set(*i);
            state.reload(s);
        }})
        .selected(state.selected_project.get());

    Panel::new(PaddedView::new((1, 1, 0, 0), left_nav))
        .fixed_width(20)
}
