use once_cell::sync::Lazy;
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

// if you change the token length you most change validate request length
pub const VERIFY_CODE_LEN: usize = 5;
pub const EXPIRE_SESSION_CODE_SECS: Duration = Duration::from_secs(2000);
pub const EXPIRE_TWO_FACTOR_CODE_SECS: Duration = Duration::from_secs(200);
pub const EXPIRE_BEARER_TOKEN_SECS: Duration = Duration::from_secs(600);
pub const EXPIRE_REFRESH_TOKEN_SECS: Duration = Duration::from_secs(3600);

pub const AUTHORIZATION: &str = "Authorization";
pub const BEARER: &str = "Bearer";

pub const APP_EMAIL_ADDR: &str = "mofarahali76@gmail.com";
pub static IMAGES_PATH: Lazy<PathBuf> = Lazy::new(|| root_dir("static/images").unwrap());
pub static APP_IMAGE: Lazy<PathBuf> = Lazy::new(|| root_dir("static/images/logo.jpg").unwrap());

pub fn root_dir<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
    Ok(
        project_root::get_project_root()
            .or_else(|_| std::env::current_dir())?
            .join(path),
    )
}
