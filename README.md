# 說明

- 這是一個 actix_web + mongo + google cloud storage 的練習專案，會盡量把所有功能都練習到

## 環境

- .env 檔需設置: 
  - DB_URI：連線到 MongoDB 
  - SERVER_HOST：伺服器的 IP
  - SERVER_PORT：伺服器的 Port
  - GOOGLE_APPLICATION_CREDENTIALS：Service Account 的認證檔案路徑
  - GCS_BUCKET：Google Cloud Storage 的 Bucket 名稱

## 目前已有範例

### Actix Web

- Json
- Multipart

### Google Cloud Storage

- upload

### MongoDB

- insert
- delete