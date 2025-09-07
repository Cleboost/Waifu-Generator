use crate::models::{WaifuTags, UserSettings};

pub fn fetch_waifu_tags_sync() -> Result<WaifuTags, String> {
    Ok(WaifuTags {
        versatile: vec![
            "waifu".to_string(),
            "neko".to_string(),
            "shinobu".to_string(),
            "megumin".to_string(),
            "bully".to_string(),
            "cuddle".to_string(),
            "cry".to_string(),
            "hug".to_string(),
            "awoo".to_string(),
            "kiss".to_string(),
            "lick".to_string(),
            "pat".to_string(),
            "smug".to_string(),
            "bonk".to_string(),
            "yeet".to_string(),
            "blush".to_string(),
            "smile".to_string(),
            "wave".to_string(),
            "highfive".to_string(),
            "handhold".to_string(),
            "nom".to_string(),
            "bite".to_string(),
            "glomp".to_string(),
            "slap".to_string(),
            "kill".to_string(),
            "kick".to_string(),
            "happy".to_string(),
            "wink".to_string(),
            "poke".to_string(),
            "dance".to_string(),
            "cringe".to_string(),
        ],
        nsfw: vec![
            "waifu".to_string(),
            "neko".to_string(),
            "trap".to_string(),
            "blowjob".to_string(),
        ],
    })
}

pub async fn fetch_waifu_image_async(settings: &UserSettings) -> Result<String, String> {
    use rand::seq::SliceRandom;
    use rand::Rng;
    
    if settings.selected_versatile.is_empty() && settings.selected_nsfw.is_empty() {
        return Err("No category selected".to_string());
    }
    
    let mut rng = rand::thread_rng();
    let (selected_tag, is_nsfw) = if !settings.selected_versatile.is_empty() && !settings.selected_nsfw.is_empty() {
        if rng.gen_bool(0.5) {
            let tag = settings.selected_versatile.choose(&mut rng).unwrap();
            (tag.clone(), false)
        } else {
            let tag = settings.selected_nsfw.choose(&mut rng).unwrap();
            (tag.clone(), true)
        }
    } else if !settings.selected_versatile.is_empty() {
        let tag = settings.selected_versatile.choose(&mut rng).unwrap();
        (tag.clone(), false)
    } else {
        let tag = settings.selected_nsfw.choose(&mut rng).unwrap();
        (tag.clone(), true)
    };
    
    let url = if is_nsfw {
        format!("https://api.waifu.pics/nsfw/{}", selected_tag)
    } else {
        format!("https://api.waifu.pics/sfw/{}", selected_tag)
    };
    
    println!("Request URL: {}", url);
    println!("Randomly selected tag: {}", selected_tag);
    println!("Type: {}", if is_nsfw { "NSFW" } else { "SFW" });
    
    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(json) => {
                        if let Some(image_url) = json.get("url").and_then(|u| u.as_str()) {
                            println!("Image found: {}", image_url);
                            return Ok(image_url.to_string());
                        }
                        Err("No image URL found in response".to_string())
                    }
                    Err(e) => {
                        println!("JSON parsing error: {}", e);
                        Err(format!("JSON parsing error: {}", e))
                    }
                }
            } else {
                println!("HTTP error: {}", response.status());
                Err(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => {
            println!("Request error: {}", e);
            Err(format!("Request error: {}", e))
        }
    }
}