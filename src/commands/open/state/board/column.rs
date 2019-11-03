use cursive::{views::*, view::*};
use super::State;
use super::task;
use crate::model::*;

pub fn column(state: State, project: &Project, column: &Column) -> impl View {
    let column_view = column.tasks().iter()
        .filter_map(|task_id| project.task_with_id(task_id))
        .map(|t| task::card(state.clone(), t))
        .fold(LinearLayout::vertical(), LinearLayout::child);

    let scroll_view = ScrollView::new(column_view)
        .full_height()
        .fixed_width(80);

    Panel::new(scroll_view)
        .title(column.name())
}
