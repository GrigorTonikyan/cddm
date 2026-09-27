#![forbid(unsafe_code)]

use super::types::{TaskRecord, TaskStatus};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use tokio::sync::{Mutex, RwLock};
use tokio::task::AbortHandle;

static TASK_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Thread-safe manager for MCP 2026-07-28 background tasks.
pub struct TaskManager {
    tasks: Arc<RwLock<HashMap<String, TaskRecord>>>,
    abort_handles: Arc<Mutex<HashMap<String, AbortHandle>>>,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
            abort_handles: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Returns the global singleton task manager.
    pub fn global() -> &'static TaskManager {
        static INSTANCE: OnceLock<TaskManager> = OnceLock::new();
        INSTANCE.get_or_init(TaskManager::new)
    }

    /// Lists all tasks known to the manager.
    pub async fn list_tasks(&self) -> Vec<TaskRecord> {
        let store = self.tasks.read().await;
        let mut list: Vec<TaskRecord> = store.values().cloned().collect();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        list
    }

    /// Gets a task by ID.
    pub async fn get_task(&self, task_id: &str) -> Option<TaskRecord> {
        let store = self.tasks.read().await;
        store.get(task_id).cloned()
    }

    /// Cancels a running or queued task by ID.
    pub async fn cancel_task(&self, task_id: &str) -> Result<TaskRecord, String> {
        // Abort background worker if running
        {
            let mut handles = self.abort_handles.lock().await;
            if let Some(handle) = handles.remove(task_id) {
                handle.abort();
            }
        }

        let mut store = self.tasks.write().await;
        let task = store
            .get_mut(task_id)
            .ok_or_else(|| format!("Task '{}' not found", task_id))?;

        if task.status != TaskStatus::Completed && task.status != TaskStatus::Failed {
            task.status = TaskStatus::Cancelled;
            task.completed_at = Some(Utc::now().to_rfc3339());
            task.updated_at = Utc::now().to_rfc3339();
        }

        Ok(task.clone())
    }

    /// Spawns an asynchronous background task invoking the target tool.
    pub async fn spawn_task(
        &self,
        name: String,
        arguments: Option<serde_json::Value>,
    ) -> TaskRecord {
        let count = TASK_COUNTER.fetch_add(1, Ordering::SeqCst);
        let task_id = format!("task-{}-{}", Utc::now().timestamp_millis(), count);
        let now = Utc::now().to_rfc3339();

        let initial_record = TaskRecord {
            task_id: task_id.clone(),
            name: name.clone(),
            status: TaskStatus::Running,
            created_at: now.clone(),
            updated_at: now,
            completed_at: None,
            result: None,
            error: None,
            progress: Some(0.0),
        };

        {
            let mut store = self.tasks.write().await;
            store.insert(task_id.clone(), initial_record.clone());
        }

        let task_store = self.tasks.clone();
        let abort_store = self.abort_handles.clone();
        let task_id_clone = task_id.clone();
        let tool_name = name;
        let tool_args = arguments;

        let join_handle = tokio::spawn(async move {
            let call_params = serde_json::json!({
                "name": tool_name,
                "arguments": tool_args.unwrap_or_else(|| serde_json::json!({}))
            });

            let resp = crate::tools::dispatch_tool_call(None, Some(&call_params)).await;

            let mut store = task_store.write().await;
            if let Some(task) = store.get_mut(&task_id_clone)
                && task.status != TaskStatus::Cancelled
            {
                if let Some(err) = resp.error {
                    task.status = TaskStatus::Failed;
                    task.error = Some(err.to_string());
                } else {
                    task.status = TaskStatus::Completed;
                    task.result = resp.result;
                }
                task.completed_at = Some(Utc::now().to_rfc3339());
                task.updated_at = Utc::now().to_rfc3339();
                task.progress = Some(1.0);
            }

            let mut handles = abort_store.lock().await;
            handles.remove(&task_id_clone);
        });

        {
            let mut handles = self.abort_handles.lock().await;
            handles.insert(task_id, join_handle.abort_handle());
        }

        initial_record
    }
}
