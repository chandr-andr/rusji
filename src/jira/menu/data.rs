use std::str::FromStr;

enum MenuVariant {
    IamAssignee,
    IamCreator,
}

struct MenuVariantErr;

impl FromStr for MenuVariant {
    type Err = MenuVariantErr;

    fn from_str(str_menu_variant: &str) -> Result<Self, Self::Err> {
        match str_menu_variant {
            "I'm assignee tasks" => Ok(Self::IamAssignee),
            "I'm creator tasks" => Ok(Self::IamCreator),
            _ => Err(MenuVariantErr {}),
        }
    }
}

impl From<MenuVariant> for &str {
    fn from(menu_variant: MenuVariant) -> Self {
        match menu_variant {
            MenuVariant::IamAssignee => "I'm assignee tasks",
            MenuVariant::IamCreator => "I'm creator tasks",
        }
    }
}

impl MenuVariant {
    pub fn get_menu_variants() -> Vec<&'static str> {
        vec![Self::IamAssignee.into(), Self::IamCreator.into()]
    }
}
