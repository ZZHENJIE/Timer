use crate::economy::DataItem;
use chrono::{DateTime, Datelike, Duration, NaiveDateTime, TimeZone};
use chrono_tz::America::New_York;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub calendar_id: i64,
    pub ticker: String,
    pub event: String,
    pub category: String,
    pub date: String,
    pub reference: Option<String>,
    pub reference_date: Option<String>,
    pub actual: Option<String>,
    pub previous: Option<String>,
    pub forecast: Option<String>,
    pub teforecast: Option<String>,
    pub importance: u8,
    pub is_higher_positive: i8,
    pub has_no_detail: bool,
    pub alert: Option<serde_json::Value>,
    pub all_day: bool,
    pub non_emptiness_score: i8,
}

pub async fn fetch(client: &reqwest::Client, timestamp: i64) -> anyhow::Result<Vec<DataItem>> {
    let (begin, end) = get_week(timestamp);
    let url = format!(
        "https://finviz.com/api/calendar/economic?dateFrom={}&dateTo={}",
        begin, end
    );
    let response = client.get(url).send().await?;
    let items: Vec<Item> = response.json().await?;
    let mut result: Vec<DataItem> = Vec::new();

    for item in items {
        let date = str_to_timestamp(&item.date)?;
        result.push(DataItem {
            date_timestamp: date,
            event: item.event,
            importance: item.importance,
            previous: item.previous,
            actual: item.actual,
            forecast: item.forecast,
        });
    }

    Ok(result)
}

fn str_to_timestamp(date: &str) -> anyhow::Result<i64> {
    let naive = NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S")?;
    let eastern_time = New_York
        .from_local_datetime(&naive)
        .single()
        .expect("Unable to convert to Eastern Time (possibly during daylight saving time switch).");
    Ok(eastern_time.timestamp())
}

fn get_week(timestamp: i64) -> (String, String) {
    let datetime = DateTime::from_timestamp(timestamp, 0).expect("Invalid timestamp");
    let weekday = datetime.weekday().num_days_from_monday();
    let monday = datetime.date_naive() - Duration::days(weekday as i64);
    let friday = monday + Duration::days(4);

    (
        format!("{}", monday.format("%Y-%m-%d")),
        format!("{}", friday.format("%Y-%m-%d")),
    )
}
