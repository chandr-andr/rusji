use cursive::{
    views::{
        LinearLayout,
        SelectView,
        ResizedView,
        ScrollView,
        TextView,
        ViewRef,
        DummyView,
        NamedView,
    },
    view::{Resizable, Nameable},
    Cursive,
};
use cursive::align;

const INNER_VIEW_ALIGN: align::Align = align::Align {
    h: align::HAlign::Center,
    v: align::VAlign::Top,
};


pub fn make_jira_screen(cursive: &mut Cursive) {
    let screen_size = cursive.screen_size();
    let side_width = screen_size.x * 2 / 7;
    let center_width = screen_size.x * 3 / 7;

    let mut main_layer = make_main_layer();

    let tasks_projects_layer = make_tasks_projects_layer();
    let info_layer = make_info_layer();
    let actions_something_layer = make_actions_something_layer();

    main_layer.add_child(tasks_projects_layer.min_width(side_width));
    main_layer.add_child(info_layer.min_width(center_width));
    main_layer.add_child(actions_something_layer.min_width(side_width));

    cursive.add_layer(main_layer);
}

fn make_main_layer() -> LinearLayout {
    LinearLayout::horizontal()
}

fn make_tasks_projects_layer() -> LinearLayout {
    let mut tasks_projects_layer = LinearLayout::vertical();

    tasks_projects_layer.add_child(make_projects_view());
    tasks_projects_layer.add_child(DummyView);
    tasks_projects_layer.add_child(make_tasks_view());

    tasks_projects_layer
}

fn make_info_layer() -> LinearLayout {
    let mut info_layer = LinearLayout::vertical();

    info_layer.add_child(make_info_view());

    info_layer
}

fn make_actions_something_layer() -> LinearLayout {
    let mut info_layer = LinearLayout::vertical();
    info_layer.add_child(make_actions_view());
    info_layer.add_child(DummyView);
    info_layer.add_child(make_something_view());
    info_layer
}

fn make_projects_view() -> ResizedView<ScrollView<NamedView<SelectView>>> {
    let mut inner_projects_view = SelectView::<String>::new()
        .align(INNER_VIEW_ALIGN)
        .on_submit(on_select_project);
    inner_projects_view.add_all_str(
        vec!["Project1", "Project2", "Project3", "Project4"]
    );
    ScrollView::new(inner_projects_view.with_name("projects_view")).full_height()
}

fn make_tasks_view() -> ResizedView<ScrollView<NamedView<SelectView>>> {
    let mut inner_tasks_view = SelectView::<String>::new()
        .align(INNER_VIEW_ALIGN);
    inner_tasks_view.add_all_str(
        vec!["Project1", "Project2", "Project3", "Project4"]
    );
    ScrollView::new(inner_tasks_view.with_name("tasks_view")).full_height()
}

fn make_info_view() -> ScrollView<NamedView<TextView>> {
    let mut inner_info_view = TextView::new("")
        .align(INNER_VIEW_ALIGN);

    inner_info_view.set_content("CONTENT");

    ScrollView::new(inner_info_view.with_name("info_view"))
}

fn make_actions_view() -> ResizedView<ScrollView<NamedView<SelectView>>> {
    let mut inner_actions_view = SelectView::<String>::new()
        .align(INNER_VIEW_ALIGN);

    inner_actions_view.add_all_str(vec!["Action1", "Action2", "Action3", "Action4"]);

    ScrollView::new(inner_actions_view.with_name("actions_view")).full_height()
}

fn make_something_view() -> ResizedView<ScrollView<NamedView<SelectView>>> {
    let mut inner_something_view = SelectView::<String>::new()
        .align(INNER_VIEW_ALIGN)
        .with_name("tasks_view");
    ScrollView::new(inner_something_view).full_height()
}

fn on_select_project(cursive: &mut Cursive, action: &String) {
    let mut view: ViewRef<TextView> = cursive.find_name("info_view").unwrap();
    view.set_content("YES!")
}
