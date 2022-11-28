use cursive::Cursive;
use cursive::views::{
    Button,
    Dialog,
    DummyView,
    EditView,
    LinearLayout,
    SelectView,
};
use crate::config::get_all_jira_companies;


pub fn start_screen() -> std::io::Result<()> {
    let mut cursive = cursive::default();
    let mut start_screen_select = SelectView::<String>::new()
        .on_submit(on_submit_start_screen);

    start_screen_select.add_all_str(
        vec!["Jira", "Quit"],
    );

    cursive.add_layer(Dialog::around(LinearLayout::horizontal()
        .child(start_screen_select)
        .child(DummyView)));

    cursive.run();

    Ok(())
}

fn make_main_page(cursive: &mut Cursive) {
    cursive.pop_layer();
    let all_companies = get_all_jira_companies();
    match all_companies {
        Ok(companies) => {
            let mut select = SelectView::<String>::new()
                .on_submit(on_submit);

            for company_name in companies {
                select.add_item_str(company_name)
            }
            cursive.add_layer(Dialog::around(LinearLayout::horizontal()
                .child(select)
                .child(DummyView))
                .title("Select a company"));
        },
        Err(err) => {
            cursive.pop_layer();
            cursive.add_layer(Dialog::text(format!("Error: {}", err)));
        }
    }
}

fn show_all_company_projects(cursive: &mut Cursive, company_name: &'static str) {

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

fn on_submit(s: &mut Cursive, name: &str) {
    s.pop_layer();
    s.add_layer(Dialog::text(format!("Name: {}\nAwesome: yes", name))
        .title(format!("{}'s info", name))
        .button("Back", make_main_page));
}

fn on_submit_start_screen(cursive: &mut Cursive, action: &str) {
    cursive.pop_layer();
    if action == "Jira" {
        make_main_page(cursive)
    } else {
        Cursive::quit(cursive);
    }
}
