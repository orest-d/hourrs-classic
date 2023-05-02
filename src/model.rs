#![allow(dead_code)]
use anyhow::{anyhow, Result};
use chrono::{Datelike, Local, Timelike};
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub datatype: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    pub fields: Vec<Field>,
    #[serde(rename = "primaryKey")]
    pub primary_key: Vec<String>,
    pub pandas_version: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct HoursRecord {
    pub index: isize,
    pub rowid: isize,
    pub name: String,
    pub year: i32,
    pub month: u32,
    pub start: String,
    pub end: String,
    pub hours: String,
}

impl HoursRecord {
    pub fn new(
        index: isize,
        rowid: isize,
        name: String,
        year: i32,
        month: u32,
        start: String,
        end: String,
        hours: String,
    ) -> Self {
        Self {
            index,
            rowid,
            name,
            year,
            month,
            start,
            end,
            hours,
        }
    }

    pub fn start_dt(&self) -> Result<chrono::NaiveDateTime> {
        let start = chrono::NaiveDateTime::parse_from_str(&self.start, "%Y-%m-%d %H:%M:%S")?;
        Ok(start)
    }

    pub fn end_dt(&self) -> Result<chrono::NaiveDateTime> {
        let end = chrono::NaiveDateTime::parse_from_str(&self.end, "%Y-%m-%d %H:%M:%S")?;
        Ok(end)
    }

    pub fn calculate_hours(&self) -> Result<f64> {
        let duration = self.end_dt()?.signed_duration_since(self.start_dt()?);
        Ok(duration.num_seconds() as f64 / 3600.0)
    }
    pub fn original_hours(&self) -> String {
        if let Ok(h) = self.calculate_hours() {
            format!("{:.2}", h)
        } else {
            "-".to_string()
        }
    }
    pub fn hours(&self) -> String {
        if let Ok(h) = self.hours.parse::<f64>() {
            format!("{:.2}", h)
        } else {
            self.original_hours()
        }
    }

    pub fn date(&self) -> String {
        if let Ok(d) = self.start_dt() {
            format!("{:04}/{:02}/{:02}", d.year(), d.month(), d.day())
        } else {
            "?".to_string()
        }
    }
    pub fn start_time(&self) -> String {
        if let Ok(d) = self.start_dt() {
            format!("{:02}:{:02}", d.hour(), d.minute())
        } else {
            "".to_string()
        }
    }
    pub fn end_time(&self) -> String {
        if let Ok(d) = self.end_dt() {
            format!("{:02}:{:02}", d.hour(), d.minute())
        } else {
            "".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoursDataFrame {
    pub schema: Schema,
    pub data: Vec<HoursRecord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoursData {
    pub dataframe: HoursDataFrame,
    pub names: Vec<String>,
}

impl HoursData {
    pub fn from_store<P: AsRef<Path>>(path: P) -> Result<HoursData> {
        let file = File::open(path.as_ref().join("hours_dataframe.json"))?;
        //        let mut buf_reader = BufReader::new(file);
        let dataframe: HoursDataFrame = serde_json::from_reader(file)?;

        let file = File::open(path.as_ref().join("hours_names.json"))?;
        //        let mut buf_reader = BufReader::new(file);
        let names: Names = serde_json::from_reader(file)?;

        Ok(HoursData { dataframe, names })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        std::fs::write(
            path.as_ref().join("hours_dataframe.json"),
            serde_json::to_string_pretty(&self.dataframe).unwrap(),
        )?;
        std::fs::write(
            path.as_ref().join("hours_names.json"),
            serde_json::to_string_pretty(&self.names).unwrap(),
        )?;
        Ok(())
    }

    pub fn start(&mut self, name: &str) -> Result<()> {
        let now = Local::now();
        let year = now.year();
        let month = now.month();
        let day = now.day();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();
        let start = format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            year, month, day, hour, minute, second
        );
        let end = "".to_string();
        let hours = "".to_string();
        let index = self.dataframe.data.len() as isize;
        let rowid = index;
        let record = HoursRecord::new(
            index,
            rowid,
            name.to_string(),
            year,
            month,
            start,
            end,
            hours,
        );
        self.dataframe.data.push(record);
        Ok(())
    }

    pub fn end(&mut self, name: &str) -> Result<()> {
        let now = Local::now();
        let year = now.year();
        let month = now.month();
        let day = now.day();
        let hour = now.hour();
        let minute = now.minute();
        let second = now.second();
        let end = format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            year, month, day, hour, minute, second
        );
        let _index = self.dataframe.data.len() as isize;
        let mut rowid = -1;
        for (i, record) in self.dataframe.data.iter().enumerate() {
            if record.name == name && record.end == "" {
                rowid = i as isize;
            }
        }
        if rowid == -1 {
            return Err(anyhow!("No start record found for {}", name));
        }
        let mut record = self.dataframe.data.get_mut(rowid as usize).unwrap();
        record.end = end;
        let start = chrono::NaiveDateTime::parse_from_str(&record.start, "%Y-%m-%d %H:%M:%S")?;
        let end = chrono::NaiveDateTime::parse_from_str(&record.end, "%Y-%m-%d %H:%M:%S")?;
        let duration = end.signed_duration_since(start);
        let hours = format!("{}", duration.num_seconds() as f64 / 3600.0);
        record.hours = hours;
        Ok(())
    }

    pub fn is_started(&self, name: &str) -> bool {
        let mut started = false;
        for record in self.dataframe.data.iter() {
            if record.name == name {
                started = record.end == "";
            }
        }
        started
    }
}

pub type Names = Vec<String>;
