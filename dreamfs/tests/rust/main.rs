#[test]
fn version_matches() {
    let version = dreamfs::version();
    assert_eq!(version.to_str().unwrap(), "0.1.0");

    let files = dreamfs::list(".".into());
    println!("{} files, {} bytes", files.len(), dreamfs::total_size(files));
}
