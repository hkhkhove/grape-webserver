use axum::{
    Router,
    extract::{Multipart, Path, State},
    http::{StatusCode, header::CONTENT_TYPE},
    response::{Json, Response},
    routing::{get, post},
};
use sqlx::SqlitePool;
use std::{collections::HashMap, env, path::PathBuf, sync::Arc};
use tokio::{
    fs,
    io::AsyncWriteExt,
    sync::{Mutex, Semaphore},
};
use tower_http::services::{ServeDir, ServeFile};

mod config;
mod database;
mod models;
mod tasks;

use config::Config;
use models::{Task, TaskCreateResponse, TaskResponse, TaskStatus};

#[derive(Clone)]
struct AppState {
    cpu_task_semaphore: Arc<Semaphore>,
    db_pool: SqlitePool,
    task_sender: tokio::sync::mpsc::UnboundedSender<String>,
    task_queue: Arc<Mutex<std::collections::VecDeque<String>>>,
}

// API处理函数
async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "GRAPE-LM API Server",
        "version": "0.1.0"
    }))
}

// 接收用户上传的任务，添加到任务队列
// 如果 Ok(Json(...))，就返回一个 200 状态码、JSON 格式的 HTTP 响应
// 如果 Err((StatusCode, String))，axum 会自动把这个元组转换成 HTTP 响应
async fn upload_task(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<TaskCreateResponse>, (StatusCode, String)> {
    let mut form_data = HashMap::new();
    let mut file_fields = Vec::new(); // 暂存文件数据

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        let field_name = field.name().unwrap_or("unknown").to_string();

        if let Some(filename) = field.file_name() {
            let filename = filename.to_string();
            let data = field
                .bytes()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
            file_fields.push((field_name, filename, data));
        } else {
            let value = field
                .text()
                .await
                .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
            form_data.insert(field_name, value);
        }
    }
    let task_id = form_data
        .get("task_id")
        .ok_or((StatusCode::BAD_REQUEST, "task_id is required".to_string()))?
        .clone();
    let task_name = form_data
        .get("task_name")
        .ok_or((StatusCode::BAD_REQUEST, "task_name is required".to_string()))?
        .clone();
    let seed_seqs = form_data
        .get("seed_seqs")
        .ok_or((StatusCode::BAD_REQUEST, "seed_seqs is required".to_string()))?
        .clone();
    let gen_num = form_data
        .get("gen_num")
        .ok_or((StatusCode::BAD_REQUEST, "gen_num is required".to_string()))?
        .clone();

    validate_seed_sequences(&seed_seqs)?;
    validate_gen_num(&gen_num)?;

    let home = Config::home();

    let upload_dir = home.join("tasks").join("uploads").join(task_id.to_string());
    let results_dir = home.join("tasks").join("results").join(task_id.to_string());

    fs::create_dir_all(&upload_dir)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    fs::create_dir_all(&results_dir)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut uploaded_files = Vec::new();
    for (_field_name, filename, data) in file_fields {
        let file_path = upload_dir.join(&filename);
        let mut file = fs::File::create(&file_path)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        file.write_all(&data)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        uploaded_files.push(filename);
    }

    //把task的信息保存到数据库中
    sqlx::query(r#"INSERT INTO tasks (id, name, status, upload_time) VALUES (?, ?, ?, ?)"#)
        .bind(&task_id)
        .bind(task_name)
        .bind(TaskStatus::Pending)
        .bind(chrono::Utc::now())
        .execute(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let result_path = home
        .join("tasks")
        .join("results")
        .join(task_id.to_string())
        .join(format!("generation_{}.txt", task_id));

    form_data.insert(
        "output_file".to_string(),
        result_path.to_string_lossy().to_string(),
    );

    //保存form_data
    let form_data_path = home
        .join("tasks")
        .join("uploads")
        .join(task_id.to_string())
        .join("form_data.json");
    let form_data_json = serde_json::to_string(&form_data).unwrap();
    fs::write(form_data_path, form_data_json)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 添加task到队列
    state.task_queue.lock().await.push_back(task_id.clone());

    // 通过通道通知调度器有新任务
    if let Err(_) = state.task_sender.send(task_id.clone()) {
        eprintln!("Failed to send task {} to dispatcher", task_id);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to queue task".to_string(),
        ));
    }

    Ok(Json(TaskCreateResponse {
        task_id,
        message: "Task created successfully and added to the queue.".to_string(),
    }))
}

