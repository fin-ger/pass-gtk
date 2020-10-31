use crate::prelude::*;
use crate::password_list_model::Entry;
use crate::password_list_component::{PasswordListHeaderbar, PasswordList, PasswordListMessage};

use vgtk::{ext::*, gtk, VNode};
use vgtk::lib::gtk::*;
use id_tree::NodeId;
use pango::EllipsizeMode;

impl View for PasswordListHeaderbar {
    fn view(&self) -> VNode<Self> {
        let main_menu = vgtk::menu()
            .section(
                vgtk::menu()
                    .item("Reload", "app.reload")
            )
            .section(
                vgtk::menu()
                    .item("Preferences", "app.preferences")
                    .item("About", "app.about")
            )
            .build();
        let main_menu_popover = Popover::from_model(None as Option<&MenuButton>, &main_menu);

        gtk! {
            <HeaderBar
                show_close_button=true
             >
                // This label is a hack to prevent GTK from automatically inserting the window title
                // into the titlebar. By setting this label as a custom title and not setting any
                // content, the title is invisible.
                <Label
                    HeaderBar::custom_title=true
                />

                // The back button in the upper left corner
                <Button
                    HeaderBar::pack_type=PackType::Start
                    sensitive=self.model.can_go_back()
                    image="go-previous-symbolic"
                    on clicked=|_| PasswordListMessage::Back
                />

                // The path bar containing the navigatable password path
                <Box
                    HeaderBar::pack_type=PackType::Start
                    spacing=2
                >
                    // The 'Home' button is always visible
                    <ToggleButton
                        relief=ReliefStyle::None
                        property_draw_indicator=true
                        focus_on_click=false
                        active=self.model.is_active_page(".")
                        image="go-home-symbolic"
                        on clicked=|_| PasswordListMessage::ShowDirectory(".".into())
                    />

                    // The password navigation bar
                    {
                        self.model.history
                            .iter()
                            .scan(vec![], |path, dir| {
                                path.push(dir.to_owned());

                                let path_str = path.join("/");
                                Some(gtk! {
                                    <ToggleButton
                                        relief=ReliefStyle::None
                                        property_draw_indicator=true
                                        focus_on_click=false
                                        tooltip_text=dir.to_owned()
                                        active=self.model.is_active_page(&path_str)
                                        user_data_path=path_str
                                        on clicked=|btn| PasswordListMessage::ShowDirectory(btn.get_user_data_path())
                                    >
                                        <Label
                                            label=dir.to_owned()
                                            ellipsize=EllipsizeMode::Middle
                                        />
                                    </ToggleButton>
                                })
                            })
                            // throw away the "." button
                            .skip(1)
                            // put a / between all buttons
                            .flat_map(|btn| vec![gtk! { <Label label="/" /> }, btn])
                    }

                    // The password in the navigation bar, if a password is shown/edited
                    {
                        if let Some(ref password) = self.model.current_password {
                            let name = password.split("/").last().unwrap().to_owned();
                            vec![
                                gtk! { <Label label="/" /> },
                                gtk! {
                                    <ToggleButton
                                        relief=ReliefStyle::None
                                        property_draw_indicator=true
                                        focus_on_click=false
                                        tooltip_text=name.to_owned()
                                        active=true
                                        user_data_path=password.to_owned()
                                        on clicked=|btn| PasswordListMessage::ShowPassword(btn.get_user_data_path())
                                    >
                                        <Label
                                            label=name
                                            ellipsize=EllipsizeMode::Middle
                                        />
                                    </ToggleButton>
                                },
                            ]
                        } else {
                            vec![]
                        }
                    }
                </Box>

                // The main hamburger menu button
                <MenuButton
                    HeaderBar::pack_type=PackType::End
                    @MenuButtonExt::direction=ArrowType::Down
                    popover=Some(main_menu_popover)
                    image="open-menu-symbolic"
                />
            </HeaderBar>
        }
    }
}

