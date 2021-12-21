Rust JWT Auth Service

|METHOD|PATH|PAYLOAD|HEADER|
|------|----|-------|------|
|`POST`|`/api/auth/login`|`{ "username": "user", "password": "1234" }`||
|`GET`|`/api/auth/logout`||Authorization: jwt|
|`POST`|`/api/auth/token`||Authorization: refresh_token|
|`POST`|`/api/password/reset`|`{}`||