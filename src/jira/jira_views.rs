use cursive::{views::{LinearLayout, SelectView, Panel, ScrollView, BoxedView, ResizedView}, view::Resizable};

struct ProjectsView {
    view: Panel<ResizedView<ResizedView<SelectView>>>,
}

impl ProjectsView {
    fn new() -> ProjectsView {
        let mut select_projects_view = SelectView::<String>::new();

        select_projects_view.add_all_str(
            vec!["Jira", "Quit"],
        );
        ProjectsView {
            view: Panel::new(select_projects_view.full_width().full_height()),
        }
    }
}

struct TasksView {
    view: Panel<ResizedView<ResizedView<SelectView>>>,
}

impl TasksView {
    fn new() -> TasksView {
        let mut select_tasks_view = SelectView::<String>::new();

            select_tasks_view.add_all_str(
            vec!["FRE-1", "FRE-2"],
        );
        TasksView {
            view: Panel::new(select_tasks_view.full_width().full_height()),
        }
    }
}

struct ProjectsTasksLayer {
    layer: LinearLayout,
    projects_view: ProjectsView,
    tasks_view: TasksView,
}

impl ProjectsTasksLayer {
    fn new() -> LinearLayout {
        // let projects_view = BoxedView::new(
        //     Box::new(
        //         ScrollView::new(
        //             ProjectsView::new().view,
        //         )
        //     )
        // );
        // let tasks_view = BoxedView::new(
        //     Box::new(
        //         ScrollView::new(
        //             TasksView::new().view,
        //         )
        //     )
        // );

        let projects_view = ProjectsView::new().view;
        let tasks_view = TasksView::new().view;

        LinearLayout::vertical()
        .child(projects_view)
        .child(tasks_view)
    }
}

struct JiraLayer {
    layer: LinearLayout,
}

struct Screen {
    main_layer: JiraLayer,
    project_tasks_layer: ProjectsTasksLayer,
}
