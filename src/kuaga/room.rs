use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::influxdb::{Influxdb, Record};

#[derive(Serialize, Debug, PartialEq)]
pub struct RoomStatus {
    pub humidity: f64,
    pub temperature: f64,
}

#[derive(Deserialize, Debug)]
pub struct Room {
    pub influxdb: Influxdb,
}

impl Room {
    /// クエリがめちゃくちゃれもくすハウス固有なのをどうにかしたい
    pub async fn status(&self) -> Result<RoomStatus> {
        let records = self.influxdb.query_record(r#"
            from(bucket: "room-status")
            |> range(start: -1m)
            |> filter(fn: (r) => r["device"] == "bme280")
            |> filter(fn: (r) => r["_field"] == "humidity" or r["_field"] == "temperature")
            |> filter(fn: (r) => r["_measurement"] == "climate")
            |> aggregateWindow(every: 1s, fn: mean, createEmpty: false)
            |> last()
            |> yield(name: "mean")"#.to_string()
        ).await?;

        let status = RoomStatus::try_from(records)?;

        Ok(status)
    }
}

impl TryFrom<Vec<Record>> for RoomStatus {
    type Error = anyhow::Error;
    fn try_from(records: Vec<Record>) -> Result<Self, Self::Error> {
        let mut humidity = Option::<f64>::None;
        let mut temperature = Option::<f64>::None;

        for record in records {
            if record.field == "humidity" {
                humidity = Some(record.value);
            } else if record.field == "temperature" {
                temperature = Some(record.value);
            }
        }
        if let Some((humidity, temperature)) = humidity.zip(temperature) {
            return Ok(RoomStatus {
                humidity,
                temperature,
            });
        }

        Err(anyhow!("not found"))
    }
}

#[test]
pub fn test_from_csv() {
    const CSV: &str = r#"
        ,result,table,_start,_stop,_time,_value,_field,_measurement,device
        ,mean,0,2022-01-01T13:29:30.226835818Z,2022-01-01T13:30:30.226835818Z,2022-01-01T13:29:51Z,36.61,humidity,climate,bme280
        ,mean,1,2022-01-01T13:29:30.226835818Z,2022-01-01T13:30:30.226835818Z,2022-01-01T13:29:51Z,8.59,temperature,climate,bme280
    "#;

    let records = Record::from_csv_text(CSV.into()).unwrap_or_default();

    let status = RoomStatus::try_from(records)
        .ok()
        .map(|status| (status.humidity.to_string(), status.temperature.to_string()));

    assert_eq!(
        status,
        Some(("36.61".to_string(), "8.59".to_string()))
    );
}
