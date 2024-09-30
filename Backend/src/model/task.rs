use crate::context::Context;
use crate::model::{Error, ModelManager, Result};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region:    --- Task Types
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}
// endregion: --- Task Types

// region:    --- Task Model Controller
pub struct TaskBmc;

// impl DbBmc for TaskBmc {
//     const TABLE: &'static str = "task";
// }

impl TaskBmc {
    pub async fn create(
        context: &Context,
        model_manager: &ModelManager,
        task_created: TaskForCreate,
    ) -> Result<i64> {
        let db = model_manager.db();

        let (id,) =
            sqlx::query_as::<_, (i64,)>("INSERT INTO task (title) values ($1) returning id")
                .bind(task_created.title)
                .fetch_one(db)
                .await?;

        Ok(id)
    }

    pub async fn get(context: &Context, model_manager: &ModelManager, id: i64) -> Result<Task> {
        let db = model_manager.db();

        let task = sqlx::query_as("SELECT * from task where id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound { entity: "task", id })?;

        Ok(task)
    }

    pub async fn list(context: &Context, model_manager: &ModelManager) -> Result<Vec<Task>> {
        let db = model_manager.db();

        let tasks = sqlx::query_as("SELECT * from task ORDER BY id")
            .fetch_all(db)
            .await?;

        Ok(tasks)
    }

    // pub async fn update(
    //     ctx: &Context,
    //     mm: &ModelManager,
    //     id: i64,
    //     task_updated: TaskForUpdate,
    // ) -> Result<()> {
    //     base::update::<Self, _>(ctx, mm, id, task_updated).await
    // }

    pub async fn delete(context: &Context, model_manager: &ModelManager, id: i64) -> Result<()> {
        let db = model_manager.db();

        let count = sqlx::query("DELETE from task where id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();

        if count == 0 {
            return Err(Error::EntityNotFound { entity: "task", id });
        }

        Ok(())
    }
}
// region:    --- Task Model Controller

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use anyhow::Result;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let function_title = "test_create_ok title";

        // -- Exec
        let task_created = TaskForCreate {
            title: function_title.to_string(),
        };
        let id = TaskBmc::create(&context, &model_manager, task_created).await?;

        // -- Check
        let task = TaskBmc::get(&context, &model_manager, id).await?;
        assert_eq!(function_title, task.title);

        // Clean
        TaskBmc::delete(&context, &model_manager, id).await?;

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let function_id = 100;

        // -- Exec
        let res = TaskBmc::get(&context, &model_manager, function_id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let function_titles = &[
            "test_list_ok-task 01",
            "test_list_ok-task 02",
            "test_list_ok-task 03",
        ];

        _dev_utils::seed_tasks(&context, &model_manager, function_titles).await?;

        // -- Exec
        let tasks = TaskBmc::list(&context, &model_manager).await?;

        // -- Check
        let tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|t| t.title.starts_with("test_list_ok-task"))
            .collect();
        assert_eq!(tasks.len(), 3, "number of seeded tasks.");

        // -- Clean
        for task in tasks.iter() {
            TaskBmc::delete(&context, &model_manager, task.id).await?;
        }

        Ok(())
    }

    // #[serial]
    // #[tokio::test]
    // async fn test_update_ok() -> Result<()> {
    // 	// -- Setup & Fixtures
    // 	let mm = _dev_utils::init_test().await;
    // 	let ctx = Ctx::root_ctx();
    // 	let fx_title = "test_update_ok - task 01";
    // 	let fx_title_new = "test_update_ok - task 01 - new";
    // 	let fx_task = _dev_utils::seed_tasks(&ctx, &mm, &[fx_title])
    // 		.await?
    // 		.remove(0);

    // 	// -- Exec
    // 	TaskBmc::update(
    // 		&ctx,
    // 		&mm,
    // 		fx_task.id,
    // 		TaskForUpdate {
    // 			title: Some(fx_title_new.to_string()),
    // 		},
    // 	)
    // 	.await?;

    // 	// -- Check
    // 	let task = TaskBmc::get(&ctx, &mm, fx_task.id).await?;
    // 	assert_eq!(task.title, fx_title_new);

    // 	Ok(())
    // }

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        // -- Setup & Fixtures
        let model_manager = _dev_utils::init_test().await;
        let context = Context::root_ctx();
        let function_id = 100;

        // -- Exec
        let res = TaskBmc::delete(&context, &model_manager, function_id).await;

        // -- Check
        assert!(
            matches!(
                res,
                Err(Error::EntityNotFound {
                    entity: "task",
                    id: 100
                })
            ),
            "EntityNotFound not matching"
        );

        Ok(())
    }
}
// endregion: --- Tests
