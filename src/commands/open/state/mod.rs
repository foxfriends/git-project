use std::cell::{Cell, RefCell};
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use cursive::Cursive;
use cursive::{align::*, menu::*, theme::*, traits::*, views::*, utils::markup::StyledString, event};
use crate::model::*;

mod board;
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
        let edit_menu = MenuTree::new()
            .leaf("New Task", { let state = self.clone(); move |s| { /* TODO */ }});
        siv.menubar()
            .add_subtree("File", file_menu)
            .add_subtree("Edit", edit_menu);

        self.reload(&mut siv);

        siv.run();
        Ok(())
    }

    fn reload(&self, siv: &mut Cursive) {
        siv.pop_layer();
        let project_view = self.git_project_view();
        let global_events = OnEventView::new(project_view)
            .on_event(event::Key::Esc, |s| s.select_menubar())
            .on_event('?', show_help);
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

    fn git_project_view(&self) -> impl View {
        let git_project = self.git_project.borrow();

        let mut left_nav = SelectView::new()
            .on_select({ let state = self.clone(); move |s, i| {
                state.selected_project.set(*i);
                state.reload(s);
            }});
        for (i, project) in git_project.projects().iter().enumerate() {
            left_nav.add_item(project.name(), i);
        }
        left_nav.set_selection(self.selected_project.get());
        let padded_left_nav = PaddedView::new((1, 1, 0, 0), left_nav);

        LinearLayout::horizontal()
            .child(Panel::new(padded_left_nav))
            .child(board::board(self.clone()))
            .full_screen()
    }

    fn project_view(&self) -> impl View {
        board::board(self.clone())
    }

    fn show_task(&self, task: Task, siv: &mut Cursive) {
        let git_project = self.git_project.borrow();
        let project = &git_project.projects()[self.selected_project.get()];
        let tags_list = task.tags().iter()
            .map(|tag| TextView::new(format!("#{}", tag)).no_wrap())
            .fold(LinearLayout::vertical(), LinearLayout::child);

        let mut assignee_text = StyledString::plain("Assigned to: ");
        match task.assignee() {
            Some(assignee) if assignee == self.current_user => {
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

        let task_dialog = Dialog::around(PaddedView::new((0, 0, 1, 0), task_contents))
            .title(task.id())
            .title_position(HAlign::Left)
            .button("Close", |s| { s.pop_layer(); })
            .button("Edit", { let state = self.clone(); move |s| { state.edit_task(task.clone(), s) }});

        siv.add_layer(task_dialog);
    }

    fn new_task(&self, siv: &mut Cursive) {
        let form_dialog = form::task::new(self.clone());
        siv.add_layer(form_dialog);
    }

    fn new_column(&self, siv: &mut Cursive) {

    }

    fn new_project(&self, siv: &mut Cursive) {

    }

    fn edit_task(&self, task: Task, siv: &mut Cursive) {
        // TODO
    }

    fn edit_column(&self, column: Column, siv: &mut Cursive) {
        // TODO
    }

    fn edit_project(&self, project: Project, siv: &mut Cursive) {
        // TODO
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

fn show_help(siv: &mut Cursive) {
    let help_text = LinearLayout::vertical()
        .child(TextView::new("?: Show this help"));

    let dialog = Dialog::around(help_text)
        .title("Help")
        .button("Ok", |s| { s.pop_layer(); });

    siv.add_layer(dialog);
}