fn validate_seed_sequences(seed_seqs: &str) -> Result<(), (StatusCode, String)> {
    const MAX_SEQUENCES: usize = 10000;

    let lines: Vec<&str> = seed_seqs
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();

    // 检查数量限制
    if lines.len() > MAX_SEQUENCES {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Maximum {} sequences allowed. Got: {}",
                MAX_SEQUENCES,
                lines.len()
            ),
        ));
    }

    // 检查每个序列
    for (i, line) in lines.iter().enumerate() {
        // 只允许 ACGU 字符
        if !line
            .chars()
            .all(|c| matches!(c.to_ascii_uppercase(), 'A' | 'C' | 'G' | 'U'))
        {
            return Err((
                StatusCode::BAD_REQUEST,
                format!(
                    "Line {}: Invalid characters. Only A, C, G, U allowed",
                    i + 1
                ),
            ));
        }

        // 长度必须为 20
        if line.len() != 20 {
            return Err((
                StatusCode::BAD_REQUEST,
                format!(
                    "Line {}: Sequence must be 20 characters long. Got: {}",
                    i + 1,
                    line.len()
                ),
            ));
        }
    }

    Ok(())
}

fn validate_gen_num(gen_num_str: &str) -> Result<(), (StatusCode, String)> {
    let gen_num = gen_num_str.parse::<u32>().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "gen_num must be a valid number".to_string(),
        )
    })?;
    if gen_num < 1 || gen_num > 10000 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Generated sequences count must be between 1 and 10000".to_string(),
        ));
    }
    Ok(())
}

// 获取特定任务状态
async fn get_task_status(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<TaskResponse>, (StatusCode, String)> {
    let task: Task = sqlx::query_as("SELECT * FROM tasks WHERE id = ?")
        .bind(&task_id)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    let response = match task.status {
        TaskStatus::Pending => {
            let queue = state.task_queue.lock().await;
            let position = queue
                .iter()
                .position(|id| id == &task_id)
                .map(|p| p + 1 + Config::workers());
            TaskResponse::Pending {
                upload_time: task.upload_time,
                start_time: "".to_string(),
                end_time: "".to_string(),
                position: position,
            }
        }
        TaskStatus::Processing => TaskResponse::Processing {
            upload_time: task.upload_time,
            start_time: task.start_time,
            end_time: "".to_string(),
        },
        TaskStatus::Completed => TaskResponse::Completed {
            upload_time: task.upload_time,
            start_time: task.start_time,
            end_time: task.end_time,
        },
        TaskStatus::Failed => TaskResponse::Failed {
            upload_time: task.upload_time,
            start_time: task.start_time,
            end_time: task.end_time,
            error: task
                .error_message
                .clone()
                .unwrap_or_else(|| "Unknown error".to_string()),
        },
    };

    Ok(Json(response))
}

async fn download_result(Path(task_id): Path<String>) -> Result<Response, (StatusCode, String)> {
    let home = Config::home();
    let result_path = home
        .join("tasks")
        .join("results")
        .join(task_id.to_string())
        .join(format!("generation_{}.txt", task_id));

    // 检查文件是否存在
    if !std::path::Path::new(&result_path).exists() {
        return Err((StatusCode::NOT_FOUND, "Task not found".to_string()));
    }

    // 读取文件
    let file_content = fs::read(&result_path).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to read form data file.".to_string(),
        )
    })?;

    // 返回文件
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "text/plain; charset=utf-8")
        .header(
            "Content-Disposition",
            format!("attachment; filename=\"{}_generation.txt\"", task_id),
        )
        .body(file_content.into())
        .unwrap())
}

