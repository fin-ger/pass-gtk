use crate::password_list_model::PasswordListModel;

pub const APP_ID: &str = "io.github.fin_ger.PasswordStore";

#[derive(Clone, Debug, Default)]
pub struct App {
    pub password_list_model: PasswordListModel,
}

impl App {
}