impl View for PasswordList {
    fn view(&self) -> VNode<Self> {
        gtk! {
            <ScrolledWindow
                hscrollbar_policy=PolicyType::Never
                vscrollbar_policy=PolicyType::Automatic
            >
                <Frame
                    border_width=24
                    shadow_type=ShadowType::EtchedIn
                >
                    <Stack
                        visible_child_name=self.model.visible_page()
                        homogeneous=false
                        transition_type=StackTransitionType::SlideLeftRight
                    >
                        // pages of the password list
                        {
                            let root = self.model.passwords.root_node_id().unwrap();
                            let entry = match self.model.passwords.get(root).unwrap().data() {
                                Entry::Password(name) => name,
                                Entry::Directory(name) => name,
                            };
                            self.view_password_list(root, entry.clone())
                        }

                        // dummy page for now to show a password
                        <Box
                            orientation=Orientation::Vertical
                            spacing=8 Stack::name="show_password"
                        >
                            <Label
                                label=self.model.current_password.clone().unwrap_or("".into())
                            />
                            <Label
                                label="Not implemented yet"
                            />
                        </Box>
                    </Stack>
                </Frame>
            </ScrolledWindow>
        }
    }
}

impl PasswordList {
    fn view_password_list(&self, node: &NodeId, name: String) -> Vec<VNode<Self>> {
        let child_lists = self.model.passwords.children_ids(node).unwrap().flat_map(|child| {
            if let Entry::Directory(child_name) = self.model.passwords.get(child).unwrap().data() {
                self.view_password_list(child, format!("{}/{}", name, child_name))
            } else {
                vec![]
            }
        });

        std::iter::once(
            gtk! {
                <ListBox Stack::name=name.clone()
                         on row_activated=|_, row| {
                             let widget_name = row.get_widget_name();
                             if widget_name == "show_password" {
                                 PasswordListMessage::ShowPassword(row.get_user_data_path())
                             } else {
                                 PasswordListMessage::ShowDirectory(widget_name.into())
                             }
                         }>
                {
                    self.model.passwords.children_ids(node).unwrap().map(|child| {
                        let children = self.model.passwords.children_ids(child).unwrap().count();
                        let child = self.model.passwords.get(child).unwrap().data().clone();
                        match child {
                            Entry::Password(child_name) => {
                                gtk! {
                                    <ListBoxRow selectable=false
                                                user_data_path=format!("{}/{}", name, child_name)
                                                widget_name="show_password">
                                        <Box orientation=Orientation::Horizontal
                                             spacing=16
                                             margin_top=4
                                             margin_bottom=4
                                             margin_start=8
                                             margin_end=8>
                                            <Image property_icon_name="dialog-password"
                                                   property_icon_size=3 />
                                            <Label label=child_name />
                                        </Box>
                                    </ListBoxRow>
                                }
                            },
                            Entry::Directory(child_name) => {
                                gtk! {
                                    <ListBoxRow selectable=false
                                                widget_name=format!("{}/{}", name, child_name)>
                                        <Box orientation=Orientation::Horizontal
                                             spacing=16
                                             margin_top=4
                                             margin_bottom=4
                                             margin_start=8
                                             margin_end=8>
                                            <Image property_icon_name="folder"
                                                   property_icon_size=3 />
                                            <Box orientation=Orientation::Vertical spacing=2>
                                                <Label markup=format!("<b>{}</b>", child_name) xalign=0.0 />
                                                <Label markup=format!(
                                                           "<small>{} {}</small>",
                                                           children,
                                                           if children == 1 { "Password" } else { "Passwords" }
                                                       )
                                                       xalign=0.0
                                                       classes=vec!["dim-label".into()] />
                                            </Box>
                                            <Image property_icon_name="go-next-symbolic"
                                                   hexpand=true
                                                   halign=Align::End />
                                        </Box>
                                    </ListBoxRow>
                                }
                            },
                        }
                    })
                }
                </ListBox>
            },
        ).chain(child_lists).collect()
    }
}
