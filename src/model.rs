#![allow(dead_code)]
use anyhow::{anyhow, Result};
use chrono::{Datelike, Local, Timelike};
use serde::{Deserialize, Serialize};

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::iter::Sum;
use std::ops::{Add, Sub};
use std::path::{Path, PathBuf};

const MAX_SECONDS: i64 = 60*60*15;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    #[serde(rename = "type")]
    pub datatype: String,
}
impl Field {
    pub fn new(name: &str, datatype: &str) -> Self {
        Self {
            name: name.to_string(),
            datatype: datatype.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Period {
    pub year: i32,
    pub month: u32,
}

impl Display for Period {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}/{:02}", self.year, self.month)
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Hours(f64);

impl Display for Hours {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let h = self.0.floor();
        let m = ((self.0 - h) * 60.0).trunc();
        write!(f, "{:02}:{:02}", h, m)
    }
}

impl Hours {
    pub fn new(h: f64) -> Self {
        Self(h)
    }
}

impl From<f64> for Hours {
    fn from(h: f64) -> Self {
        Self(h)
    }
}

impl From<Hours> for f64 {
    fn from(h: Hours) -> Self {
        h.0
    }
}

impl From<Hours> for String {
    fn from(h: Hours) -> Self {
        format!("{}", h)
    }
}

impl Add for Hours {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Hours {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Sum for Hours {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |a, b| a + b)
    }
}

impl Period {
    pub fn new(year: i32, month: u32) -> Period {
        Period { year, month }
    }
    pub fn current() -> Period {
        let now = Local::now();
        Period::new(now.year(), now.month())
    }
    pub fn next(&self) -> Period {
        if self.month == 12 {
            Period::new(self.year + 1, 1)
        } else {
            Period::new(self.year, self.month + 1)
        }
    }
    pub fn previous(&self) -> Period {
        if self.month == 1 {
            Period::new(self.year - 1, 12)
        } else {
            Period::new(self.year, self.month - 1)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Schema {
    pub fields: Vec<Field>,
    #[serde(rename = "primaryKey")]
    pub primary_key: Vec<String>,
    pub pandas_version: String,
}

impl Default for Schema {
    fn default() -> Self {
        Self {
            fields: vec![
                Field::new("index", "integer"),
                Field::new("rowid", "integer"),
                Field::new("name", "string"),
                Field::new("year", "integer"),
                Field::new("month", "integer"),
                Field::new("start", "string"),
                Field::new("end", "string"),
                Field::new("hours", "string"),
            ],
            primary_key: vec!["index".to_string()],
            pandas_version: "0.20.0".to_string(),
        }
    }
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

    pub fn period(&self) -> Period {
        Period::new(self.year, self.month)
    }
    pub fn start_dt(&self) -> Result<chrono::NaiveDateTime> {
        let start = chrono::NaiveDateTime::parse_from_str(&self.start, "%Y-%m-%d %H:%M:%S")?;
        Ok(start)
    }

    pub fn end_dt(&self) -> Result<chrono::NaiveDateTime> {
        let end = chrono::NaiveDateTime::parse_from_str(&self.end, "%Y-%m-%d %H:%M:%S")?;
        Ok(end)
    }

    pub fn calculate_hours(&self) -> Result<Hours> {
        let duration = self.end_dt()?.signed_duration_since(self.start_dt()?);
        Ok((duration.num_seconds() as f64 / 3600.0).into())
    }
    pub fn original_hours(&self) -> String {
        if let Ok(h) = self.calculate_hours() {
            format!("{}", h)
        } else {
            "-".to_string()
        }
    }
    pub fn hours(&self) -> String {
        if let Ok(h) = self.hours.parse::<f64>() {
            format!("{}", Hours(h))
        } else {
            if let Ok(h) = self.calculate_hours() {
                if h.0 > 0.0 && h.0*60.0*60.0<MAX_SECONDS as f64{
                    format!("{}", h)
                } else {
                    "unfinished".to_string()
                }
            } else {
                "?".to_string()
            }
        }
    }

    pub fn hours_worked(&self) -> Hours {
        if let Ok(h) = self.hours.parse::<f64>() {
            h.into()
        } else {
            self.calculate_hours().unwrap_or(Hours::default())
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
    pub fn finished(&self) -> bool {
        if let Ok(start) = self.start_dt() {
            let end = self
                .end_dt()
                .unwrap_or_else(|_e| Local::now().naive_local());
            let duration = end.signed_duration_since(start);

            duration.num_seconds() > 0 && duration.num_seconds() < MAX_SECONDS
        } else {
            false
        }
    }
    pub fn worked(&self) -> String {
        if let Ok(start) = self.start_dt() {
            let end = self
                .end_dt()
                .unwrap_or_else(|_e| Local::now().naive_local());
            let duration = end.signed_duration_since(start);
            if duration.num_seconds() > 0 && duration.num_seconds() < MAX_SECONDS {
                format!(
                    "{:02}:{:02}",
                    duration.num_hours() % 24,
                    duration.num_minutes() % 60
                )
            } else {
                "Not finished".to_string()
            }
        } else {
            "Not started".to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HoursDataFrame {
    pub schema: Schema,
    pub data: Vec<HoursRecord>,
}

impl HoursDataFrame {
    pub fn new() -> HoursDataFrame {
        HoursDataFrame {
            schema: Schema::default(),
            data: Vec::new(),
        }
    }
    pub fn for_period(&self, name: &str, period: &Period) -> HoursDataFrame {
        let mut df = HoursDataFrame::new();
        df.schema = self.schema.clone();
        df.data = self
            .data
            .iter()
            .filter(|r| r.period() == *period && r.name == name)
            .cloned()
            .collect();
        df
    }
    pub fn hours_for_period(&self, name: &str, period: &Period) -> Hours {
        self.data
            .iter()
            .filter(|r| r.period() == *period && r.name == name)
            .map(|r| r.hours_worked())
            .sum()
    }
    pub fn status_for_period(&self, name: &str, period: &Period) -> String {
        self.data
            .iter()
            .filter(|r| r.period() == *period && r.name == name)
            .last()
            .map(|r| r.worked())
            .unwrap_or(" - ".to_string())
    }
    pub fn first_period(&self) -> Period {
        let mut min = Period::new(9999, 99);
        for r in &self.data {
            if r.period() < min {
                min = r.period();
            }
        }
        min
    }
    pub fn last_period(&self) -> Period {
        let mut max = Period::new(0, 0);
        for r in &self.data {
            if r.period() > max {
                max = r.period();
            }
        }
        max
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct HoursData {
    pub dataframe: HoursDataFrame,
    pub names: Vec<String>,
}

impl HoursData {
    pub fn folder() -> PathBuf {
        dirs::home_dir().unwrap().join("Hours")
    }
    pub fn create_if_not_exists() -> Result<()> {
        let path = Self::folder();
        if !path.is_dir() {
            std::fs::create_dir_all(&path)?;
        }
        let dataframe_path = (&path).join("hours_dataframe.json");
        if !dataframe_path.exists() {
            let hours_data = HoursData::default();
            hours_data.save_to(&path)?;
        }
        Ok(())
    }
    pub fn load() -> Result<HoursData> {
        Self::create_if_not_exists()?;
        HoursData::from_store(Self::folder())
    }
    pub fn from_store<P: AsRef<Path>>(path: P) -> Result<HoursData> {
        println!("Load from {:?}", path.as_ref());

        let file = File::open(path.as_ref().join("hours_dataframe.json"))?;
        //        let mut buf_reader = BufReader::new(file);
        let dataframe: HoursDataFrame = serde_json::from_reader(file)?;

        let file = File::open(path.as_ref().join("hours_names.json"))?;
        //        let mut buf_reader = BufReader::new(file);
        let names: Names = serde_json::from_reader(file)?;

        Ok(HoursData { dataframe, names })
    }

    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<()> {
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
    pub fn save(&self) -> Result<()> {
        let path = Self::folder();
        println!("Saving to {:?}", path);
        self.save_to(path)
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
        self.save()?;
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
        /*
        let start = chrono::NaiveDateTime::parse_from_str(&record.start, "%Y-%m-%d %H:%M:%S")?;
        let end = chrono::NaiveDateTime::parse_from_str(&record.end, "%Y-%m-%d %H:%M:%S")?;
        let duration = end.signed_duration_since(start);
        let hours = format!("{}", duration.num_seconds() as f64 / 3600.0);
        record.hours = hours;
        */
        record.hours = "".to_string();
        self.save()?;
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
