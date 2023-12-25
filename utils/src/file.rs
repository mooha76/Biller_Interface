use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

use errors::AppResult;

pub use constants::root_dir;

pub async fn save_file(file_path: &Path, content: &[u8]) -> AppResult<()> {
    if let Some(parent_dir) = file_path.parent() {
        fs::create_dir_all(&parent_dir).await?;
    }
    let mut file = fs::File::create(&file_path).await?;
    file.write_all(content).await?;
    Ok(())
}
