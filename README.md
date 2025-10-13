# GRAPE-LM Webserver

This repository provides the web server implementation for the **GRAPE-LM** (Generator of RNA Aptamers Powered by activity-guided Evolution and Language Model) method.

🌐 **Try GRAPE-LM online at: [https://grape-lm.bioailab.net](https://grape-lm.bioailab.net)**

## How to Run

### 1. Local Environment

**Requirements:**

-   Python 3.10
-   Node.js & npm (for building the frontend)
-   Rust toolchain (for building the backend)

**Steps:**

1. **Install Python dependencies:**

    ```bash
    pip install -r requirements.txt
    ```

2. **Build the frontend:**
   In the `webpage` directory, install dependencies and build the frontend:

    ```bash
    npm install
    npm run build
    ```

3. **Build the backend:**
   In the `webserver` directory, build the backend using Cargo:

    ```bash
    cargo build --release
    ```

4. **Run the webserver:**
   From the `project root` directory, execute:
    ```bash
    ./webserver/target/release/grape-lm-webserver <project_dir> <address:port> <max_concurrent_tasks>
    ```
    - `<project_dir>`: Path to your GRAPE project directory
    - `<address:port>`: Address and port to bind (e.g., `127.0.0.1:12358`)
    - `<max_concurrent_tasks>`: Maximum number of concurrent tasks to process

---

### 2. Docker

You can also run the webserver using Docker for easier deployment:

```bash
docker build -t grape-lm-webserver .
docker run -d \
  --name grape-lm-webserver \
  --restart unless-stopped \
  -p 127.0.0.1:12358:12358 \
  -v <model_parameters_dir>:/app/model_parameters \
  -v <tasks_dir>:/app/tasks \
  grape-lm-webserver
```

If you need to change the startup parameters (such as address, or maximum concurrent tasks), you can modify the CMD section in the Dockerfile to fit your requirements.
