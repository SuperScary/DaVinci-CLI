extern crate embed_resource;

fn main() {
    println!("cargo:rerun-if-changed=resources.rc");
    println!("cargo:rerun-if-changed=icon.ico");
    embed_resource::compile("resources.rc", embed_resource::NONE).manifest_optional().unwrap();
}