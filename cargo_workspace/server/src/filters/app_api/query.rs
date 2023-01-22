use super::context::Context;
use db_client::model::{
    get_all_questions, get_options_by_question, get_poll_question, PollOptions, PollQuestion,
};
use juniper::{graphql_object, FieldResult};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    pub async fn poll_question(ctx: &Context, id: String) -> FieldResult<PollQuestion> {
        Ok(get_poll_question(&ctx.pl, id).await?)
    }

    pub async fn get_all_questions(ctx: &Context) -> FieldResult<Vec<PollQuestion>> {
        Ok(get_all_questions(&ctx.pl).await?)
    }

    pub async fn get_options_for_question(
        ctx: &Context,
        question_id: String,
    ) -> FieldResult<Vec<PollOptions>> {
        Ok(get_options_by_question(&ctx.pl, question_id).await?)
    }
}
