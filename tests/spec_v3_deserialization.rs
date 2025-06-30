use asyncapiv3::spec::AsyncApiSpec;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[test]
fn deserialize_asyncapi_v3_examples() {
    let mut paths = fs::read_dir("./test-res/3.0.0")
        .unwrap()
        .map(|entry| entry.expect("Cannot read direntry").path())
        .collect::<Vec<_>>();

    paths.sort();

    for path in paths {
        deserialize_spec(&path).unwrap_or_else(|_| panic!("Cannot deserialize spec {path:?}"));

        println!(" * {path:?} - Deserialized");
    }
}

fn deserialize_spec(entry: &Path) -> Result<AsyncApiSpec, Box<dyn std::error::Error>> {
    let file = File::open(entry)?;
    let reader = BufReader::new(file);
    let spec = serde_yaml::from_reader::<_, AsyncApiSpec>(reader).inspect_err(|err| {
        println!("Error location: {:?}", err.location());
    })?;
    Ok(spec)
}
