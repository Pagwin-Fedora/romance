use std::path;


/// Returns the path to the folder that is being used as a hub for various files that the program
/// temporarily needs
fn get_hub_path<'a>()-> &'a path::Path {
    path::Path::new(concat!("/tmp/",std::env!("CARGO_PKG_NAME")))
}
/// returns the path to the location the repo is cloned to
pub fn get_repo_path()-> Result<path::PathBuf,std::io::Error>{
    let mut repo = get_proc_path()?;
    repo.push("repo");
    Ok(repo)
}
/// Returns the name of the dir of the bare repo for usage as an id in the hub
fn get_id()->Result<String,std::io::Error>{
    let pwd = std::env::current_dir()?;
    //pwd.pop();
    pwd.file_name()
        .ok_or(std::io::Error::new(std::io::ErrorKind::NotFound,"couldn't get name of the dir"))
        .map(std::ffi::OsStr::to_os_string)
        .map(std::ffi::OsString::into_string)
        .map(Result::unwrap)
}

pub fn get_proc_path()->Result<path::PathBuf, std::io::Error>{
    let mut hub = get_hub_path().to_path_buf();
    hub.push(get_id()?);
    Ok(hub)
}
