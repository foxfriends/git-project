use std::error::Error;
use cursive::Cursive;
use cursive::views::*;
use cursive::traits::*;
use crate::model::GitProject;

#[derive(Debug)]
struct State {
    git_project: GitProject,
}

pub fn open() -> Result<(), Box<dyn Error>> {
    let git_project = GitProject::open()?;
    let state = State::new(git_project);
    state.run()
}

impl State {
    fn new(git_project: GitProject) -> Self {
        Self { git_project }
    }

    fn run(self) -> Result<(), Box<dyn Error>> {
        let mut siv = Cursive::default();
        siv.add_global_callback('?', show_help);
        siv.run();
        Ok(())
    }
}

fn show_help(siv: &mut Cursive) {
    let help_text = LinearLayout::vertical()
        .child(TextView::new("?: Show this help"));

    let dialog = Dialog::around(help_text)
        .title("Help")
        .button("Ok", |s| { s.pop_layer(); });

    let event_handler = OnEventView::new(dialog)
        .on_event('?', Cursive::noop);

    siv.add_layer(event_handler);
}
