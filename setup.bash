# Step 1: Install Rust and wasm-pack
cargo install wasm-pack

# Step 2: Clone the project and navigate into it
git clone https://github.com/Connor315/ChatApp.git
cd ChatApp

# Step 3: Navigate to the frontend folder, compile it, and output to the static directory
cd frontend
wasm-pack build --target web --out-dir ../static --release

# Step 4: Build and run the backend from the project root directory
cd ..
cargo build --release
./target/release/chat