# surreal-with-backup
This repository builds surrealdb and its backup manager into a docker container

## ENV

backup manager has

- DB_USERNAME = "root"
- DB_PASSWORD = "root"
- DB_URL = "http://localhost:8000"
- S3_ACCESS_KEY = ""
- S3_SECRET_KEY = ""
- S3_SECURITY_TOKEN = ""
- S3_SESSION_TOKEN = ""
- S3_BUCKET_NAME = ""
- S3_REGION = ""
- S3_ENDPOINT = ""
- PERIOD = "daily" | "hourly"

surreal has its own keys like user pass etc..
