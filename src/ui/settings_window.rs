use gtk4::prelude::*;
use gtk4::{
    Application, Button, Label, Box as GtkBox, Orientation, 
    HeaderBar, Window, CheckButton, ScrolledWindow, Separator, Spinner
};
use std::rc::Rc;

use crate::models::UserSettings;
use crate::services::fetch_waifu_tags_sync;

pub fn open_settings_window(app: &Application) {
    let settings_window = Rc::new(Window::builder()
        .application(app)
        .title("Settings - Waifu Generator")
        .default_width(500)
        .default_height(600)
        .resizable(true)
        .modal(true)
        .build());

    let settings_header = HeaderBar::new();
    settings_header.set_show_title_buttons(true);
    settings_window.set_titlebar(Some(&settings_header));

    let scrolled = ScrolledWindow::new();
    let main_box = GtkBox::new(Orientation::Vertical, 15);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);

    let title_label = Label::new(Some("Category Selection"));
    title_label.add_css_class("title-1");
    main_box.append(&title_label);

    let loading_spinner = Spinner::new();
    loading_spinner.set_size_request(32, 32);
    loading_spinner.start();
    
    let loading_label = Label::new(Some("Loading categories..."));
    loading_label.add_css_class("title-2");
    
    let loading_box = GtkBox::new(Orientation::Vertical, 10);
    loading_box.set_halign(gtk4::Align::Center);
    loading_box.set_valign(gtk4::Align::Center);
    loading_box.append(&loading_spinner);
    loading_box.append(&loading_label);
    
    main_box.append(&loading_box);

    let button_box = GtkBox::new(Orientation::Horizontal, 10);
    button_box.set_halign(gtk4::Align::End);

    let cancel_button = Button::with_label("Cancel");
    let window_clone1 = Rc::clone(&settings_window);
    cancel_button.connect_clicked(move |_| {
        window_clone1.close();
    });

    let save_button = Button::with_label("Save");
    save_button.add_css_class("suggested-action");
    save_button.set_sensitive(false);
    let window_clone2 = Rc::clone(&settings_window);
    let main_box_clone_for_save = main_box.clone();
    save_button.connect_clicked(move |_| {
        save_settings(&main_box_clone_for_save);
        println!("Settings saved!");
        window_clone2.close();
    });

    button_box.append(&cancel_button);
    button_box.append(&save_button);
    main_box.append(&button_box);

    scrolled.set_child(Some(&main_box));
    settings_window.set_child(Some(&scrolled));

    settings_window.present();

    match fetch_waifu_tags_sync() {
        Ok(tags) => {
            while let Some(child) = loading_box.first_child() {
                loading_box.remove(&child);
            }

            let config_path = UserSettings::default_config_path();
            let current_settings = UserSettings::load_from_file(&config_path).unwrap_or_default();

            let versatile_label = Label::new(Some("Versatile Categories"));
            versatile_label.add_css_class("title-2");
            main_box.insert_child_after(&versatile_label, Some(&main_box.first_child().unwrap()));

            let versatile_box = GtkBox::new(Orientation::Vertical, 5);
            
            for tag in &tags.versatile {
                let check_button = CheckButton::with_label(&capitalize_first(tag));
                if current_settings.selected_versatile.contains(tag) {
                    check_button.set_active(true);
                }
                versatile_box.append(&check_button);
            }

            main_box.insert_child_after(&versatile_box, Some(&versatile_label));

            let separator1 = Separator::new(Orientation::Horizontal);
            main_box.insert_child_after(&separator1, Some(&versatile_box));

            let nsfw_label = Label::new(Some("NSFW Categories"));
            nsfw_label.add_css_class("title-2");
            main_box.insert_child_after(&nsfw_label, Some(&separator1));

            let nsfw_box = GtkBox::new(Orientation::Vertical, 5);
            
            for tag in &tags.nsfw {
                let check_button = CheckButton::with_label(&capitalize_first(tag));
                if current_settings.selected_nsfw.contains(tag) {
                    check_button.set_active(true);
                }
                nsfw_box.append(&check_button);
            }

            main_box.insert_child_after(&nsfw_box, Some(&nsfw_label));

            let separator2 = Separator::new(Orientation::Horizontal);
            main_box.insert_child_after(&separator2, Some(&nsfw_box));

            save_button.set_sensitive(true);
        }
        Err(e) => {
            while let Some(child) = loading_box.first_child() {
                loading_box.remove(&child);
            }
            let error_label = Label::new(Some(&format!("Error: {}", e)));
            error_label.add_css_class("error");
            loading_box.append(&error_label);
        }
    }
}


fn save_settings(main_box: &GtkBox) {
    let mut selected_versatile = Vec::new();
    let mut selected_nsfw = Vec::new();
    let mut is_in_nsfw_section = false;

    let mut child = main_box.first_child();
    while let Some(widget) = child {
        if let Some(label) = widget.downcast_ref::<Label>() {
            let label_text = label.text();
            if label_text == "NSFW Categories" {
                is_in_nsfw_section = true;
            } else if label_text == "Versatile Categories" {
                is_in_nsfw_section = false;
            }
        } else if let Some(box_widget) = widget.downcast_ref::<GtkBox>() {
            let mut box_child = box_widget.first_child();
            while let Some(box_widget_child) = box_child {
                if let Some(check_button) = box_widget_child.downcast_ref::<CheckButton>() {
                    if check_button.is_active() {
                        let label = check_button.label().unwrap_or_default();
                        let tag = label.to_lowercase();
                        
                        println!("Checkbox found: '{}' (active) - Section: {}", tag, if is_in_nsfw_section { "NSFW" } else { "SFW" });
                        
                        if is_in_nsfw_section {
                            selected_nsfw.push(tag);
                        } else {
                            selected_versatile.push(tag);
                        }
                    }
                }
                box_child = box_widget_child.next_sibling();
            }
        }
        child = widget.next_sibling();
    }

    println!("Selected SFW tags: {:?}", selected_versatile);
    println!("Selected NSFW tags: {:?}", selected_nsfw);

    let settings = UserSettings {
        selected_versatile,
        selected_nsfw,
    };

    let config_path = UserSettings::default_config_path();
    
    if let Some(parent) = std::path::Path::new(&config_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    match settings.save_to_file(&config_path) {
        Ok(_) => {
            println!("Settings saved in: {}", config_path);
            println!("Settings saved!");
        },
        Err(e) => eprintln!("Error during save: {}", e),
    }
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        #[allow(non_snake_case)]
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
