use diesel::{
    ExpressionMethods, Insertable, QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::Post;
use crate::infra::db::errors::{db_internal_error, diesel_error, DbError};
use crate::infra::db::schema::posts;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostDb {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPostDb {
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub async fn insert(
    pool: &deadpool_diesel::postgres::Pool,
    new_post: NewPostDb,
) -> Result<Post, DbError> {
    let conn = pool.get().await.map_err(db_internal_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(posts::table)
                .values(new_post)
                .returning(PostDb::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(db_internal_error)?
        .map_err(diesel_error)?;

    Ok(adapt_post_db_to_post(res))
}

pub async fn get(pool: &deadpool_diesel::postgres::Pool, id: Uuid) -> Result<Post, DbError> {
    let conn = pool.get().await.map_err(db_internal_error)?;
    let res = conn
        .interact(move |conn| {
            posts::table
                .filter(posts::id.eq(id))
                .select(PostDb::as_select())
                .get_result(conn)
        })
        .await
        .map_err(db_internal_error)?
        .map_err(diesel_error)?;

    Ok(adapt_post_db_to_post(res))
}

pub async fn get_all(pool: &deadpool_diesel::postgres::Pool) -> Result<Vec<Post>, DbError> {
    let conn = pool.get().await.map_err(db_internal_error)?;
    let res = conn
        .interact(move |conn| posts::table.select(PostDb::as_select()).get_results(conn))
        .await
        .map_err(db_internal_error)?
        .map_err(diesel_error)?;

    let posts: Vec<Post> = res
        .into_iter()
        .map(|post_db| adapt_post_db_to_post(post_db))
        .collect();

    Ok(posts)
}

fn adapt_post_db_to_post(post_db: PostDb) -> Post {
    Post {
        id: post_db.id,
        title: post_db.title,
        body: post_db.body,
        published: post_db.published,
    }
}
