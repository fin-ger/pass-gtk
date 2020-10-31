use crate::prelude::*;
use crate::app_model::App;
use crate::password_list_component::PasswordListMessage;

use vgtk::{Component, UpdateAction, VNode};

#[derive(Clone, Debug)]
pub enum AppMessage {
    Exit,
    PasswordList(PasswordListMessage),
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn update(&mut self, message: AppMessage) -> UpdateAction<Self> {
        match message {
            AppMessage::Exit => {
                vgtk::quit();
                UpdateAction::None
            },
            AppMessage::PasswordList(msg) => {
                self.password_list_model.update(msg);
                UpdateAction::Render
            },
        }
    }

    fn view(&self) -> VNode<Self> {
        View::view(self)
    }
}
