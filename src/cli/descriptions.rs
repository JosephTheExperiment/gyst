pub fn descriptions(subcommand: &str) -> String {
    let dis = match subcommand {
        "install" => {
            "Install libraries via conan using the the name and version of the library
if you don't know if the library or the version you need dose exist
you can visit conan center at https://conan.io/center"
        }
        _ => "",
    };
    return dis.to_string();
}
