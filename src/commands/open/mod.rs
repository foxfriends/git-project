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
    unsaved_changes: Rc<Cell<bool>>,
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
            unsaved_changes: Rc::new(Cell::new(false)),
            current_user,
        }
    }

    fn run(self) -> Result<(), Box<dyn Error>> {
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

    fn new_task(&self, siv: &mut Cursive) {
        let git_project = self.git_project.borrow();
        let project = &git_project.projects()[self.selected_project.get()];

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
            .button("Discard", { let state = self.clone(); let selected_tags = selected_tags.clone(); move |s| { 
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
            .button("Save", { let state = self.clone(); move |s| { 
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
