use std::cell::{Cell, RefCell};
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;
use cursive::Cursive;
use cursive::{menu::*, traits::*, views::*, event};
use crate::model::GitProject;

#[derive(Clone, Debug)]
struct State {
    git_project: Rc<RefCell<GitProject>>,
    selected_project: Rc<Cell<usize>>,
}

pub fn open() -> Result<(), Box<dyn Error>> {
    let git_project = GitProject::open()?;
    let state = State::new(git_project);
    state.run()
}

impl State {
    fn new(git_project: GitProject) -> Self {
        Self {
            git_project: Rc::new(RefCell::new(git_project)),
            selected_project: Rc::new(Cell::new(0)),
        }
    }

    fn run(self) -> Result<(), Box<dyn Error>> {
        let mut siv = Cursive::default();

        let file_menu = MenuTree::new()
            .leaf("Save", { let state = self.clone(); move |s| { state.save(s); }})
            .leaf("Quit", { let state = self.clone(); move |s| { state.quit(s); }});
        siv.menubar()
            .add_subtree("File", file_menu);

        self.reload(&mut siv);

        siv.run();
        Ok(())
    }

    fn reload(&self, siv: &mut Cursive) {
        siv.pop_layer();
        let project_view = self.git_project_view(siv);
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

    fn git_project_view(&self, siv: &mut Cursive) -> impl View {
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

        let mut container = LinearLayout::horizontal();
        container.add_child(Panel::new(left_nav));

        if let Some(project) = git_project.projects().iter().nth(self.selected_project.get()) {
            let header = LinearLayout::vertical()
                .child(TextView::new(project.name()))
                .child(TextView::new(project.description().unwrap_or("")));
            let project_board = DummyView;
            let content = LinearLayout::vertical()
                .child(header)
                .child(project_board)
                .full_width();
            container.add_child(Panel::new(content));
        } else {
            let empty_state = TextView::new("No projects found. Create a new one?");
            container.add_child(Panel::new(empty_state));
        }

        let size = siv.screen_size();
        container
            .fixed_size((size.x, size.y - 1))
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
