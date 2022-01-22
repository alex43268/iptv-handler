use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow};

use crate::{Connection, CRUD};

#[derive(Debug, Clone)]
pub struct ExtInfRequest {
    pub name: String,
    pub url: String,
    pub m3u_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]

pub struct ExtInfModel {
    id: u64,
    name: String,
    url: String,
    m3u_id: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ExtInf {}

#[async_trait::async_trait]
impl CRUD<ExtInfModel, ExtInfRequest> for ExtInf {
    async fn get(&self, tx: &mut Connection, id: u64) -> Result<ExtInfModel, Error> {
        let res = sqlx::query_as!(ExtInfModel, "select * from extinf where id = ?", id)
            .fetch_one(tx)
            .await;

        res
    }

    async fn insert(&self, tx: &mut Connection, extinf: ExtInfRequest) -> Result<u64, Error> {
        let res = sqlx::query_as!(
            ExtInfModel,
            r#"insert into extinf (name, url, m3u_id) values (?, ?, ?)"#,
            extinf.name,
            extinf.url,
            extinf.m3u_id,
        )
        .execute(tx)
        .await?
        .last_insert_id();

        Ok(res)
    }

    async fn delete(&self, tx: &mut Connection, id: u64) -> Result<u64, Error> {
        let res = sqlx::query_as!(u64, r#"delete from extinf where id = ?"#, id)
            .execute(tx)
            .await?
            .rows_affected();

        Ok(res)
    }
}
