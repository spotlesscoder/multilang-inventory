use crate::infra::db::db;
use mongodb::bson::doc;
use nanoid::nanoid;

#[allow(dead_code)]
pub(crate) async fn test_db() -> Result<mongodb::Database, anyhow::Error> {
    let db_name = format!("test_db_{}", nanoid!());
    // run command setParameter notablescan
    let client = db("admin").await?;
    client
        .run_command(doc! { "setParameter": 1, "notablescan": 1 })
        .await
        .map_err(|e| anyhow::Error::from(e))?;
    db(&db_name).await.map_err(|e| anyhow::Error::from(e))
}
