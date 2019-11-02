use std::rc::Rc;
use std::cell::RefCell;
use cursive::{views::*, view::*, event};
use super::super::State;
use crate::model::*;

pub fn new(state: State) -> impl View {
    let git_project = state.git_project.borrow();
    let project = &git_project.projects()[state.selected_project.get()];

    let id = LinearLayout::horizontal()
        .child(TextView::new("Task ID").fixed_width(12))
        .child(EditView::new()
               .with_id("new-task-id")
               .full_width());

    let title = LinearLayout::horizontal()
        .child(TextView::new("Title").fixed_width(12))
        .child(EditView::new()
               .with_id("new-task-title")
               .full_width());

    let assignee = LinearLayout::horizontal()
        .child(TextView::new("Assignee").fixed_width(12))
        .child(project.all_assignees().into_iter()
               .fold(SelectView::new().item("None", None), |sel, assignee| sel.item(assignee, Some(assignee.to_string())))
               .popup()
               .autojump()
               .with_id("new-task-assignee"));

    let column = LinearLayout::horizontal()
        .child(TextView::new("Column").fixed_width(12))
        .child(project.columns().into_iter()
               .map(|col| col.name())
               .enumerate()
               .fold(SelectView::<usize>::new(), |sel, (i, col)| sel.item(col, i))
               .popup()
               .autojump()
               .with_id("new-task-column"));

    let description = LinearLayout::horizontal()
        .child(TextView::new("Description").fixed_width(12))
        .child(TextArea::new()
               .with_id("new-task-description")
               .full_width()
               .min_height(5));

    let selected_tags = Rc::new(RefCell::new(vec![]));
    let tags = LinearLayout::horizontal()
        .child(LinearLayout::vertical().child(DummyView).child(TextView::new("Tags").fixed_width(12)))
        .child(Panel::new(OnEventView::new(SelectView::<String>::new().with_id("selected-tags"))
                          .on_event(event::Key::Del, { let selected_tags = selected_tags.clone(); move |s| {
                              // remove a tag from the list:
                              // 1.  Remove from selected tags
                              // 2.  Remove from selected list
                              // 3.  Add to suggestions
                              let mut selected_tags_view = s.find_id::<SelectView>("selected-tags").unwrap();
                              if let Some(id) = selected_tags_view.selected_id() {
                                  let tag = selected_tags.borrow_mut().remove(id);
                                  selected_tags_view.remove_item(id)(s);
                                  let mut suggested_tags_view = s.find_id::<SelectView>("suggested-tags").unwrap();
                                  suggested_tags_view.add_item_str(tag);
                              }
                          }})
                          .min_size((10, 2))))
        .child(LinearLayout::vertical()
               .child(DummyView)
               .child(project.all_tags().into_iter()
                      .fold(SelectView::<String>::new().item("Add tag", "".to_string()), SelectView::item_str)
                      .on_submit({ let selected_tags = selected_tags.clone(); move |s, tag: &String| {
                          // add a tag from the list:
                          // 1.  Add to selected tags
                          // 2.  Add to selected list
                          // 3.  Remove from suggestions
                          if tag.is_empty() { return }
                          let mut selected_tags_view = s.find_id::<SelectView>("selected-tags").unwrap();
                          selected_tags.borrow_mut().push(tag.clone());
                          selected_tags_view.add_item_str(tag);
                          let mut suggested_tags_view = s.find_id::<SelectView>("suggested-tags").unwrap();
                          let id = suggested_tags_view.selected_id().unwrap();
                          suggested_tags_view.remove_item(id);
                          suggested_tags_view.set_selection(0);
                      }})
                      .popup()
                      .with_id("suggested-tags"))
               .child(EditView::new()
                      .on_submit_mut({ let selected_tags = selected_tags.clone(); move |s, tag| {
                          // add a newly invented tag
                          // 1.  Add to selected tags
                          // 2.  Add to selected list
                          // 3.  Remove from suggestions
                          // 4.  Reset input field
                          let tag = tag.trim();
                          if tag.contains(|ch: char| ch.is_whitespace() || ch == ',') || tag.is_empty() { return }
                          if selected_tags.borrow().contains(&tag.to_string()) { return }
                          selected_tags.borrow_mut().push(tag.to_string());

                          let mut selected_tags_view = s.find_id::<SelectView>("selected-tags").unwrap();
                          selected_tags_view.add_item_str(tag);

                          let mut suggested_tags_view = s.find_id::<SelectView>("suggested-tags").unwrap();
                          let position = suggested_tags_view.iter().position(|(_, suggestion)| suggestion == tag);
                          if let Some(position) = position {
                              suggested_tags_view.remove_item(position)(s);
                          }

                          let mut edit_view = s.find_id::<EditView>("tag-input").unwrap();
                          edit_view.set_content("")(s);
                      }})
    .with_id("tag-input")
        .fixed_width(20)));

    let form = LinearLayout::vertical()
        .child(id)
        .child(DummyView)
        .child(title)
        .child(DummyView)
        .child(assignee)
        .child(DummyView)
        .child(column)
        .child(DummyView)
        .child(description)
        .child(DummyView)
        .child(tags);

    let form_dialog = Dialog::around(PaddedView::new((0, 0, 1, 0), form))
        .button("Discard", { let state = state.clone(); let selected_tags = selected_tags.clone(); move |s| { 
            let id = s.find_id::<EditView>("new-task-id").unwrap().get_content().to_string();
            let title = s.find_id::<EditView>("new-task-title").unwrap().get_content().to_string();
            let assignee = s.find_id::<SelectView<Option<String>>>("new-task-assignee").unwrap().selection().and_then(|rc| (*rc).clone());
            let description = s.find_id::<TextArea>("new-task-description").unwrap().get_content().to_string();
            if !id.is_empty() || !title.is_empty() || assignee.is_some() || !description.is_empty() || !selected_tags.borrow().is_empty() {
                state.confirm(s, "Discard new task?", |s| { s.pop_layer(); });
            } else {
                s.pop_layer();
            }
        }})
    .button("Save", { let state = state.clone(); move |s| { 
        let id = s.find_id::<EditView>("new-task-id").unwrap().get_content().trim().to_string();
        let title = s.find_id::<EditView>("new-task-title").unwrap().get_content().trim().to_string();
        let assignee = s.find_id::<SelectView<Option<String>>>("new-task-assignee").unwrap().selection().and_then(|rc| (*rc).clone());
        let column = *s.find_id::<SelectView<usize>>("new-task-column").unwrap().selection().unwrap();
        let description = s.find_id::<TextArea>("new-task-description").unwrap().get_content().trim().to_string();
        if id.is_empty() || title.is_empty() || description.is_empty() {
            s.add_layer(Dialog::info("Required information is missing"));
            return;
        }
        let mut task = Task::new(&id).name(title).description(description);
        if let Some(assignee) = assignee { task = task.assignee(assignee); }
        let task = selected_tags.borrow().iter().fold(task, |task, tag| task.tag(tag)).build().unwrap();
        let success = state.git_project
            .borrow_mut()
            .projects_mut()
            [state.selected_project.get()]
            .add_task(task, column);
        if success {
            s.pop_layer(); 
            state.reload(s);
        } else {
            s.add_layer(Dialog::info(format!("A task with ID {} already exists", id)));
        }
    }});

    form_dialog
}
