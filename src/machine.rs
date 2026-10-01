use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::metrics::accuracy;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
*/

pub fn regression(
    pathfasta: &str,
    paththreshold: &str,
    pathpredict: &str,
) -> Result<String, Box<dyn Error>> {
    let filefasta = File::open(pathfasta).expect("file not present");
    let filefasta_read = BufReader::new(filefasta);
    let mut fastafilevec: Vec<String> = Vec::new();
    let mut fastafileseq: Vec<String> = Vec::new();
    for i in filefasta_read.lines() {
        let line = i.expect("");
        if line.starts_with("#") {
            fastafilevec.push(line.replace(">", ""));
        }
        if !line.starts_with("#") {
            fastafileseq.push(line);
        }
    }
    let mut predictname: Vec<String> = Vec::new();
    let mut predictseq: Vec<String> = Vec::new();
    let predictopen = File::open(pathpredict).expect("file not present");
    let predictread = BufReader::new(predictopen);
    for i in predictread.lines() {
        let line = i.expect("line not present");
        if line.starts_with("#") {
            predictname.push(line.replace(">", ""))
        }
        if !line.starts_with("#") {
            predictseq.push(line)
        }
    }
    let pathvec_a = vectorcast(fastafileseq).unwrap();
    let predict_b: Vec<Vec<Vec<f32>>> = vectorcast(predictseq).unwrap();
    let mut classification: Vec<i32> = Vec::new();
    let classificationopen = File::open(paththreshold).expect("file not present");
    let classread = BufReader::new(classificationopen);
    for i in classread.lines() {
        let line = i.expect("line not present");
        let linevalue = line.split(",").collect::<Vec<_>>()[0]
            .parse::<i32>()
            .unwrap();
        classification.push(linevalue);
    }

    let mut pathvec_a_unpack: Vec<Vec<f32>> = Vec::new();
    for i in pathvec_a.iter() {
        let value = i.iter().flatten().cloned().collect::<Vec<f32>>();
        pathvec_a_unpack.push(value);
    }
    let mut pathvec_b_unpack: Vec<Vec<f32>> = Vec::new();
    for i in predict_b.iter() {
        let value = i.iter().flatten().cloned().collect::<Vec<f32>>();
        pathvec_b_unpack.push(value);
    }
    let inputmatrix = DenseMatrix::from_2d_vec(&pathvec_a_unpack).unwrap();
    let prediction_matrix = DenseMatrix::from_2d_vec(&pathvec_b_unpack).unwrap();
    let model = LogisticRegression::fit(&inputmatrix, &classification, Default::default()).unwrap();
    let predictvalue = model.predict(&prediction_matrix).unwrap();
    let accuracyvalue = accuracy(&classification, &predictvalue);
    print!("The accuracy value for the model is {:?}", accuracyvalue);
    Ok("The file has been written".to_string())
}

pub fn vectorcast(vecstring: Vec<String>) -> Result<Vec<Vec<Vec<f32>>>, Box<dyn Error>> {
    let vecstringclone = vecstring;
    let mut vecconvert_a: Vec<Vec<Vec<f32>>> = Vec::new();
    for i in vecstringclone.iter() {
        let seqchars = i.chars().collect::<Vec<char>>();
        let mut newvec: Vec<Vec<f32>> = Vec::new();
        for i in seqchars.iter() {
            match i {
                'A' => newvec.push(vec![1.0, 0.0, 0.0, 0.0]),
                'T' => newvec.push(vec![0.0, 1.0, 0.0, 0.0]),
                'G' => newvec.push(vec![0.0, 0.0, 1.0, 0.0]),
                'C' => newvec.push(vec![0.0, 0.0, 0.0, 1.0]),
                'N' => newvec.push(vec![1.0, 1.0, 1.0, 1.0]),
                _ => continue,
            }
        }
        vecconvert_a.push(newvec);
    }
    let mut vecconvert_b: Vec<Vec<Vec<f32>>> = Vec::new();
    let maxlength = vecconvert_a
        .iter()
        .flatten()
        .map(|x| x.len())
        .collect::<Vec<_>>()
        .into_iter()
        .max()
        .unwrap();
    for i in vecconvert_a.iter() {
        let veclength = i.len();
        let lengthdifference = veclength as f32 - maxlength as f32;
        if lengthdifference == 0f32 {
            continue;
        } else if lengthdifference != 0f32 {
            let mut newvec = i.clone();
            let mut vecadd: Vec<Vec<f32>> = Vec::new();
            for _i in 0..lengthdifference as i32 {
                vecadd.push(vec![0.0, 0.0, 0.0, 0.0]);
            }
            newvec.append(&mut vecadd);
            vecconvert_b.push(newvec);
        }
    }
    Ok(vecconvert_b)
}
