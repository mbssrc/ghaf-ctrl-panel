use glib::subclass::Signal;
use glib::Binding;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Image, Label, MenuButton, Popover};
use std::cell::RefCell;
use std::sync::OnceLock;

use crate::audio_settings::AudioSettings;
use crate::control_action::ControlAction;
use crate::security_icon::SecurityIcon;
use crate::service_gobject::ServiceGObject;

mod imp {
    use super::*;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/org/gnome/controlpanelgui/ui/service_settings.ui")]
    pub struct ServiceSettings {
        #[template_child]
        pub name_slot_1: TemplateChild<Label>,
        #[template_child]
        pub name_slot_2: TemplateChild<Label>,
        #[template_child]
        pub status_label: TemplateChild<Label>,
        #[template_child]
        pub status_icon: TemplateChild<Image>,
        #[template_child]
        pub details_label: TemplateChild<Label>,
        #[template_child]
        pub security_icon: TemplateChild<Image>,
        #[template_child]
        pub security_label: TemplateChild<Label>,
        #[template_child]
        pub audio_settings_box: TemplateChild<AudioSettings>,
        #[template_child]
        pub control_label: TemplateChild<Label>,
        #[template_child]
        pub action_menu_button: TemplateChild<MenuButton>,
        #[template_child]
        pub popover_menu: TemplateChild<Popover>,

        // Vector holding the bindings to properties of `Object`
        pub bindings: RefCell<Vec<Binding>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ServiceSettings {
        const NAME: &'static str = "ServiceSettings";
        type Type = super::ServiceSettings;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl ServiceSettings {
        #[template_callback]
        fn on_start_clicked(&self) {
            let name = self.name_slot_1.label();
            let display_name = self.name_slot_2.label();
            self.obj().emit_by_name::<()>(
                "vm-control-action",
                &[&ControlAction::Start, &name, &display_name],
            );
            self.popover_menu.popdown();
        }
        #[template_callback]
        fn on_shutdown_clicked(&self) {
            let name = self.name_slot_1.label();
            let display_name = self.name_slot_2.label();
            self.obj().emit_by_name::<()>(
                "vm-control-action",
                &[&ControlAction::Shutdown, &name, &display_name],
            );
            self.popover_menu.popdown();
        }
        #[template_callback]
        fn on_pause_clicked(&self) {
            let name = self.name_slot_1.label();
            let display_name = self.name_slot_2.label();
            self.obj().emit_by_name::<()>(
                "vm-control-action",
                &[&ControlAction::Pause, &name, &display_name],
            );
            self.popover_menu.popdown();
        }
        #[template_callback]
        fn on_mic_changed(&self, value: u32) {
            println!("Mic changed: {}", value);
        }
        #[template_callback]
        fn on_speaker_changed(&self, value: u32) {
            println!("Speaker changed: {}", value);
        }
        #[template_callback]
        fn on_mic_volume_changed(&self, value: f64) {
            println!("Mic volume: {}", value);
        }
        #[template_callback]
        fn on_speaker_volume_changed(&self, value: f64) {
            println!("Speaker volume: {}", value);
            //send message to client mod via channel in DataProvider
        }
    } //end #[gtk::template_callbacks]

    impl ObjectImpl for ServiceSettings {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("vm-control-action")
                        .param_types([
                            ControlAction::static_type(),
                            String::static_type(),
                            String::static_type(),
                        ])
                        .build(),
                    Signal::builder("vm-mic-changed")
                        .param_types([u32::static_type()])
                        .build(),
                    Signal::builder("vm-speaker-changed")
                        .param_types([u32::static_type()])
                        .build(),
                    Signal::builder("vm-mic-volume-changed")
                        .param_types([f64::static_type()])
                        .build(),
                    Signal::builder("vm-speaker-volume-changed")
                        .param_types([f64::static_type()])
                        .build(),
                ]
            })
        }
    }
    impl WidgetImpl for ServiceSettings {}
    impl BoxImpl for ServiceSettings {}
}

