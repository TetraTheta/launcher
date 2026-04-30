fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_manifest_file("resource/app.manifest");
        if let Err(err) = res.compile() {
            panic!("failed to compile Windows resources: {err}")
        }
    }
}
