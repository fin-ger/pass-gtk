use vgtk::lib::gtk::*;
use vgtk::lib::glib::{ObjectType, ObjectExt, GString};

pub trait UserDataPathExt: WidgetExt + ObjectType {
    fn set_user_data_path(&self, path: String) {
        unsafe { self.set_data("path", path); }
    }

    fn get_user_data_path(&self) -> String {
        if let Some(path) = unsafe { self.get_data::<String>("path") } {
            path.to_owned()
        } else {
            String::new()
        }
    }
}

impl<A> UserDataPathExt for A where A: WidgetExt + ObjectType {}

pub trait StyleClassExt: WidgetExt {
    fn get_classes(&self) -> Vec<GString> {
        let context = self.get_style_context();
        context.list_classes()
    }

    fn set_classes(&self, classes: &Vec<GString>) {
        let context = self.get_style_context();
        for class in classes {
            context.add_class(class);
        }
    }
}

impl<A> StyleClassExt for A where A: WidgetExt {}

pub trait ScrollPolicyExt: ScrolledWindowExt {
    fn get_hscrollbar_policy(&self) -> PolicyType {
        self.get_policy().0
    }

    fn get_vscrollbar_policy(&self) -> PolicyType {
        self.get_policy().1
    }

    fn set_hscrollbar_policy(&self, policy: PolicyType) {
        self.set_policy(policy, self.get_vscrollbar_policy());
    }

    fn set_vscrollbar_policy(&self, policy: PolicyType) {
        self.set_policy(self.get_hscrollbar_policy(), policy);
    }
}

impl<A> ScrollPolicyExt for A where A: ScrolledWindowExt {}
