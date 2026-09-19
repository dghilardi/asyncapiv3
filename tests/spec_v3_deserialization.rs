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

#[cfg(feature = "preserve_order")]
#[test]
fn preserve_order() {
    fn assert_eq_with_preserved_order(a: &AsyncApiSpec, b: &AsyncApiSpec) {
        assert_eq!(
            serde_yaml::to_string(a).unwrap(),
            serde_yaml::to_string(b).unwrap(),
        );
    }

    let spec = deserialize_spec(Path::new(
        "./test-res/3.0.0/kraken-websocket-request-reply-message-filter-in-reply-asyncapi.yml",
    ))
    .unwrap();

    assert_eq_with_preserved_order(&spec, &spec);

    // spec -> json string -> spec
    let roundtrip = serde_json::from_str(&serde_json::to_string(&spec).unwrap()).unwrap();
    assert_eq_with_preserved_order(&spec, &roundtrip);

    // spec -> json value -> spec
    let roundtrip = serde_json::from_value(serde_json::to_value(&spec).unwrap()).unwrap();
    assert_eq_with_preserved_order(&spec, &roundtrip);

    // spec -> yaml string -> spec
    let roundtrip = serde_yaml::from_str(&serde_yaml::to_string(&spec).unwrap()).unwrap();
    assert_eq_with_preserved_order(&spec, &roundtrip);

    // spec -> yaml value -> spec
    let roundtrip = serde_yaml::from_value(serde_yaml::to_value(&spec).unwrap()).unwrap();
    assert_eq_with_preserved_order(&spec, &roundtrip);
}
