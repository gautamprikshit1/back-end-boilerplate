use juniper::{GraphQLInputObject, GraphQLObject};
use sqlx::{postgres::PgQueryResult, query_as_unchecked, query_unchecked};

use crate::Pool;

#[derive(Debug, Clone, GraphQLObject)]
pub struct PollQuestion {
    pub id: String,
    pub question: String,
}

#[derive(Debug, GraphQLObject, Clone)]
pub struct PollOptions {
    id: String,
    pub question_id: String,
    pub text: String,
    pub votes: i32,
}

#[derive(Debug, GraphQLInputObject)]
pub struct InputPollQuestion {
    pub question: String,
}

#[derive(Debug, GraphQLInputObject)]
pub struct InputPollOption {
    pub question_id: String,
    pub text: String,
}

pub async fn get_all_questions(connection: &Pool) -> anyhow::Result<Vec<PollQuestion>> {
    query_as_unchecked!(PollQuestion, r#"select * from questions"#)
        .fetch_all(connection)
        .await
        .map_err(|e| e.into())
}

pub async fn get_poll_question(connection: &Pool, id: String) -> anyhow::Result<PollQuestion> {
    query_as_unchecked!(PollQuestion, r#"select * from questions where id = $1"#, id)
        .fetch_one(connection)
        .await
        .map_err(|e| e.into())
}

pub async fn get_options_by_question(
    connection: &Pool,
    question_id: String,
) -> anyhow::Result<Vec<PollOptions>> {
    query_as_unchecked!(
        PollOptions,
        r#"select * from options where question_id = $1"#,
        question_id
    )
    .fetch_all(connection)
    .await
    .map_err(|e| e.into())
}

pub async fn get_options(
    connection: &Pool,
    question_id: String,
) -> anyhow::Result<Vec<PollOptions>> {
    query_as_unchecked!(
        PollOptions,
        r#"select * from options where question_id = $1"#,
        question_id
    )
    .fetch_all(connection)
    .await
    .map_err(|e| e.into())
}

pub async fn get_votes(connection: &Pool, option_id: String) -> anyhow::Result<i32> {
    let query = query_unchecked!(r#"select votes from options where id = $1"#, option_id)
        .fetch_one(connection)
        .await
        .map_err(|e| <sqlx::Error as Into<anyhow::Error>>::into(e))
        .unwrap()
        .votes
        .unwrap();
    return Ok(query);
}

pub async fn update_option(connection: &Pool, id: String) -> anyhow::Result<PgQueryResult> {
    query_unchecked!(
        r#"update options set votes = $1 where id = $2"#,
        get_votes(connection, id.clone()).await.unwrap() + 1,
        id
    )
    .execute(connection)
    .await
    .map_err(|e| e.into())
}

pub async fn create_question(
    connection: &Pool,
    id: String,
    question: String,
) -> anyhow::Result<PgQueryResult> {
    query_unchecked!(
        r#"
                     insert into questions (id, question)
                     VALUES ($1, $2)
                     "#,
        id,
        question
    )
    .execute(connection)
    .await
    .map_err(|e| e.into())
}

pub async fn create_option(
    connection: &Pool,
    id: String,
    question_id: String,
    text: String,
) -> anyhow::Result<PgQueryResult> {
    query_unchecked!(
        r#"
                     insert into options (id, question_id, text, votes)
                     VALUES ($1, $2, $3, $4)
                     "#,
        id,
        question_id,
        text,
        0
    )
    .execute(connection)
    .await
    .map_err(|e| e.into())
}
