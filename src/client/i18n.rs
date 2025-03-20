// In a new file i18n.rs
use std::collections::HashMap;

pub struct I18n {
    translations: HashMap<String, HashMap<String, String>>,
    current_locale: String,
}

impl I18n {
    pub fn new(default_locale: &str) -> Self {
        Self {
            translations: HashMap::new(),
            current_locale: default_locale.to_string(),
        }
    }

    pub fn add_translation(&mut self, locale: &str, key: &str, value: &str) {
        self.translations
            .entry(locale.to_string())
            .or_insert_with(HashMap::new)
            .insert(key.to_string(), value.to_string());
    }

    pub fn set_locale(&mut self, locale: &str) {
        if self.translations.contains_key(locale) {
            self.current_locale = locale.to_string();
        }
    }

    pub fn get(&self, key: &str) -> String {
        // Try to get the translation for the current locale
        if let Some(locale_map) = self.translations.get(&self.current_locale) {
            if let Some(value) = locale_map.get(key) {
                return value.clone();
            }
        }

        // Fallback to English if available
        if self.current_locale != "en" {
            if let Some(en_map) = self.translations.get("en") {
                if let Some(value) = en_map.get(key) {
                    return value.clone();
                }
            }
        }

        // Return the key as fallback
        key.to_string()
    }

    pub fn load_translations_from_file(
        &mut self,
        file_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        let translations: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(&content)?;

        for (locale, translations) in translations {
            for (key, value) in translations {
                self.add_translation(&locale, &key, &value);
            }
        }

        Ok(())
    }
}
