use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Record {
    #[serde(rename = "_field")]
    pub field: String,

    #[serde(rename = "_value")]
    pub value: f64,
}

impl Record {
    pub fn from_csv_text(text: String) -> Result<Vec<Record>> {
        let mut reader = csv::Reader::from_reader(text.as_bytes());
        let records: Vec<Record> = reader
            .deserialize::<Record>()
            .into_iter()
            .flatten()
            .collect();
        Ok(records)
    }
}

#[derive(Deserialize, Debug)]
pub struct Influxdb {
    pub token: String,
    pub host: String,
    pub org_id: String,
}

impl Influxdb {
    pub async fn query(&self, text: String) -> Result<String> {
        use reqwest::*;
        let response = Client::new()
            .post(format!("{}/api/v2/query", self.host))
            .header("Accept", "application/csv")
            .header("Content-Type", "application/vnd.flux")
            .header("Authorization", format!("Token {}", self.token))
            .query(&[("orgID", &self.org_id)])
            .body(text)
            .send()
            .await?;
        let text = response.text().await?;
        Ok(text)
    }

    pub async fn query_record(&self, text: String) -> Result<Vec<Record>> {
        let text = self.query(text).await?;
        let records = Record::from_csv_text(text)?;
        Ok(records)
    }
}
