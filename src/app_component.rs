use crate::prelude::*;
use crate::app_model::App;

use vgtk::{Component, UpdateAction, VNode};

#[derive(Clone, Debug)]
pub enum AppMessage {
    Exit,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn update(&mut self, message: AppMessage) -> UpdateAction<Self> {
        match message {
            AppMessage::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        View::view(self)
    }
}
