use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use asyncapiv3::spec::AsyncApiSpec;

#[test]
fn deserialize_asyncapi_v3_examples() {
    let mut paths = fs::read_dir("./test-res/3.0.0")
        .unwrap()
        .into_iter()
        .map(|entry| entry.expect("Cannot read direntry").path())
        .collect::<Vec<_>>();

    paths.sort();

    for path in paths {
        deserialize_spec(&path)
            .expect(&format!("Cannot deserialize spec {path:?}"));

        println!(" * {path:?} - Deserialized");
    }
}

fn deserialize_spec(entry: &Path) -> Result<AsyncApiSpec, Box<dyn std::error::Error>> {
    let file = File::open(entry)?;
    let reader = BufReader::new(file);
    let spec = serde_yaml::from_reader::<_, AsyncApiSpec>(reader)
        .map_err(|err| {
            println!("Error location: {:?}", err.location());
            err
        })?;
    Ok(spec)
}