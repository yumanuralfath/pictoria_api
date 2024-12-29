use crate::models::threads::{NewThread, Thread, UpdateThread};
use crate::output::pagination_output::PaginationInfo;
use crate::output::thread_output::{PaginatedThreadResponse, ThreadOutput};
use crate::schema::threads::dsl::*;
use crate::utils::auth::AuthenticatedUser;
use crate::utils::db::DbPool;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

pub struct ThreadService<'a> {
    pool: &'a DbPool,
}

impl<'a> ThreadService<'a> {
    pub fn new(pool: &'a DbPool) -> Self {
        ThreadService { pool }
    }

    fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().expect("Failed to get DB connection")
    }

    pub fn create_thread(
        &self,
        mut new_thread: NewThread,
        auth_user: &AuthenticatedUser,
    ) -> Result<Thread, String> {
        let mut conn = self.get_connection();

        if new_thread.user_id.is_none() {
            new_thread.user_id = Some(auth_user.user_id);
        }

        diesel::insert_into(threads)
            .values(new_thread)
            .returning(Thread::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Error creating thread: {}", e))
    }

    pub fn get_threads(&self, offset: i64, limit: i64) -> Vec<Thread> {
        let mut conn = self.get_connection();
        threads
            .limit(limit)
            .offset(offset)
            .select(Thread::as_select())
            .load::<Thread>(&mut conn)
            .unwrap_or_default()
    }

    //UWU gak jadi di pake wkkwkwwk
    // pub fn get_random_threads(&self, limit: i64) -> Vec<Thread> {
    //     let mut conn = self.get_connection();

    //     // Langkah 1: Ambil ID secara acak dengan limit
    //     let random_ids: Vec<i32> = threads
    //         .select(id) // Mengakses kolom `id` dari `threads`
    //         .order_by(sql::<diesel::sql_types::Text>("RANDOM()")) // Acak ID
    //         .limit(limit)
    //         .load::<i32>(&mut conn)
    //         .unwrap_or_default();

    //     if random_ids.is_empty() {
    //         return vec![];
    //     }

    //     // Langkah 2: Ambil thread berdasarkan ID acak
    //     threads
    //         .filter(id.eq_any(random_ids)) // Filter berdasarkan ID yang dipilih
    //         .load::<Thread>(&mut conn)
    //         .unwrap_or_default()
    // }

    pub fn count_threads(&self) -> i64 {
        let mut conn = self.get_connection();
        threads.count().get_result(&mut conn).unwrap_or(0)
    }

    pub fn get_paginated_threads(
        &self,
        offset: i64,
        limit: i64,
        page: u32,
        _auth: &AuthenticatedUser,
    ) -> PaginatedThreadResponse {
        let thread_list = self.get_threads(offset, limit);

        let modified_result = thread_list
            .into_iter()
            .map(ThreadOutput::from_thread)
            .collect();

        PaginatedThreadResponse {
            threads: modified_result,
            pagination: PaginationInfo {
                current_page: page,
                limit: limit as u32,
                total_items: self.count_threads(),
            },
        }
    }

    fn get_thread_by_id(&self, thread_id: i32) -> Option<Thread> {
        let mut conn = self.get_connection();
        threads
            .find(thread_id)
            .select(Thread::as_select())
            .first(&mut conn)
            .ok()
    }

    pub fn update_thread(
        &self,
        thread_id: i32,
        update_thread: UpdateThread,
        auth_user: &AuthenticatedUser,
    ) -> Result<Thread, String> {
        let mut conn = self.get_connection();

        let thread = self
            .get_thread_by_id(thread_id)
            .ok_or_else(|| "Thread not found".to_string())?;

        if thread.user_id != auth_user.user_id {
            return Err("Unauthorized to update this thread".to_string());
        }

        diesel::update(threads.find(thread_id))
            .set(update_thread)
            .returning(Thread::as_returning())
            .get_result(&mut conn)
            .map_err(|e| format!("Error updating thread: {}", e))
    }

    pub fn delete_thread(
        &self,
        thread_id: i32,
        auth_user: &AuthenticatedUser,
    ) -> Result<(), String> {
        let mut conn = self.get_connection();

        let thread = self
            .get_thread_by_id(thread_id)
            .ok_or_else(|| "Thread not found".to_string())?;

        if thread.user_id != auth_user.user_id {
            return Err("Unauthorized to delete this thread".to_string());
        }

        diesel::delete(threads.find(thread_id))
            .execute(&mut conn)
            .map_err(|e| format!("Error deleting thread: {}", e))
            .map(|_| ())
    }
}
