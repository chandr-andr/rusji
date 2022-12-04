use cursive::Cursive;
use cursive::view::Resizable;
use cursive::views::{
    LinearLayout,
    SelectView,
    BoxedView,
    Panel,
    ScrollView,
};


pub fn start_screen() -> std::io::Result<()> {
    let mut cursive = cursive::default();

    let screen_id = cursive.add_active_screen();

    let projects_view = build_projects_view();
    let projects_view2 = build_projects_view();

    let left_layout = LinearLayout::vertical()
        .child(Panel::new(projects_view.full_width().full_height()))
        .child(Panel::new(projects_view2.full_width().full_height()));

    let main_layout = LinearLayout::vertical()
        .child(left_layout.max_width(30).min_height(1000));

    cursive.add_layer(main_layout.full_screen());

    cursive.run();

    Ok(())
}

fn build_projects_view() -> BoxedView {
    let mut select_projects_view = SelectView::<String>::new()
        .on_submit(on_submit_start_screen);

    select_projects_view.add_all_str(
        vec!["Jira", "Quit"],
    );

    BoxedView::new(Box::new(ScrollView::new(select_projects_view)))
}

// fn delete_name(s: &mut Cursive) {
//     let mut select = s.find_name::<SelectView<String>>("select").unwrap();
//     match select.selected_id() {
//         None => s.add_layer(Dialog::info("No name to remove")),
//         Some(focus) => {
//             select.remove_item(focus);
//         }
//     }
// }

// fn add_name(s: &mut Cursive) {
//     fn ok(s: &mut Cursive, name: &str) {
//         s.call_on_name("select", |view: &mut SelectView<String>| {
//             view.add_item_str(name)
//         });
//         s.pop_layer();
//     }

//     s.add_layer(Dialog::around(EditView::new()
//             .on_submit(ok)
//             .with_name("name")
//             .fixed_width(10))
//         .title("Enter a new name")
//         .button("Ok", |s| {
//             let name =
//                 s.call_on_name("name", |view: &mut EditView| {
//                     view.get_content()
//                 }).unwrap();
//             ok(s, &name);
//         })
//         .button("Cancel", |s| {
//             s.pop_layer();
//         }));
// }

// fn on_submit(s: &mut Cursive, name: &str) {
//     s.pop_layer();
//     s.add_layer(Dialog::text(format!("Name: {}\nAwesome: yes", name))
//         .title(format!("{}'s info", name))
//         .button("Back", make_main_page));
// }

fn on_submit_start_screen(cursive: &mut Cursive, action: &String) {
    Cursive::quit(cursive);
}
