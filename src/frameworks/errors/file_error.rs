use thiserror::Error;

#[derive(Debug, Error)]
pub enum FileError {
    #[error("檔案大小錯誤")]
    SizeError,
    #[error("檔案類型不允許")]
    TypeNotAllow,
    #[error("檔案名稱取得失敗")]
    FileNameEmpty,
    #[error("上傳目錄取得失敗")]
    UploadDirEmpty,
    #[error("檔案內容取得失敗")]
    FileContentEmpty,
}