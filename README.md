# 說明

- 這是一個用 rust 做伺服器的練習專案，會盡量把所有功能都練習到，測試部分只有寫成功的，例外與失敗的測試會在後續補上。

## 環境

- .env 檔需設置:
    - DB_URI：連線到 MongoDB
    - DB_NAME：MongoDB 的資料庫名稱
    - SERVER_HOST：伺服器的 IP
    - SERVER_PORT：伺服器的 Port
    - SECRET：JWT 的加密字串

## 目前已有範例

### JsonWebToken

- encode
- decode
- decode_from_req

### Actix Web

- Json
- Query
- Path

### MongoDB

- insert_one
- find_one
- delete_one