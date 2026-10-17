use miilo_adapter_sdk::compression::{Codec, Limits, bundle};
fn main() {
    let dir = std::env::args().nth(1).expect("fixture directory");
    let path = std::path::Path::new(&dir);
    let input = std::fs::read(path.join("payload")).unwrap();
    for (name, codec) in [
        ("gzip", Codec::Gzip),
        ("brotli", Codec::Brotli),
        ("zstd", Codec::Zstd),
    ] {
        let from_python = std::fs::read(path.join(format!("python-{name}"))).unwrap();
        assert_eq!(
            bundle::decode(&from_python, Limits::default()).unwrap().1,
            input
        );
        std::fs::write(
            path.join(format!("rust-{name}")),
            bundle::encode(codec, &input, Limits::default()).unwrap(),
        )
        .unwrap();
    }
}
