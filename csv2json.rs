use csv::*;
use json::*;

pub async fn csvtojson(data: String) -> core::result::Result<JsonValue, Box<dyn std::error::Error>>{
    let mut rdr = csv::ReaderBuilder::new().from_reader(data.as_bytes());
    let _header: Vec<String> = rdr.headers().unwrap().into_iter().map(|f| f.to_string()).collect();
    let mut jarray = JsonValue::new_array();
    for result in rdr.records() {
        let record = result.unwrap();
        let mut jdata = JsonValue::new_object();
        for i in 0..record.len() {
            jdata[_header[i].clone()] = record[i].into();
        }
        jarray.push(jdata).unwrap();
    }
    println!("creat esheet completed {:?} ", jarray);
    Ok(jarray)
}
