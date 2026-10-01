use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
*/

pub fn retentionindexcal(pathfile: &str) -> Result<String, Box<dyn Error>> {
    let filevector = File::open(pathfile).expect("file not present");
    let fileread = BufReader::new(filevector);
    let mut elutes: Vec<f64> = Vec::new();
    let mut tr_read: Vec<f64> = Vec::new();
    let mut tn_read: Vec<f64> = Vec::new();
    let mut tn_1_read: Vec<f64> = Vec::new();

    for i in fileread.lines() {
        let line = i.expect("line not found");
        let linevec = line
            .split("\t")
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<_>>();
        println!("{:?}", linevec);
        elutes.push(linevec[0]);
        tr_read.push(linevec[1]);
        tn_read.push(linevec[2]);
        tn_1_read.push(linevec[3]);
    }

    let mut difference_upper: Vec<f64> = Vec::new();
    let mut difference_lower: Vec<f64> = Vec::new();

    for i in 0..tr_read.len() {
        let value_upper: f64 = tr_read[i] - tn_read[i];
        let value_lower: f64 = tn_1_read[i] - tn_read[i];
        difference_upper.push(value_upper);
        difference_lower.push(value_lower);
    }
    let mut finalvalue: Vec<f64> = Vec::new();
    for i in 0..difference_upper.len() {
        finalvalue.push(difference_upper[i] / difference_lower[i]);
    }

    let mut retentionindex: Vec<(f64, f64)> = Vec::new();
    for i in 0..elutes.len() {
        let retentionindex_cal: (f64, f64) = (elutes[i], 100f64 * (elutes[i] + finalvalue[i]));
        retentionindex.push(retentionindex_cal);
    }

    let mut filewrite = File::create("retentionindex.txt").expect("file not present");
    writeln!(filewrite, "{}\t{}", "elutes", "retentiontime").expect("line not present");
    for i in retentionindex.iter() {
        writeln!(filewrite, "{}\t{}", i.0, i.1).expect("line not present");
    }

    Ok("path to the retention time has been written".to_string())
}
