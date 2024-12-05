# Guide

## Running the Server
cargo build --release
target/release/chat
- **Server Initialization Messages**:
  - `Sqlite database initialized successfully.`
  - `Tables created successfully.`
  - `Sled database initialized successfully.`
  - The server is currently listening on `localhost:8080`.

## API Endpoints and `curl` Commands

### 1. Register User

Use the following command to register a new user:

    curl http://localhost:8080/user/register --json '{"username": "Connor", "password": "connor123"}'

### 2. Login User

Use the following command to login and save the session cookies:

    curl http://localhost:8080/user/login --json '{"username": "Connor", "password": "connor123"}' -c cookies.txt -b cookies.txt

### 3. Logout User

Use the following command to logout and update the session cookies:

    curl -b cookies.txt -c cookies.txt -X POST http://localhost:8080/user/logout

### 4. Create Channel

curl "http://localhost:8080/channel/create" -c cookies.txt -b cookies.txt --json '{"name": "General"}'
