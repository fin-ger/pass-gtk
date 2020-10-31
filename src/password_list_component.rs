use crate::prelude::*;
use crate::password_list_model::PasswordListModel;

use vgtk::{Component, UpdateAction, Callback, VNode};

#[derive(Clone, Debug)]
pub enum PasswordListMessage {
    Reload,
    Back,
    ShowDirectory(String),
    ShowPassword(String),
}

#[derive(Clone, Debug, Default)]
pub struct PasswordListHeaderbar {
    pub model: PasswordListModel,
    pub on_message: Callback<PasswordListMessage>,
}

impl Component for PasswordListHeaderbar {
    type Message = PasswordListMessage;
    type Properties = Self;

    fn create(props: Self) -> Self {
        props
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }

    fn update(&mut self, message: PasswordListMessage) -> UpdateAction<Self> {
        self.on_message.send(message);
        UpdateAction::Render
    }

    fn view(&self) -> VNode<Self> {
        View::view(self)
    }
}

#[derive(Clone, Debug, Default)]
pub struct PasswordList {
    pub model: PasswordListModel,
    pub on_message: Callback<PasswordListMessage>,
}

impl Component for PasswordList {
    type Message = PasswordListMessage;
    type Properties = Self;

    fn create(props: Self) -> Self {
        props
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }

    fn update(&mut self, message: PasswordListMessage) -> UpdateAction<Self> {
        self.on_message.send(message);
        UpdateAction::Render
    }

    fn view(&self) -> VNode<Self> {
        View::view(self)
    }
}

impl PasswordListModel {
    pub fn update(&mut self, message: PasswordListMessage) {
        match message {
            PasswordListMessage::Reload => {
                self.reload_passwords();
            },
            PasswordListMessage::Back => {
                if self.can_go_back() {
                    if self.current_password.is_some() {
                        self.current_password = None;
                    } else {
                        self.history.pop();
                    }
                }
            },
            PasswordListMessage::ShowDirectory(directory_path) => {
                self.history = directory_path
                    .split("/")
                    .map(|s| s.to_owned())
                    .collect();
                self.current_password = None;
            },
            PasswordListMessage::ShowPassword(password) => {
                self.current_password = Some(password);
            },
        }
    }
}
