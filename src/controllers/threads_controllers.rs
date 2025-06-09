use crate::models::threads::{NewThread, Thread, UpdateThread};
use crate::output::thread_output::PaginatedThreadResponse;
use crate::services::threads_services::ThreadService;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;

pub struct ThreadController<'a> {
    service: ThreadService<'a>,
}

impl<'a> ThreadController<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        ThreadController {
            service: ThreadService::new(pool),
        }
    }

    pub fn create_thread(
        &self,
        new_thread: NewThread,
        auth_user: &AuthenticatedUser,
    ) -> Result<Thread, String> {
        self.service.create_thread(new_thread, auth_user)
    }

    pub fn get_paginated_threads(
        &self,
        limit: i64,
        offset: i64,
        page: u32,
    ) -> PaginatedThreadResponse {
        self.service
            .get_paginated_threads(offset, limit, page)
    }

    pub fn edit_thread(
        &self,
        thread_id: i32,
        thread: UpdateThread,
        auth_user: &AuthenticatedUser,
    ) -> Result<Thread, String> {
        self.service.update_thread(thread_id, thread, auth_user)
    }

    pub fn delete_thread(
        &self,
        thread_id: i32,
        auth_user: &AuthenticatedUser,
    ) -> Result<(), String> {
        self.service.delete_thread(thread_id, auth_user)
    }
}
