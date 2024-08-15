use chrono::{DateTime, Local};

pub fn calc_timing(published_at: Option<&String>) -> &'static str
{
    let today = Local::now().naive_local().date();
    let yesterday = today - chrono::Duration::days(1);

    match published_at {
        Some(pub_date_str) => match DateTime::parse_from_rfc3339(pub_date_str) {
            Ok(pub_datetime) => {
                let pub_day = pub_datetime.naive_local().date();
                if pub_day == today || pub_day == yesterday {
                    "Música (nova)"
                } else {
                    "Pack Aleatório"
                }
            }
            Err(_) => "",
        },
        None => "",
    }
}

pub fn format_publish_date(published_at: Option<&str>) -> String
{
    match published_at {
        Some(pub_date_str) => DateTime::parse_from_rfc3339(pub_date_str).map_or_else(
            |_| "".to_string(),
            |pub_datetime| {
                let pub_day = pub_datetime.naive_local().date();
                format!("Publicado em: {}", pub_day.format("%d/%m/%Y"))
            },
        ),
        None => "".to_string(),
    }
}
