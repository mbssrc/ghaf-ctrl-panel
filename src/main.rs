mod add_network_popup;
mod admin_settings_page;
mod application;
mod audio_settings;
mod confirm_display_settings_popup;
mod connection_config;
mod control_action;
mod data_gobject;
mod data_provider;
mod display_settings_page;
mod error_popup;
mod info_settings_page;
mod keyboard_settings_page;
mod language_region_notify_popup;
mod language_region_settings_page;
mod mouse_settings_page;
mod security_icon;
mod security_settings_page;
mod service_gobject;
mod service_row;
mod service_settings;
mod settings;
mod settings_action;
mod settings_gobject;
mod trust_level;
mod vm_row;
mod wifi_settings_page;
mod window;

use self::application::ControlPanelGuiApplication;
use self::window::ControlPanelGuiWindow;
use clap::Parser;

use gtk::prelude::*;
use gtk::{gio, glib};

const ADMIN_SERVICE_ADDR: &str = "192.168.100.10";
const ADMIN_SERVICE_PORT: u16 = 9001;

#[derive(Parser, Debug)]
#[command(name = "ctrl-panel")]
#[command(about = "Ghaf Control panel", long_about = None)]
struct Args {
    #[arg(long)]
    addr: Option<String>,
    #[arg(long)]
    port: Option<u16>,
}

fn main() /*-> glib::ExitCode*/
{
    //std::env::set_var("RUST_BACKTRACE", "full");

    // Parse the command-line arguments
    let args = Args::parse();

    let addr = if let Some(addr) = args.addr {
        addr
    } else {
        String::from(ADMIN_SERVICE_ADDR)
    };

    let port = if let Some(port) = args.port {
        port
    } else {
        ADMIN_SERVICE_PORT
    };

    // Load resources
    gio::resources_register_include!("control_panel_gui.gresource")
        .expect("Failed to register resources.");

    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.
    let app = ControlPanelGuiApplication::new(
        "org.gnome.controlpanelgui",
        &gio::ApplicationFlags::empty(),
        addr,
        port,
    );

    // Run the application. This function will block until the application
    // exits. Upon return, we have our exit code to return to the shell. (This
    // is the code you see when you do `echo $?` after running a command in a
    // terminal.
    //put empty array as args, cause we need our ones to be processed
    let empty: Vec<String> = vec![];
    app.run_with_args(&empty);
}
