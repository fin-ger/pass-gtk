use crate::password_list_model::SharedPasswordListModel;

pub const APP_ID: &str = "io.github.fin_ger.PasswordStore";

#[derive(Clone, Debug, Default)]
pub struct App {
    pub password_list_model: SharedPasswordListModel,
}

impl App {
}
