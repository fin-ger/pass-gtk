use crate::prelude::*;
use crate::password_list_model::SharedPasswordListModel;

use vgtk::{Component, UpdateAction, VNode};

#[derive(Clone, Debug)]
pub enum PasswordListMessage {
    Reload,
    ShowDirectory(String),
    ShowPassword(String),
    Back,
}

#[derive(Clone, Debug, Default)]
pub struct PasswordListHeaderbar {
    pub model: SharedPasswordListModel,
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
        match message {
            _ => UpdateAction::None
        }
    }

    fn view(&self) -> VNode<Self> {
        View::view(self)
    }
}

#[derive(Clone, Debug, Default)]
pub struct PasswordList {
    pub model: SharedPasswordListModel,
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
        match message {
            PasswordListMessage::Reload => {
                self.model.reload_passwords();
                UpdateAction::Render
            },
            PasswordListMessage::Back => {
                if self.model.can_go_back() {
                    if self.model.current_password.is_some() {
                        self.model.current_password = None;
                    } else {
                        self.model.history.pop();
                    }

                    UpdateAction::Render
                } else {
                    UpdateAction::None
                }
            },
            PasswordListMessage::ShowDirectory(directory_path) => {
                self.model.history = directory_path
                    .split("/")
                    .map(|s| s.to_owned())
                    .collect();
                self.model.current_password = None;
                UpdateAction::Render
            },
            PasswordListMessage::ShowPassword(password) => {
                self.model.current_password = Some(password);
                UpdateAction::Render
            },
        }
    }

    fn view(&self) -> VNode<Self> {
        View::view(self)
    }
}
