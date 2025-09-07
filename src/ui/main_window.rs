use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Button, Label, Box as GtkBox, Orientation,
    HeaderBar, Image, ScrolledWindow, Picture, FileChooserDialog, ResponseType, DrawingArea
};
use gtk4::gio;
use gtk4::cairo;

use crate::ui::settings_window::open_settings_window;
use crate::models::{UserSettings, ImageCache};
use crate::services::fetch_waifu_image_async;
use std::rc::Rc;
use std::cell::RefCell;

pub fn build_main_window(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Waifu Generator")
        .default_width(400)
        .default_height(300)
        .resizable(true)
        .decorated(true)
        .build();

    let header_bar = HeaderBar::new();
    header_bar.set_show_title_buttons(true);
    
    let settings_button = Button::new();
    let settings_icon = Image::from_icon_name("preferences-system-symbolic");
    settings_button.set_child(Some(&settings_icon));
    settings_button.set_tooltip_text(Some("Settings"));
    settings_button.add_css_class("flat");
    
    let app_clone = app.clone();
    settings_button.connect_clicked(move |_| {
        println!("Opening settings...");
        open_settings_window(&app_clone);
    });
    
    let download_button = Button::new();
    let download_icon = Image::from_icon_name("document-save-symbolic");
    download_button.set_child(Some(&download_icon));
    download_button.set_tooltip_text(Some("Download image"));
    download_button.add_css_class("flat");
    
    let loading_spinner = create_circular_loader();
    loading_spinner.set_size_request(24, 24);
    loading_spinner.set_visible(false);
    
    header_bar.pack_start(&settings_button);
    header_bar.pack_start(&download_button);
    header_bar.pack_start(&loading_spinner);
    
    window.set_titlebar(Some(&header_bar));

    let scrolled = ScrolledWindow::new();
    let main_box = GtkBox::new(Orientation::Vertical, 15);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);

    let image_container = GtkBox::new(Orientation::Vertical, 10);
    image_container.set_halign(gtk4::Align::Center);
    
    let navigation_box = GtkBox::new(Orientation::Horizontal, 10);
    navigation_box.set_halign(gtk4::Align::Center);
    
    let prev_button = Button::new();
    let prev_icon = Image::from_icon_name("go-previous-symbolic");
    prev_button.set_child(Some(&prev_icon));
    prev_button.set_tooltip_text(Some("Previous image"));
    prev_button.add_css_class("flat");
    prev_button.set_sensitive(false);
    prev_button.set_size_request(48, 48);
    prev_button.set_hexpand(false);
    prev_button.set_vexpand(false);
    
    let image_display_container = GtkBox::new(Orientation::Vertical, 10);
    image_display_container.set_halign(gtk4::Align::Center);
    image_display_container.set_hexpand(true);
    
    let next_button = Button::new();
    let next_icon = Image::from_icon_name("go-next-symbolic");
    next_button.set_child(Some(&next_icon));
    next_button.set_tooltip_text(Some("Generate new image"));
    next_button.add_css_class("flat");
    next_button.set_sensitive(true);
    next_button.set_size_request(48, 48);
    next_button.set_hexpand(false);
    next_button.set_vexpand(false);
    
    navigation_box.append(&prev_button);
    navigation_box.append(&image_display_container);
    navigation_box.append(&next_button);

    image_container.append(&navigation_box);
    main_box.append(&image_container);

    scrolled.set_child(Some(&main_box));
    window.set_child(Some(&scrolled));

    let image_cache = Rc::new(RefCell::new(ImageCache::new(20)));

    let image_cache_clone = Rc::clone(&image_cache);
    download_button.connect_clicked(move |_| {
        download_current_image(&image_cache_clone);
    });

    let image_display_container_clone = image_display_container.clone();
    let image_cache_clone = Rc::clone(&image_cache);
    let prev_button_clone = prev_button.clone();
    let next_button_clone = next_button.clone();
    let loading_spinner_clone = loading_spinner.clone();
    prev_button.connect_clicked(move |_| {
        navigate_previous(&image_display_container_clone, &image_cache_clone, &prev_button_clone, &next_button_clone, &loading_spinner_clone);
    });

    let image_display_container_clone = image_display_container.clone();
    let image_cache_clone = Rc::clone(&image_cache);
    let prev_button_clone = prev_button.clone();
    let next_button_clone = next_button.clone();
    let loading_spinner_clone = loading_spinner.clone();
    next_button.connect_clicked(move |_| {
        navigate_or_generate_next(&image_display_container_clone, &image_cache_clone, &prev_button_clone, &next_button_clone, &loading_spinner_clone);
    });

    window.present();
    
    generate_new_image(&image_display_container, &image_cache, &loading_spinner);
    update_navigation_buttons(&image_cache, &prev_button, &next_button);
}

fn generate_new_image(image_container: &GtkBox, image_cache: &Rc<RefCell<ImageCache>>, loading_spinner: &DrawingArea) {
    println!("Generating new image...");
    
    loading_spinner.set_visible(true);
    
    let config_path = UserSettings::default_config_path();
    let settings = UserSettings::load_from_file(&config_path).unwrap_or_default();
    
    let image_container_clone = image_container.clone();
    let image_cache_clone = Rc::clone(image_cache);
    let loading_spinner_clone = loading_spinner.clone();
    
    glib::spawn_future_local(async move {
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        match rt.block_on(fetch_waifu_image_async(&settings)) {
            Ok(image_url) => {
                println!("Image generated: {}", image_url);
                
                loading_spinner_clone.set_visible(false);
                
                image_cache_clone.borrow_mut().add_image(image_url.clone());
            
                display_image_with_loader(&image_container_clone, &image_url, &loading_spinner_clone);
            }
            Err(e) => {
                println!("Error during generation: {}", e);
                
                loading_spinner_clone.set_visible(false);
                
                while let Some(child) = image_container_clone.first_child() {
                    image_container_clone.remove(&child);
                }
                
                let error_label = Label::new(Some(&format!("Error: {}", e)));
                error_label.add_css_class("title-2");
                error_label.add_css_class("error");
                image_container_clone.append(&error_label);
            }
        }
    });
}

