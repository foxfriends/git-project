use std::rc::Rc;
use std::cell::RefCell;
use cursive::{Cursive, views::*, view::*, event};
use super::super::State;
use crate::model::*;

fn form(state: State, column: Option<&Column>) -> impl View {
    let git_project = state.git_project.borrow();
    let project = &git_project.projects()[state.selected_project.get()];

    DummyView
}

pub fn new(state: State) -> impl View {
    form(state, None)
}

pub fn edit(state: State, column: Column) -> impl View {
    form(state, Some(&column))
}
