use cursive::{views::*, view::*};
use super::State;

mod board;
mod column;
mod task;

mod nav;

pub fn view(state: State) -> impl View {
    LinearLayout::horizontal()
        .child(nav::new(state.clone()))
        .child(board::board(state))
        .full_screen()
}
