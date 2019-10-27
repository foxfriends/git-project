use std::error::Error;
use pancurses::{ALL_MOUSE_EVENTS, initscr, curs_set, endwin, mousemask, noecho, Input};
use crate::model::GitProject;

pub fn open() -> Result<(), Box<dyn Error>> {
    let mut git_project = GitProject::open()?;

    let window = initscr();
    window.keypad(true);
    curs_set(0);
    mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut());
    noecho();


    loop {
        match window.getch() {
            Some(Input::Character('q')) => break,
            _ => {}
        }
    }

    endwin();
    Ok(())
}
