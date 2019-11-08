use cursive::{views::*, view::*};
use super::super::State;
use crate::model::*;

fn form(state: State, project: Option<&Project>) -> impl View {
    let name = LinearLayout::horizontal()
        .child(TextView::new("Name").fixed_width(12))
        .child(EditView::new()
            .content(project.map(Project::name).map(Into::<String>::into).unwrap_or_default())
            .with_id("project-name")
            .full_width());

    let description = LinearLayout::horizontal()
        .child(TextView::new("Description").fixed_width(12))
        .child(TextArea::new()
            .content(project.map(Project::description).unwrap_or_default())
            .with_id("project-description")
            .full_width()
            .min_height(5));

    let form = LinearLayout::vertical()
        .child(name)
        .child(DummyView)
        .child(description);

    let form_dialog = Dialog::around(PaddedView::new((0, 0, 1, 0), form))
        .button("Discard", { let state = state.clone(); let project = project.cloned(); move |s| {
            let name = s.find_id::<EditView>("project-name").unwrap().get_content().to_string();
            let description = s.find_id::<TextArea>("project-description").unwrap().get_content().to_string();
            if
                name != project.as_ref().map(Project::name).unwrap_or_default() ||
                description != project.as_ref().map(Project::description).unwrap_or_default()
            {
                state.confirm(s, if project.is_some() { "Discard changes?" } else { "Discard new project?" }, |s| { s.pop_layer(); });
            } else {
                s.pop_layer();
            }
        }})
        .button("Save", { let state = state.clone(); let editing = project.cloned(); move |s| {
            let name = s.find_id::<EditView>("project-name").unwrap().get_content().to_string();
            let description = s.find_id::<TextArea>("project-description").unwrap().get_content().to_string();
            if name.is_empty() || description.is_empty() {
                s.add_layer(Dialog::info("Required information is missing"));
                return;
            }
            let project = editing.as_ref().map(|project| project.edit()).unwrap_or(Project::new(&name))
                .name(&name)
                .description(description)
                .build()
                .unwrap();

            if let Some(editing) = editing.as_ref() {
                state.git_project
                    .borrow_mut()
                    .replace_project(editing.name(), project);
                s.pop_layer();
                state.reload(s);
            } else {
                let success = state.git_project.borrow_mut()
                    .add_project(project);
                if success {
                    s.pop_layer();
                    state.reload(s);
                } else {
                    s.add_layer(Dialog::info(format!("A project named {} already exists", name)));
                }
            }
        }});

    form_dialog
}

pub fn new(state: State) -> impl View {
    form(state, None)
}

pub fn edit(state: State, project: Project) -> impl View {
    form(state, Some(&project))
}
