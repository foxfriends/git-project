use std::cell::{Cell, RefCell};
use std::env::current_dir;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use cursive::Cursive;
use cursive::{align::*, menu::*, theme::*, traits::*, views::*, utils::markup::StyledString, event};
use git2::Repository;
use crate::model::*;

#[derive(Clone, Debug)]
struct State {
    git_project: Rc<RefCell<GitProject>>,
    selected_project: Rc<Cell<usize>>,
    current_user: String,
}

pub fn open() -> Result<(), Box<dyn Error>> {
    let repository = Repository::discover(current_dir()?)?;
    let config = repository.config()?.snapshot()?;
    let current_user = config.get_string("user.email")?;

    let git_project = GitProject::open()?;

    let state = State::new(git_project, current_user);
    state.run()
}

impl State {
    fn new(git_project: GitProject, current_user: String) -> Self {
        Self {
            git_project: Rc::new(RefCell::new(git_project)),
            selected_project: Rc::new(Cell::new(0)),
            current_user,
        }
    }

    fn run(self) -> Result<(), Box<dyn Error>> {
        let mut siv = Cursive::pancurses().unwrap();
        siv.set_autohide_menu(false);
        let file_menu = MenuTree::new()
            .leaf("Save", { let state = self.clone(); move |s| { state.save(s); }})
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
        let dialog = Dialog::text("Save before quitting?")
            .button("Cancel", |s| { s.pop_layer(); })
            .button("Quit without saving", Cursive::quit)
            .button("Save and quit", { let state = self.clone(); move |s| { state.save(s); s.quit(); }});
        siv.add_layer(dialog);
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

        let mut container = LinearLayout::horizontal();
        container.add_child(Panel::new(padded_left_nav));

        if let Some(project) = git_project.projects().iter().nth(self.selected_project.get()) {
            container.add_child(self.project_view(project).full_screen());
        } else {
            let empty_state = TextView::new("No projects found. Create a new one?");
            container.add_child(Panel::new(empty_state));
        }

        container.full_screen()
    }

    fn project_view(&self, project: &Project) -> impl View {
        let header = LinearLayout::vertical()
            .child(TextView::new(project.name()).effect(Effect::Bold))
            .child(TextView::new(project.description().map(Into::into).unwrap_or(StyledString::styled("[No description]", PaletteColor::Secondary))));

        let mut project_board = LinearLayout::horizontal();
        for column in project.columns() {
            let column_view = column.tasks()
                .iter()
                .filter_map(|task_id| project.task_with_id(task_id))
                .map(|task| self.task_card(task))
                .fold(LinearLayout::vertical(), LinearLayout::child);
            let scroll_view = ScrollView::new(column_view)
                .full_height()
                .fixed_width(100);
            project_board.add_child(Panel::new(scroll_view).title(column.name()));
        }

        LinearLayout::vertical()
            .child(Panel::new(PaddedView::new((1, 1, 0, 0), header)).full_width())
            .child(ScrollView::new(project_board).full_screen())
    }

    fn task_card(&self, task: &Task) -> impl View {
        let mut description_text = StyledString::styled(task.name(), Effect::Bold);
        if !task.name().ends_with(|ch: char| ch.is_ascii_punctuation()) {
            // add a period if the last char was not punctuation already
            description_text.append_styled(".", Effect::Bold);
        }
        description_text.append_plain(" ");
        description_text.append_plain(task.short_description());

        let actions = LinearLayout::horizontal()
            .child(DummyView.full_width())
            .child(Button::new("Details", { let state = self.clone(); let task = task.clone(); move |s| { state.show_task(task.clone(), s) }}));
        let task_contents = LinearLayout::vertical()
            .child(PaddedView::new((0, 0, 1, 1), TextView::new(description_text)))
            .child(actions);
        Panel::new(PaddedView::new((1, 1, 0, 0), task_contents))
            .title(task.id())
            .title_position(HAlign::Left)
            .full_width()
            .with_id(task.id())
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

    fn edit_task(&self, task: Task, siv: &mut Cursive) {
        // TODO
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
