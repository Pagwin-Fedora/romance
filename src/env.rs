use std::path::Path;


/// Returns the path to the folder that is being used as a hub for various files that the program
/// temporarily needs
pub fn get_hub_path<'a>()-> &'a std::path::Path {
    Path::new(concat!("/tmp/",std::env!("CARGO_PKG_NAME")))
}

/// Returns the name of the dir of the bare repo for usage as an id in the hub
pub fn get_id()->Result<String,std::io::Error>{
    let mut pwd = std::env::current_dir()?;
    pwd.pop();
    pwd.file_name()
        .ok_or(std::io::Error::new(std::io::ErrorKind::NotFound,"couldn't get name of the dir"))
        .map(std::ffi::OsStr::to_os_string)
        .map(std::ffi::OsString::into_string)
        .map(Result::unwrap)
}