// 任务调度器
async fn task_dispatcher(
    mut task_receiver: tokio::sync::mpsc::UnboundedReceiver<String>,
    db_pool: SqlitePool,
    cpu_task_semaphore: Arc<Semaphore>,
    task_queue: Arc<Mutex<std::collections::VecDeque<String>>>,
    home: PathBuf,
) {
    while let Some(task_id) = task_receiver.recv().await {
        // 获取信号量许可
        let permit = cpu_task_semaphore.clone().acquire_owned().await.unwrap();

        {
            // //将当前task移出队列
            // let mut queue = task_queue.lock().await;
            // if let Some(pos) = queue.iter().position(|id| id == &task_id) {
            //     queue.remove(pos);
            // }
            // 直接移除队列首部任务
            let mut queue = task_queue.lock().await;
            if let Some(front_task) = queue.pop_front() {
                // 验证是否是期望的任务
                if front_task != task_id {
                    eprintln!("Warning: Expected task {}, but got {}", task_id, front_task);
                }
            }
        }

        let pool_clone = db_pool.clone();
        let home_clone = home.clone();

        tokio::spawn(async move {
            // 从文件读取form_data
            let form_data_path = home_clone
                .join("tasks")
                .join("uploads")
                .join(&task_id)
                .join("form_data.json");
            let form_data = match fs::read_to_string(form_data_path).await {
                Ok(s) => serde_json::from_str(&s).unwrap_or_default(),
                Err(e) => {
                    let error_message = Some(format!("Task execution failed: {}", e));
                    if let Err(db_err) = sqlx::query(
                        "UPDATE tasks SET status = ?, end_time = ?, error_message = ? WHERE id = ?",
                    )
                    .bind(TaskStatus::Failed)
                    .bind(chrono::Utc::now())
                    .bind(error_message)
                    .bind(&task_id)
                    .execute(&pool_clone)
                    .await
                    {
                        eprintln!(
                            "Database error while updating failed task {}: {}",
                            task_id, db_err
                        );
                    }
                    return;
                }
            };

            tasks::process_task(form_data, pool_clone).await;

            drop(permit);
        });
    }

    println!("Task dispatcher stopped");
}

#[tokio::main]
async fn main() {
    println!("Usage: grape-lm-webserver <work_dir> <address> <max_workers>");
    let home = env::args()
        .nth(1)
        .map(|arg| PathBuf::from(arg))
        .unwrap_or_else(|| PathBuf::from("./"));

    let addr = env::args().nth(2).unwrap_or("127.0.0.1:12358".to_string());

    let workers = env::args()
        .nth(3)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(2);

    if let Err(e) = Config::init(home.clone(), addr.clone(), workers) {
        eprintln!("Failed to initialize config: {}", e);
        std::process::exit(1);
    }

    println!("Using work directory: {}", Config::home().display());
    println!("Server will start on: {}", Config::addr());

    let tasks_dir = home.join("tasks");
    fs::create_dir_all(&tasks_dir)
        .await
        .expect("Failed to create tasks directory");

    let database_url = format!("sqlite:{}", home.join("tasks").join("tasks.db").display());

    // 初始化数据库
    let db_pool = database::init_db(&database_url)
        .await
        .expect("Failed to initialize database");

    // 创建任务通道
    let (task_sender, task_receiver) = tokio::sync::mpsc::unbounded_channel::<String>();

    // 初始化 AppState
    let app_state = AppState {
        cpu_task_semaphore: Arc::new(Semaphore::new(Config::workers())),
        db_pool: db_pool.clone(),
        task_sender,
        task_queue: Arc::new(Mutex::new(std::collections::VecDeque::new())),
    };

    //创建并启动任务调度器
    let cpu_task_semaphore = app_state.cpu_task_semaphore.clone();
    let task_queue = app_state.task_queue.clone();
    tokio::spawn(task_dispatcher(
        task_receiver,
        db_pool,
        cpu_task_semaphore,
        task_queue,
        home.clone(),
    ));

    let api_routes = Router::new()
        .route("/", get(root))
        .route("/tasks", post(upload_task))
        .route("/tasks/{task_id}", get(get_task_status))
        .route("/tasks/{task_id}/download", get(download_result))
        .with_state(app_state);

    let app = Router::new()
        .nest("/api", api_routes)
        .nest_service(
            "/favicon.ico",
            ServeFile::new(home.join("webpage").join("dist").join("grape-lm.ico")),
        )
        .nest_service(
            "/assets",
            ServeDir::new(home.join("webpage").join("dist").join("assets")),
        )
        .fallback_service(ServeFile::new(
            home.join("webpage").join("dist").join("index.html"),
        ));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
