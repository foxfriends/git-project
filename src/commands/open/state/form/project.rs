use std::rc::Rc;
use std::cell::RefCell;
use cursive::{Cursive, views::*, view::*, event};
use super::super::State;
use crate::model::*;

fn form(state: State, project: Option<&Project>) -> impl View {
    let guess_columns = state.git_project.borrow()
        .projects().iter().last()
        .map(|project| project.columns().iter().map(|col| col.without_tasks()).collect())
        .unwrap_or(vec![]);
    let columns: Rc<RefCell<Vec<Column>>> = Rc::new(RefCell::new(project.map(Project::columns).map(|cols| cols.to_vec()).unwrap_or(guess_columns)));

    let name = LinearLayout::horizontal()
        .child(TextView::new("Name").fixed_width(12))
        .child(EditView::new()
            .content(project.map(Project::name).unwrap_or_default())
            .with_id("project-name")
            .full_width());

    let description = LinearLayout::horizontal()
        .child(TextView::new("Description").fixed_width(12))
        .child(TextArea::new()
            .content(project.map(Project::description).unwrap_or_default())
            .with_id("project-description")
            .full_width()
            .min_height(5));

    let edit_column = { let state = state.clone(); let columns = columns.clone(); move |s: &mut Cursive, column: &Column| {
        let form_dialog = super::column::edit(state.clone(), columns.clone(), column.clone(), { let columns = columns.clone(); move |s| {
            let mut columns_view = s.find_id::<SelectView<Column>>("project-columns").unwrap();
            let id = columns_view.selected_id().unwrap();
            let new_column = &columns.borrow()[id];
            columns_view.remove_item(id)(s);
            columns_view.insert_item(id, new_column.name(), new_column.clone());
        }});
        s.add_layer(form_dialog);
    }};

    let delete_column = { let state = state.clone(); let columns = columns.clone(); move |s: &mut Cursive| {
        let mut columns_view = s.find_id::<SelectView<Column>>("project-columns").unwrap();
        if let Some(id) = columns_view.selected_id() {
            let mut columns_ref = columns.borrow_mut();
            let column = &columns_ref[id];
            if column.tasks().is_empty() {
                columns_ref.remove(id);
                columns_view.remove_item(id)(s);
            } else {
                state.confirm(s, format!("Are you sure you want to delete {}? Tasks in this column will be lost.", column.name()), { let columns = columns.clone(); move |s| {
                    let mut columns_view = s.find_id::<SelectView<Column>>("project-columns").unwrap();
                    columns.borrow_mut().remove(id);
                    columns_view.remove_item(id)(s);
                }});
            }
        }
    }};

    let columns_list = columns.borrow().iter()
        .fold(SelectView::new(), |sel, col| sel.item(col.name(), col.clone()))
        .on_submit(edit_column)
        .with_id("project-columns");
    let columns_container = LinearLayout::horizontal()
        .child(TextView::new("Columns").fixed_width(12))
        .child(OnEventView::new(columns_list)
            .on_event(event::Key::Del, delete_column.clone())
            .on_event(event::Key::Backspace, delete_column))
        .child(DummyView)
        .child(Button::new("Add Column", { let state = state.clone(); let columns = columns.clone(); move |s| {
            let form_dialog = super::column::new(state.clone(), columns.clone(), { let columns = columns.clone(); move |s| {
                let mut columns_view = s.find_id::<SelectView<Column>>("project-columns").unwrap();
                let new_column = columns.borrow().last().unwrap().clone();
                columns_view.add_item(new_column.name().to_string(), new_column);
            }});
            s.add_layer(form_dialog);
        }}));

    let form = LinearLayout::vertical()
        .child(name)
        .child(DummyView)
        .child(description)
        .child(DummyView)
        .child(columns_container);

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
            let project_with_columns = columns.borrow().iter()
                .cloned()
                .fold(Project::new(&name).description(description), ProjectBuilder::column);
            let project = editing.as_ref()
                .map(|editing| editing
                    .tasks()
                    .iter()
                    .filter(|task| columns
                        .borrow()
                        .iter()
                        .find(|col| col.tasks().contains(task.id()))
                        .is_some())
                    .cloned()
                    .collect())
                .unwrap_or(vec![])
                .into_iter()
                .fold(project_with_columns, ProjectBuilder::task)
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
