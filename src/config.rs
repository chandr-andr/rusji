use crate::utils::*;
use serde_json::{Value, Map};
use crate::cli::RegisterJira;

fn get_config_in_json() -> std::io::Result<Value> {
    let path_to_app_config = build_app_config_path()?;

    let app_config = {
        let text = std::fs::read_to_string(
            &path_to_app_config,
        )?;

        serde_json::from_str::<Value>(&text)?
    };
    Ok(app_config)
}

pub fn add_new_jira_project(reg: &RegisterJira) -> std::io::Result<()> {
    let mut app_config = get_config_in_json()?;

    let companies = &app_config["Jira"]["Companies"];

    match companies {
        Value::Array(companies_map) => {
            let mut companies_list = companies_map.clone();

            let mut new_company_map: Map<String, Value> = Map::new();
            for (key, value) in reg.make_tuple_of_struct() {
                let value_v = value.clone();
                new_company_map.insert(key, Value::String(value_v));
            }
            companies_list.push(Value::Object(new_company_map));
            app_config["Jira"]["Companies"] = Value::Array(companies_list);
        },
        _ => (),
    }

    let app_config_str = app_config.to_string();

    let config_path = build_app_config_path()?;
    std::fs::write(config_path, app_config_str)?;

    Ok(())
}