fn display_image_with_loader(image_container: &GtkBox, image_url: &str, loading_spinner: &DrawingArea) {
    while let Some(child) = image_container.first_child() {
        image_container.remove(&child);
    }
    
    loading_spinner.set_visible(true);
    
    let picture = Picture::new();
    picture.set_size_request(400, 400);
    image_container.append(&picture);
    
    let picture_clone = picture.clone();
    let image_url_clone = image_url.to_string();
    let loading_spinner_clone = loading_spinner.clone();
    
    glib::spawn_future_local(async move {
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        if let Ok(response) = rt.block_on(reqwest::get(&image_url_clone)) {
            if let Ok(bytes) = rt.block_on(response.bytes()) {
                let temp_dir = std::env::temp_dir();
                let temp_file = temp_dir.join("waifu_temp.png");
                
                if let Ok(_) = std::fs::write(&temp_file, bytes) {
                    let file = gio::File::for_path(&temp_file);
                    picture_clone.set_file(Some(&file));
                    
                    loading_spinner_clone.set_visible(false);
                    
                    let temp_file_clone = temp_file.clone();
                    glib::timeout_future_seconds(5).await;
                    let _ = std::fs::remove_file(&temp_file_clone);
                }
            }
        }
    });
}

fn navigate_previous(
    image_container: &GtkBox,
    image_cache: &Rc<RefCell<ImageCache>>,
    prev_button: &Button,
    next_button: &Button,
    loading_spinner: &DrawingArea
) {
    let image_url = {
        let mut cache = image_cache.borrow_mut();
        cache.go_previous().map(|url| url.clone())
    };
    
    if let Some(url) = image_url {
        display_image_with_loader(image_container, &url, loading_spinner);
        update_navigation_buttons(image_cache, prev_button, next_button);
    }
}

fn navigate_or_generate_next(
    image_container: &GtkBox, 
    image_cache: &Rc<RefCell<ImageCache>>, 
    prev_button: &Button, 
    next_button: &Button,
    loading_spinner: &DrawingArea
) {
    let image_url = {
        let mut cache = image_cache.borrow_mut();
        cache.go_next().map(|url| url.clone())
    };
    
    if let Some(url) = image_url {
        display_image_with_loader(image_container, &url, loading_spinner);
        update_navigation_buttons(image_cache, prev_button, next_button);
    } else {
        generate_new_image(image_container, image_cache, loading_spinner);
        update_navigation_buttons(image_cache, prev_button, next_button);
    }
}

fn update_navigation_buttons(image_cache: &Rc<RefCell<ImageCache>>, prev_button: &Button, next_button: &Button) {
    let can_go_prev = {
        let cache = image_cache.borrow();
        cache.can_go_previous()
    };
    
    prev_button.set_sensitive(can_go_prev);
    next_button.set_sensitive(true);
}

fn download_current_image(image_cache: &Rc<RefCell<ImageCache>>) {
    let current_image_url = {
        let cache = image_cache.borrow();
        cache.get_current_image().map(|url| url.clone())
    };
    
    if let Some(image_url) = current_image_url {
        let dialog = FileChooserDialog::new(
            Some("Save image"),
            None::<&gtk4::Window>,
            gtk4::FileChooserAction::Save,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Save", ResponseType::Accept),
            ]
        );
        
        let default_name = format!("waifu_{}.png", 
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        
        dialog.set_current_name(&default_name);
        
        let image_url_clone = image_url.clone();
        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        if let Ok(response) = rt.block_on(reqwest::get(&image_url_clone)) {
                            if let Ok(bytes) = rt.block_on(response.bytes()) {
                                if let Err(e) = std::fs::write(&path, bytes) {
                                    eprintln!("Error during save: {}", e);
                                } else {
                                    println!("Image saved: {:?}", path);
                                }
                            }
                        }
                    }
                }
            }
            dialog.close();
        });
        
        dialog.present();
    } else {
        println!("No image to download");
    }
}

fn create_circular_loader() -> DrawingArea {
    let drawing_area = DrawingArea::new();
    let mut angle = 0.0;
    
    drawing_area.set_draw_func(move |_, cr, width, height| {
        let center_x = width as f64 / 2.0;
        let center_y = height as f64 / 2.0;
        let radius = (width.min(height) as f64 / 2.0) - 2.0;
        
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
        cr.paint().unwrap();
        
        cr.set_source_rgba(0.7, 0.7, 0.7, 0.3);
        cr.set_line_width(2.0);
        cr.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI);
        cr.stroke().unwrap();
        
        cr.set_source_rgba(0.0, 0.5, 1.0, 0.8);
        cr.set_line_width(3.0);
        cr.set_line_cap(cairo::LineCap::Round);
        
        let start_angle = angle * std::f64::consts::PI / 180.0;
        let end_angle = start_angle + 3.0 * std::f64::consts::PI / 2.0;
        
        cr.arc(center_x, center_y, radius, start_angle, end_angle);
        cr.stroke().unwrap();
    });
    
    let drawing_area_clone = drawing_area.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        angle += 10.0;
        if angle >= 360.0 {
            angle = 0.0;
        }
        drawing_area_clone.queue_draw();
        glib::ControlFlow::Continue
    });
    
    drawing_area
}
