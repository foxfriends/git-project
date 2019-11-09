use cursive::{Cursive, views::*, view::*, event};
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
        .selected(state.selected_project.get())
        .with_id("nav-projects");

    let delete_project = { let state = state.clone(); move |s: &mut Cursive| {
        let nav_projects = s.find_id::<SelectView<usize>>("nav-projects").unwrap();
        let i = *nav_projects.selection().unwrap();

        let git_project = state.git_project.borrow();
        if git_project.projects().len() == 1 {
            // cannot delete the only project
            return;
        }
        let project = git_project.projects().iter().skip(i).next().unwrap();

        state.confirm(s, format!("Delete {}", project.name()), { let state = state.clone(); move |s| { 
            state.git_project.borrow_mut().delete_project(i); 
            state.selected_project.set(i.saturating_sub(1));
            s.pop_layer();
            state.reload(s);
        }});
    }};

    let evented = OnEventView::new(left_nav)
        .on_event(event::Key::Del, delete_project.clone())
        .on_event(event::Key::Backspace, delete_project);

    Panel::new(PaddedView::new((1, 1, 0, 0), evented))
        .fixed_width(20)
}