glib::wrapper! {
pub struct ServiceSettings(ObjectSubclass<imp::ServiceSettings>)
    @extends gtk::Widget, gtk::Box,
    @implements gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for ServiceSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceSettings {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn bind(&self, object: &ServiceGObject) {
        //unbind previous ones
        self.unbind();
        //make new
        let name = self.imp().name_slot_1.get();
        let is_vm = object.is_vm();
        let status = self.imp().status_label.get();
        let status_icon = self.imp().status_icon.get();
        let details = self.imp().details_label.get();
        let security_icon = self.imp().security_icon.get();
        let security_label = self.imp().security_label.get();
        let control_label = self.imp().control_label.get();
        let audio_settings_box = self.imp().audio_settings_box.get();
        let mut bindings = self.imp().bindings.borrow_mut();

        if is_vm {
            let full_service_name = self.imp().name_slot_2.get();

            let name_binding = object
                .bind_property("display-name", &name, "label")
                .sync_create()
                .build();
            bindings.push(name_binding);

            let full_service_name_binding = object
                .bind_property("name", &full_service_name, "label")
                .sync_create()
                .build();
            bindings.push(full_service_name_binding);
        } else {
            let name_binding = object
                .bind_property("name", &name, "label")
                .sync_create()
                .build();
            bindings.push(name_binding);
        };

        let status_binding = object
            .bind_property("status", &status, "label")
            .sync_create()
            .transform_to(move |_, value: &glib::Value| {
                let status = value.get::<u8>().unwrap_or(0);
                match status {
                    //make struct like for icon?
                    0 => Some(glib::Value::from("Running")),
                    1 => Some(glib::Value::from("Powered off")),
                    2 => Some(glib::Value::from("Paused")),
                    _ => Some(glib::Value::from("Powered off")),
                }
            })
            .build();
        bindings.push(status_binding);

        let status_icon_binding = object
            .bind_property("status", &status_icon, "resource")
            .sync_create()
            .transform_to(move |_, value: &glib::Value| {
                let status = value.get::<u8>().unwrap_or(0);
                match status {
                    //make struct like for icon?
                    0 => Some(glib::Value::from(
                        "/org/gnome/controlpanelgui/icons/ellipse_green.svg",
                    )),
                    1 => Some(glib::Value::from(
                        "/org/gnome/controlpanelgui/icons/ellipse_red.svg",
                    )),
                    2 => Some(glib::Value::from(
                        "/org/gnome/controlpanelgui/icons/ellipse_yellow.svg",
                    )),
                    _ => Some(glib::Value::from(
                        "/org/gnome/controlpanelgui/icons/ellipse_red.svg",
                    )),
                }
            })
            .build();
        bindings.push(status_icon_binding);

        let details_binding = object
            .bind_property("details", &details, "label")
            .sync_create()
            .build();
        bindings.push(details_binding);

        let security_icon_binding = object
            .bind_property("trust-level", &security_icon, "resource")
            .sync_create()
            .transform_to(move |_, value: &glib::Value| {
                let trust_level = value.get::<u8>().unwrap_or(0);
                Some(glib::Value::from(SecurityIcon::new(trust_level).0))
            })
            .build();
        bindings.push(security_icon_binding);

        let security_label_binding = object
            .bind_property("trust-level", &security_label, "label")
            .sync_create()
            .transform_to(move |_, value: &glib::Value| {
                let trust_level = value.get::<u8>().unwrap_or(0);
                match trust_level {
                    //make struct like for icon?
                    0 => Some(glib::Value::from("Secure!")),
                    1 => Some(glib::Value::from("Security warning!")),
                    2 => Some(glib::Value::from("Security alert!")),
                    _ => Some(glib::Value::from("Secure!")),
                }
            })
            .build();
        bindings.push(security_label_binding);

        //change label
        let controls_title_binding = object
            .bind_property("is-vm", &control_label, "label")
            .sync_create()
            .transform_to(move |_, value: &glib::Value| {
                let is_vm = value.get::<bool>().unwrap_or(false);
                if (is_vm) {
                    Some(glib::Value::from("VM Controls"))
                } else {
                    Some(glib::Value::from("Service Controls"))
                }
            })
            .build();
        bindings.push(controls_title_binding);

        //hide audio settings for services
        let audio_settings_visibilty_binding = object
            .bind_property("is-vm", &audio_settings_box, "visible")
            .sync_create()
            .build();
        bindings.push(audio_settings_visibilty_binding);
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
        //clean name slot 2
        self.imp().name_slot_2.set_text("");
    }
}
