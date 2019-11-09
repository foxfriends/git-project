use std::rc::Rc;
use std::cell::RefCell;
use cursive::{Cursive, views::*, view::*};
use super::super::State;
use crate::model::*;

fn form<F>(state: State, columns: Rc<RefCell<Vec<Column>>>, column: Option<&Column>, finished: F) -> impl View 
where F: 'static + Fn(&mut Cursive) {
    let name = LinearLayout::horizontal()
        .child(TextView::new("Name").fixed_width(12))
        .child(EditView::new()
            .content(column.map(Column::name).unwrap_or_default())
            .with_id("column-name")
            .full_width());

    let description = LinearLayout::horizontal()
        .child(TextView::new("Description").fixed_width(12))
        .child(TextArea::new()
            .content(column.map(Column::description).unwrap_or_default())
            .with_id("column-description")
            .full_width()
            .min_height(5));

    let form = LinearLayout::vertical()
        .child(name)
        .child(DummyView)
        .child(description);

    let form_dialog = Dialog::around(PaddedView::new((0, 0, 1, 0), form))
        .button("Discard", { let state = state.clone(); let column = column.cloned(); move |s| {
            let name = s.find_id::<EditView>("column-name").unwrap().get_content().to_string();
            let description = s.find_id::<TextArea>("column-description").unwrap().get_content().to_string();
            if
                name != column.as_ref().map(Column::name).unwrap_or_default() ||
                description != column.as_ref().map(Column::description).unwrap_or_default()
            {
                state.confirm(s, if column.is_some() { "Discard changes?" } else { "Discard new column?" }, |s| { s.pop_layer(); });
            } else {
                s.pop_layer();
            }
        }})
        .button("Save", { let editing = column.cloned(); move |s| {
            let name = s.find_id::<EditView>("column-name").unwrap().get_content().to_string();
            let description = s.find_id::<TextArea>("column-description").unwrap().get_content().to_string();
            if name.is_empty() || description.is_empty() {
                s.add_layer(Dialog::info("Required information is missing"));
                return;
            }
            let empty_column = Column::new(&name).description(description);
            let column = editing.as_ref()
                .map(Column::tasks)
                .unwrap_or(&[])
                .iter()
                .fold(empty_column, ColumnBuilder::add_task_id)
                .build()
                .unwrap();

            if let Some(editing) = editing.as_ref() {
                let index = columns.borrow().iter().position(|col| col.name() == editing.name()).unwrap();
                columns.borrow_mut()[index] = column;
                s.pop_layer();
                finished(s);
            } else {
                if columns.borrow().iter().find(|col| col.name() == column.name()).is_none() {
                    columns.borrow_mut().push(column);
                    s.pop_layer();
                    finished(s);
                } else {
                    s.add_layer(Dialog::info(format!("A column named {} already exists", name)));
                }
            }
        }});


    form_dialog
}

pub fn new<F>(state: State, columns: Rc<RefCell<Vec<Column>>>, finished: F) -> impl View 
where F: 'static + Fn(&mut Cursive) {
    form(state, columns, None, finished)
}

pub fn edit<F>(state: State, columns: Rc<RefCell<Vec<Column>>>, column: Column, finished: F) -> impl View 
where F: 'static + Fn(&mut Cursive) {
    form(state, columns, Some(&column), finished)
}
