#![allow(dead_code)]
#[macro_use]
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Field{
    pub name:String,
    #[serde(rename = "type")]
    pub datatype:String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Schema{
    pub fields:Vec<Field>,
    #[serde(rename = "primaryKey")]
    pub primary_key:Vec<String>,
    pub pandas_version:String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoursRecord{
    pub index: isize,
    pub rowid: isize,
    pub name: String,
    pub year: i32,
    pub month: i32,
    pub start: String,
    pub end: String,
    pub hours: String 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoursDataFrame{
    pub schema:Schema,
    pub data: Vec<HoursRecord>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HoursData{
    pub dataframe:HoursDataFrame,
    pub names: Vec<String>,
}

impl HoursData{
    pub fn from_store<P: AsRef<Path>>(path:P)->Result<HoursData>{
        let file = File::open(path.as_ref().join("hours_dataframe.json"))?;
//        let mut buf_reader = BufReader::new(file);
        let dataframe: HoursDataFrame = serde_json::from_reader(
            file
        )?;
        
        let file = File::open(path.as_ref().join("hours_names.json"))?;
//        let mut buf_reader = BufReader::new(file);
        let names: Names = serde_json::from_reader(
            file
        )?;

        Ok(HoursData{
            dataframe: dataframe,
            names: names
        })
    }

    pub fn save<P: AsRef<Path>>(&self, path:P)->Result<()>{
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
}

pub type Names=Vec<String>;

