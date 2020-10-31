use crate::prelude::*;
use crate::app_model::{APP_ID, App};
use crate::app_component::AppMessage;
use crate::password_list_component::{PasswordListHeaderbar, PasswordList, PasswordListMessage};

use vgtk::{ext::*, gtk, VNode};
use vgtk::lib::{gtk::*, gio::{SimpleAction, ApplicationFlags}};

impl View for App {
    fn view(&self) -> VNode<Self> {
        gtk! {
            <Application::new_unwrap(Some(APP_ID), ApplicationFlags::empty())>
                <SimpleAction::new("reload", None)
                    Application::accels=["F5", "<Ctrl>R"].as_ref()
                    on activate=|_, _| AppMessage::PasswordList(PasswordListMessage::Reload)
                />
                <SimpleAction::new("go-back", None)
                    Application::accels=["BackSpace", "<Alt>Left"].as_ref()
                    on activate=|_, _| AppMessage::PasswordList(PasswordListMessage::Back)
                />

                <Window
                    default_height=400
                    default_width=400
                    title="Password Store"
                    on destroy=|_| AppMessage::Exit
                >
                    // NOTE: These model clones might have a severe performance impact as this leads
                    //       to 2 clones of the complete password list model every time the model is
                    //       updated.
                    <@PasswordListHeaderbar
                        model=self.password_list_model.clone()
                        on_message=|msg| AppMessage::PasswordList(msg)
                    />

                    <@PasswordList
                        model=self.password_list_model.clone()
                        on_message=|msg| AppMessage::PasswordList(msg)
                    />
                </Window>
            </Application>
        }
    }
}
