use crate::models::chats::{Chat, NewChat, NewChatInput};
use crate::schema::chats::dsl::*;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub struct ChatsService<'a> {
    pool: &'a DbPool,
}

impl<'a> ChatsService<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        ChatsService { pool }
    }

    fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get DB connection")
    }

    pub fn create_chat(
        &self,
        user: AuthenticatedUser,
        input: NewChatInput,
        receiver_id_params: i32,
    ) -> Result<Chat, String> {
        let mut conn = self.get_connection();

        let new_chat = NewChat {
            sender_id: user.user_id,
            receiver_id: receiver_id_params,
            message: input.message,
        };

        diesel::insert_into(chats)
            .values(&new_chat)
            .get_result::<Chat>(&mut conn)
            .map_err(|e| e.to_string())
    }
}
