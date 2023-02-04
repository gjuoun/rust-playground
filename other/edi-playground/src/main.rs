use edi::parse;
use std::fs::read_to_string;
use serde_json;

fn main() {
    // let edi_file_path = format!("{}/examples/sample_edi.txt", env!("CARGO_MANIFEST_DIR"));
    let edi_file_path = format!("{}/examples/10_12 claims-edi.txt", env!("CARGO_MANIFEST_DIR"));
    
    let edi_string = read_to_string(edi_file_path).unwrap();
    let edi_document = parse(&edi_string).unwrap();
    // `edi_document` now contains an `EdiDocument` which we can interact with.
    // println!("{:#?}", edi_document.interchanges);

    let json = serde_json::to_string_pretty(&edi_document).unwrap();

    println!("{}", json);


}
