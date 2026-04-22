// Initialize rust-i18n macro
rust_i18n::i18n!("locales", fallback = "en");

/// Extract language from Accept-Language header
pub fn extact_language_from_header(accept_language: Option<&str>) -> String {
    if let Some(header) = accept_language {
        // extract 'en' from 'en-US,en;q=0.9'
        if let Some(lang) = header.split(',').next() {
            if let Some(base_lang) = lang.split('-').next() {
                return base_lang.to_string();
            }
        }
    }
    "en".to_string()
}
