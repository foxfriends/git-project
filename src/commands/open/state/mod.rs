use std::cell::{Cell, RefCell};
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use cursive::Cursive;
use cursive::{menu::*, views::*, event};
use crate::model::*;

mod board;
mod dialog;
mod help;
mod form;

#[derive(Clone, Debug)]
pub struct State {
    git_project: Rc<RefCell<GitProject>>,
    selected_project: Rc<Cell<usize>>,
    unsaved_changes: Rc<Cell<bool>>,
    current_user: String,
}

impl State {
    pub fn new(git_project: GitProject, current_user: String) -> Self {
        Self {
            git_project: Rc::new(RefCell::new(git_project)),
            selected_project: Rc::new(Cell::new(0)),
            unsaved_changes: Rc::new(Cell::new(false)),
            current_user,
        }
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let mut siv = Cursive::pancurses().unwrap();
        siv.set_autohide_menu(false);
        let file_menu = MenuTree::new()
            .leaf("New Task", { let state = self.clone(); move |s| { state.new_task(s) }})
            .leaf("New Column", { let state = self.clone(); move |s| { state.new_column(s) }})
            .leaf("New Project", { let state = self.clone(); move |s| { state.new_project(s) }})
            .leaf("Save", { let state = self.clone(); move |s| { state.save(s); }})
            .delimiter()
            .leaf("Quit", { let state = self.clone(); move |s| { state.quit(s); }});
        let project_menu = MenuTree::new()
            .leaf("Edit Project", { let state = self.clone(); move |s| { 
                if let Some(project) = state.git_project.borrow().projects().iter().skip(state.selected_project.get()).next() {
                    state.edit_project(project.clone(), s);
                } else {
                    state.new_project(s);
                }
            }});
        siv.menubar()
            .add_subtree("File", file_menu)
            .add_subtree("Project", project_menu);

        self.reload(&mut siv);

        siv.run();
        Ok(())
    }

    fn reload(&self, siv: &mut Cursive) {
        siv.pop_layer();
        let project_view = board::view(self.clone());
        let global_events = OnEventView::new(project_view)
            .on_event(event::Key::Esc, |s| s.select_menubar())
            .on_event('?', help::show);
        siv.add_fullscreen_layer(global_events);
    }

    fn save(&self, siv: &mut Cursive) {
        let result = self.git_project.borrow().save();
        self.handle_result(siv, result);
    }

    fn quit(&self, siv: &mut Cursive) {
        if self.unsaved_changes.get() {
            let dialog = Dialog::text("Save before quitting?")
                .button("Cancel", |s| { s.pop_layer(); })
                .button("Quit without saving", Cursive::quit)
                .button("Save and quit", { let state = self.clone(); move |s| { state.save(s); s.quit(); }});
            siv.add_layer(dialog);
        } else {
            siv.quit();
        }
    }

    fn handle_result<S, E: Display>(&self, siv: &mut Cursive, result: Result<S, E>) {
        if let Err(error) = result {
            let dialog = Dialog::text(format!("{}", error))
                .title("Error")
                .button("Ok", |s| { s.pop_layer(); });
            siv.add_layer(dialog);
        }
    }

    fn show_task(&self, task: Task, siv: &mut Cursive) {
        let task_dialog = dialog::task::task(self.clone(), task);
        siv.add_layer(task_dialog);
    }

    fn new_task(&self, siv: &mut Cursive) {
        let form_dialog = form::task::new(self.clone());
        siv.add_layer(form_dialog);
    }

    fn new_column(&self, siv: &mut Cursive) {
        let form_dialog = form::column::new(self.clone());
        siv.add_layer(form_dialog);
    }

    fn new_project(&self, siv: &mut Cursive) {
        let form_dialog = form::project::new(self.clone());
        siv.add_layer(form_dialog);
    }

    fn edit_task(&self, task: Task, siv: &mut Cursive) {
        let form_dialog = form::task::edit(self.clone(), task);
        siv.add_layer(form_dialog);
    }

    fn edit_column(&self, column: Column, siv: &mut Cursive) {
        let form_dialog = form::column::edit(self.clone(), column);
        siv.add_layer(form_dialog);
    }

    fn edit_project(&self, project: Project, siv: &mut Cursive) {
        let form_dialog = form::project::edit(self.clone(), project);
        siv.add_layer(form_dialog);
    }

    fn confirm<I, F>(&self, siv: &mut Cursive, title: I, callback: F)
    where I: Into<String>, F: 'static + Fn(&mut Cursive) {
        let dialog = Dialog::text(title)
            .button("Yes", move |s| {
                s.pop_layer();
                callback(s);
            })
            .button("No", |s| { s.pop_layer(); });
        siv.add_layer(dialog);
    }
}
