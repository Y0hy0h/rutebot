use std;
use std::env;
use std::fmt::Debug;

use futures::IntoFuture;

use rutebot::client::Rutebot;
use str;

pub fn create_client() -> Rutebot {
    let token = env::var_os("TEST_TOKEN").expect(
        "Please specify a token in the TEST_TOKEN environment variable.\n\
         That bot will be used for sending the test messages.",
    );
    let token = token.to_string_lossy();

    Rutebot::new(token)
}

pub fn get_chat_id() -> i64 {
    let chat_id = env::var_os("TEST_CHAT_ID").expect(
        "Please specify a supergroup's chat id in the TEST_CHAT_ID environment variable.\n\
         That group will be used to send test messages to. Ensure that the bot used for testing is an admin."
    );
    let chat_id = chat_id.to_string_lossy();

    str::parse(&chat_id).unwrap()
}

pub fn get_user_id() -> i64 {
    let user_id = env::var_os("TEST_USER_ID").expect(
        "Please specify a user id in the TEST_USER_ID environment variable.\n\
         That user will be used for test fetching of a profile picture. You can use a bot's id.",
    );
    let user_id = user_id.to_string_lossy();

    str::parse(&user_id).unwrap()
}

pub fn run_one<F>(f: F) -> F::Item
where
    F: IntoFuture,
    F::Future: Send + 'static,
    F::Item: Send + 'static,
    F::Error: Send + Debug + 'static,
{
    let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
    runtime.block_on(f.into_future()).unwrap()
}
