extern crate csv;

use csv::*;
use std::io;
use csv::Reader;
use std::collections::HashSet;
use std::env;

fn unique_by_columns(columns: Vec<String>) -> () {
    let mut writer = csv::Writer::from_writer(io::stdout());
    let mut reader = csv::Reader::from_reader(io::stdin());

    let id_indices: Vec<usize> = reader.headers().expect("").iter().enumerate().filter(|(i, header)| {
        columns.contains(&header.to_string())
    }).map(|(i, _)| {
        i
    }).collect();

    let mut seen_ids = HashSet::new();

    writer.write_record(reader.headers().expect("").iter());

    reader.records().filter(|r| {r.is_ok()}).map(|r|{r.expect("")}).for_each( |row| {
        let id_vec: Vec<String> = id_indices.iter().map(|i| {
            row.get(*i).expect("").to_string()
        }).collect();

        let id = id_vec.join("||");
        if !seen_ids.contains(&id) {
            seen_ids.insert(id);
            writer.write_record(row.iter());
        }

    });
}


fn main() {
    let arguments: Vec<String> = env::args().collect();
    unique_by_columns(arguments);
}
