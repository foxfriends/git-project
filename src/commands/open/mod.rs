use std::error::Error;
use pancurses::{ALL_MOUSE_EVENTS, initscr, curs_set, endwin, mousemask, noecho, resize_term, Input, Window};
use crate::model::GitProject;

#[derive(Default, Debug)]
struct State {
    cmd_typed: String,
    status: String,
}

impl State {
    fn set_cmd_typed<I: AsRef<str>>(&mut self, cmd_typed: I) {
        self.cmd_typed = cmd_typed.as_ref().to_string();
    }

    fn set_status<I: AsRef<str>>(&mut self, status: I) {
        self.status = status.as_ref().to_string();
    }

    fn draw(&self, git_project: &GitProject, window: &Window) {
        let (height, width) = window.get_max_yx();
        let status_line = format!("{: <2$} {: <8}", self.status, self.cmd_typed, width as usize - 11);
        window.mvprintw(height - 1, 1, status_line);
        window.refresh();
    }
}

pub fn open() -> Result<(), Box<dyn Error>> {
    let mut git_project = GitProject::open()?;

    let window = initscr();
    window.keypad(true);
    curs_set(0);
    mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut());
    noecho();

    match run(git_project, &window) {
        Ok(()) => { endwin(); }
        error => {
            endwin();
            return error
        }
    }

    Ok(())
}

fn run(git_project: GitProject, window: &Window) -> Result<(), Box<dyn Error>> {
    let mut state = State::default();
    loop {
        match window.getch() {
            Some(Input::Character('q')) => { quit(&mut state, &git_project, window)?; break }
            Some(Input::Character('w')) => { save(&mut state, &git_project, window)?; }
            Some(Input::Character('Q')) => if force_quit(&mut state, &git_project, window) { break },
            Some(Input::Character('?')) => { show_help(&mut state, &git_project, window); }
            Some(Input::KeyResize) => { resize_term(0, 0); }
            _ => {}
        }
    }
    Ok(())
}

fn show_help(state: &mut State, git_project: &GitProject, window: &Window) {
    window.mvprintw(1, 0, r#"
    Help: git-project UI commands
    When finished, press any key to close this page.

    ?
        Show this help
    q
        Close the git-project UI, saving all changes.
    QQ
        Close the git-project UI, discarding all changes.
    w
        Save changes without quitting.
    "#);

    loop {
        window.refresh();
        match window.getch() {
            Some(Input::KeyResize) => { resize_term(0, 0); }
            Some(Input::Character(..)) => break,
            _ => {},
        }
    }

    window.clear();
    state.set_cmd_typed("?");
    state.draw(git_project, window);
}

fn quit(state: &mut State, git_project: &GitProject, window: &Window) -> Result<(), Box<dyn Error>> {
    state.set_cmd_typed("q");
    state.set_status("Saving...");
    state.draw(&git_project, window);
    git_project.save()?;
    Ok(())
}

fn save(state: &mut State, git_project: &GitProject, window: &Window) -> Result<(), Box<dyn Error>> {
    state.set_cmd_typed("w");
    state.set_status("Saving...");
    state.draw(&git_project, window);
    git_project.save()?;
    state.set_status("Saved!");
    state.draw(&git_project, window);
    Ok(())
}

fn force_quit(state: &mut State, git_project: &GitProject, window: &Window) -> bool {
    state.set_status("Really discard all changes and quit? Press Q to force quit.");
    state.set_cmd_typed("Q_");
    loop {
        state.draw(&git_project, window);
        match window.getch() {
            Some(Input::Character('Q')) => break true,
            Some(Input::KeyResize) => { resize_term(0, 0); }
            _ => {
                state.set_status("Aborted.");
                state.set_cmd_typed("");
                state.draw(git_project, window);
                break false
            }
        }
    }
}
