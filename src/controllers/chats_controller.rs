use crate::models::chats::{Chat, NewChatInput};
use crate::services::chats_service::ChatsService;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;

pub struct ChatController<'a> {
    service: ChatsService<'a>,
}

impl<'a> ChatController<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        ChatController {
            service: ChatsService::new(pool),
        }
    }

    pub fn create_chat(
        &self,
        user: AuthenticatedUser,
        input: NewChatInput,
        receiver_id_params: i32,
    ) -> Result<Chat, String> {
        self.service.create_chat(user, input, receiver_id_params)
    }
}
