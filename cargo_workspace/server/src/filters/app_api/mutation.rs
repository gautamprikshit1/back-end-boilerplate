use super::context::Context;
use db_client::model::{
    create_option, create_question, get_options, get_poll_question, get_votes, update_option,
    InputPollOption, InputPollQuestion, PollOptions, PollQuestion,
};
use juniper::{graphql_object, FieldResult};

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn create_question(ctx: &Context, input: InputPollQuestion) -> FieldResult<PollQuestion> {
        let id = nanoid::nanoid!();
        create_question(&ctx.pl, id.clone(), input.question)
            .await
            .unwrap();
        Ok(get_poll_question(&ctx.pl, id).await.unwrap())
    }

    async fn add_option(ctx: &Context, input: InputPollOption) -> FieldResult<Vec<PollOptions>> {
        let option_id = nanoid::nanoid!();
        create_option(&ctx.pl, option_id, input.question_id.clone(), input.text)
            .await
            .unwrap();
        Ok(get_options(&ctx.pl, input.question_id).await.unwrap())
    }

    async fn upvote_option(ctx: &Context, option_id: String) -> FieldResult<i32> {
        update_option(&ctx.pl, option_id.clone()).await.unwrap();
        Ok(get_votes(&ctx.pl, option_id).await.unwrap())
    }
}
