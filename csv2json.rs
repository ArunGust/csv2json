use csv::*;
use json::*;

pub async fn jsontocsv(jsonstring: String, fullpath: String) -> core::result::Result<(), Box<dyn std::error::Error>> {
    let mut wtr =Writer::from_path(fullpath)?;
    let _parsed = json::parse(&jsonstring).unwrap();    
    for i in 0.._parsed.len() {
        let record: Vec<String> = _parsed[i].members().map(|x| x.to_string()).collect::<Vec<String>>();
        wtr.write_record(record).unwrap();
    }
    wtr.flush()?;
    Ok(())
}

pub async fn json2csv(list: String, fullpath: String) -> core::result::Result<(), Box<dyn std::error::Error>> {
    //println!("createsheet : {:?}", list.clone());
    let mut wtr = Writer::from_path(fullpath)?;
    let parsed  = json::parse(&list).unwrap();
    
    let mut headerrow = Vec::new();
    parsed[0].entries().for_each(|value| {
        headerrow.push(value.0.to_string());
    });
    wtr.write_record(&headerrow)?;

    for row in 0..parsed.len() {
        let mut colrow= Vec::new();
        parsed[row].entries().for_each(|value| {
            colrow.push(value.1.to_string());
        });
        wtr.write_record(&colrow)?;
    }
    
    wtr.flush()?;
    Ok(())
}

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
