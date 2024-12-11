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

    curl http://localhost:8080/user/register --json '{"username": "Chen", "password": "chen123"}'

### 2. Login User

Use the following command to login and save the session cookies:

    curl http://localhost:8080/user/login --json '{"username": "Connor", "password": "connor123"}' -c cookies.txt -b cookies.txt

    curl http://localhost:8080/user/login --json '{"username": "Chen", "password": "chen123"}' -c cookies1.txt -b cookies1.txt

### 3. Logout User

Use the following command to logout and update the session cookies:

    curl -b cookies.txt -c cookies.txt -X POST http://localhost:8080/user/logout

### 4. Create Channel

curl "http://localhost:8080/channel/create" -c cookies.txt -b cookies.txt --json '{"name": "General"}'

wasm-pack build --target web --out-dir ../static


### 5. Enter Channel

curl -X GET \
  -H "Content-Type: application/json" \
  -b cookies.txt \
  http://localhost:8080/channel/enter/General

### 6. Chat in a channel

cargo install websocat

websocat "ws://localhost:8080/channel/ws/General" --header "Cookie: id=$(grep "id" cookies.txt | cut -f7)"  `first user`

websocat "ws://localhost:8080/channel/ws/General" --header "Cookie: id=$(grep "id" cookies1.txt | cut -f7)"   `second user`

then, send a message: "Hello, everyone!"

### 7. Retrieve chat history

curl -b cookies.txt -c cookies.txt http://localhost:8080/channel/history/General   `first user`

curl -b cookies1.txt -c cookies1.txt http://localhost:8080/channel/history/General   `second user` 

