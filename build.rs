
fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("macos") => {
            print!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.11")
        }
        Ok("windows") | Ok("linux") => {
            // noop
        }
        tos => panic!("unknown target os {:?}!", tos)
    }
}
