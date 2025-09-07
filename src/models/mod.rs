use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WaifuTags {
    pub versatile: Vec<String>,
    pub nsfw: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WaifuImage {
    pub signature: String,
    pub extension: String,
    pub image_id: u32,
    pub favorites: u32,
    pub dominant_color: String,
    pub source: String,
    pub artist: String,
    pub artist_id: u32,
    pub name: String,
    pub patreon: Option<String>,
    pub pixiv: String,
    pub twitter: String,
    pub deviant_art: Option<String>,
    pub uploaded_at: String,
    pub liked_at: Option<String>,
    pub is_nsfw: bool,
    pub width: u32,
    pub height: u32,
    pub byte_size: u32,
    pub url: String,
    pub preview_url: String,
    pub tags: Vec<WaifuTag>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WaifuTag {
    pub tag_id: u32,
    pub name: String,
    pub description: String,
    pub is_nsfw: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WaifuImageResponse {
    pub images: Vec<WaifuImage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub selected_versatile: Vec<String>,
    pub selected_nsfw: Vec<String>,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            selected_versatile: vec!["waifu".to_string()],
            selected_nsfw: vec![],
        }
    }
}

impl UserSettings {
    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if std::path::Path::new(path).exists() {
            let json = std::fs::read_to_string(path)?;
            let settings: UserSettings = serde_json::from_str(&json)?;
            Ok(settings)
        } else {
            Ok(UserSettings::default())
        }
    }

    pub fn default_config_path() -> String {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.config/waifu-generator/settings.json", home)
    }
}

#[derive(Debug, Clone)]
pub struct ImageCache {
    pub images: Vec<String>,
    pub current_index: usize,
    pub max_size: usize,
}

impl ImageCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            images: Vec::new(),
            current_index: 0,
            max_size,
        }
    }
    
    pub fn add_image(&mut self, url: String) {
        if self.current_index < self.images.len() {
            self.images.truncate(self.current_index + 1);
        }
        
        self.images.push(url);
        
        if self.images.len() > self.max_size {
            self.images.remove(0);
        } else {
            self.current_index = self.images.len() - 1;
        }
    }
    
    pub fn get_current_image(&self) -> Option<&String> {
        self.images.get(self.current_index)
    }
    
    pub fn can_go_previous(&self) -> bool {
        self.current_index > 0
    }
    
    pub fn can_go_next(&self) -> bool {
        !self.images.is_empty() && self.current_index < self.images.len() - 1
    }
    
    
    pub fn go_previous(&mut self) -> Option<&String> {
        if self.can_go_previous() {
            self.current_index -= 1;
            self.get_current_image()
        } else {
            None
        }
    }
    
    pub fn go_next(&mut self) -> Option<&String> {
        if self.can_go_next() {
            self.current_index += 1;
            self.get_current_image()
        } else {
            None
        }
    }
    
}
