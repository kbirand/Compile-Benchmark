use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::{DateTime, Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use std::collections::HashMap;
use url::Url;

pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn generate_slug(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"
    ).unwrap();
    email_regex.is_match(email)
}

pub fn is_valid_url(url_str: &str) -> bool {
    Url::parse(url_str).is_ok()
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

pub fn encode_base64(data: &[u8]) -> String {
    BASE64.encode(data)
}

pub fn decode_base64(encoded: &str) -> Result<Vec<u8>, base64::DecodeError> {
    BASE64.decode(encoded)
}

pub fn format_currency(amount: f64, currency: &str) -> String {
    match currency.to_uppercase().as_str() {
        "USD" => format!("${:.2}", amount),
        "EUR" => format!("€{:.2}", amount),
        "GBP" => format!("£{:.2}", amount),
        "JPY" => format!("¥{:.0}", amount),
        _ => format!("{:.2} {}", amount, currency),
    }
}

pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

pub fn format_duration(seconds: i64) -> String {
    let duration = Duration::seconds(seconds);
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    let secs = duration.num_seconds() % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

pub fn time_ago(datetime: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(datetime);

    if duration.num_days() > 365 {
        let years = duration.num_days() / 365;
        format!("{} year{} ago", years, if years == 1 { "" } else { "s" })
    } else if duration.num_days() > 30 {
        let months = duration.num_days() / 30;
        format!("{} month{} ago", months, if months == 1 { "" } else { "s" })
    } else if duration.num_days() > 0 {
        let days = duration.num_days();
        format!("{} day{} ago", days, if days == 1 { "" } else { "s" })
    } else if duration.num_hours() > 0 {
        let hours = duration.num_hours();
        format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
    } else if duration.num_minutes() > 0 {
        let minutes = duration.num_minutes();
        format!("{} minute{} ago", minutes, if minutes == 1 { "" } else { "s" })
    } else {
        "just now".to_string()
    }
}

pub fn parse_query_string(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next().unwrap_or("");
            Some((
                urlencoding::decode(key).ok()?.into_owned(),
                urlencoding::decode(value).ok()?.into_owned(),
            ))
        })
        .collect()
}

pub fn sanitize_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

pub fn extract_domain(url_str: &str) -> Option<String> {
    Url::parse(url_str)
        .ok()
        .and_then(|url| url.host_str().map(|s| s.to_string()))
}

pub fn mask_email(email: &str) -> String {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return email.to_string();
    }

    let local = parts[0];
    let domain = parts[1];

    let masked_local = if local.len() <= 2 {
        "*".repeat(local.len())
    } else {
        format!(
            "{}{}{}",
            &local[..1],
            "*".repeat(local.len() - 2),
            &local[local.len() - 1..]
        )
    };

    format!("{}@{}", masked_local, domain)
}

pub fn mask_credit_card(card_number: &str) -> String {
    let digits: String = card_number.chars().filter(|c| c.is_ascii_digit()).collect();
    
    if digits.len() < 4 {
        return "*".repeat(digits.len());
    }

    format!("****-****-****-{}", &digits[digits.len() - 4..])
}

pub fn calculate_percentage(value: f64, total: f64) -> f64 {
    if total == 0.0 {
        0.0
    } else {
        (value / total) * 100.0
    }
}

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Hello World!"), "hello-world");
        assert_eq!(generate_slug("  Multiple   Spaces  "), "multiple-spaces");
        assert_eq!(generate_slug("Special@#$Characters"), "special-characters");
    }

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name@domain.co.uk"));
        assert!(!is_valid_email("invalid-email"));
        assert!(!is_valid_email("@nodomain.com"));
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(500), "500 bytes");
        assert_eq!(format_file_size(1024), "1.00 KB");
        assert_eq!(format_file_size(1048576), "1.00 MB");
        assert_eq!(format_file_size(1073741824), "1.00 GB");
    }

    #[test]
    fn test_mask_email() {
        assert_eq!(mask_email("test@example.com"), "t**t@example.com");
        assert_eq!(mask_email("ab@example.com"), "**@example.com");
    }

    #[test]
    fn test_mask_credit_card() {
        assert_eq!(mask_credit_card("4111111111111111"), "****-****-****-1111");
        assert_eq!(mask_credit_card("4111-1111-1111-1111"), "****-****-****-1111");
    }
}
