use std::process::Command;

fn main() {
    let _ = Command::new("sh")
                        .args(&["-c", "cd data && glib-compile-resources resources.gresource.xml"])
                        .output()
                        .expect("failed to execute process");

}
