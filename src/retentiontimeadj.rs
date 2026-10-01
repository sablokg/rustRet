use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok,
codeprog@icloud.com
*/

pub fn retentionadjust(pathfile: &str, adjustvalue: &str) -> Result<String, Box<dyn Error>> {
    let fileopen = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(fileopen);

    let mut column_length: Vec<f32> = Vec::new();
    let mut flow_rate: Vec<f32> = Vec::new();
    let mut estimaterate: Vec<f32> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("file not present");
        let linevec = line
            .split(",")
            .map(|x| x.parse::<f32>().unwrap())
            .collect::<Vec<_>>();
        column_length.push(linevec[0].clone());
        flow_rate.push(linevec[1].clone());
    }
    for i in 0..column_length.len() {
        estimaterate.push(column_length[i] / flow_rate[i] as f32);
    }

    let mut finalestimate: Vec<f32> = Vec::new();
    for i in estimaterate.iter() {
        let value = i * (1 as f32 + adjustvalue.parse::<f32>().unwrap());
        finalestimate.push(value);
    }

    let mut filewrite = File::open("RetentionTime").expect("file not present");
    writeln!(
        filewrite,
        "{}\t{}\t{}",
        "column_length", "flowrate", "estimaterate"
    )
    .expect("column not found");
    for i in 0..column_length.len() {
        writeln!(
            filewrite,
            "{}\t{}\t{}",
            column_length[i], flow_rate[i], finalestimate[i]
        )
        .expect("value not present");
    }
    Ok("Time of Retention has been calculated".to_string())
}
