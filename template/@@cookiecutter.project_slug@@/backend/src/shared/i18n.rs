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

/// Translate registration sandbox error
pub fn translate_sandbox_error(domain: &str) -> String {
    rust_i18n::t!("registration_sandbox_error", domain = domain).to_string()
}
