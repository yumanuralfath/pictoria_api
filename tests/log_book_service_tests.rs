// // Deklarasikan modul common untuk mengakses helper
// mod common;

// use common::{TestLogBookGuard, TestUserGuard};
// use yumana_api::{
//     models::log_books::{NewLogBook, UpdateLogBook},
//     services::log_book_service::LogBookService,
//     utils::{auth::AuthenticatedUser, db::DbPool},
// };

// /// Helper untuk setup environment tes.
// /// Membuat DbPool, satu user admin, dan satu user biasa.
// fn setup() -> (DbPool, TestUserGuard, TestUserGuard) {
//     let client = common::setup_client();
//     let pool = client
//         .rocket()
//         .state::<DbPool>()
//         .expect("Gagal mengambil pool")
//         .clone();

//     // Buat admin dan user biasa untuk menguji logika otorisasi.
//     // Asumsi: TestUserGuard.new() membuat user non-admin secara default.
//     // Untuk membuat admin, Anda mungkin perlu memodifikasi TestUserGuard atau membuat helper baru.
//     // Untuk kesederhanaan, kita akan membuat AuthenticatedUser secara manual di setiap tes.
//     let admin_guard = common::TestUserGuard::new(&pool, "adminuser", "admin@test.com");
//     let user_guard = common::TestUserGuard::new(&pool, "regularuser", "user@test.com");

//     (pool, admin_guard, user_guard)
// }

// #[test]
// fn test_create_log_book_permissions() {
//     let (pool, admin_guard, user_guard) = setup();
//     let service = LogBookService::new(&pool);

//     // Buat objek AuthenticatedUser secara manual untuk mengontrol status admin
//     let admin_auth = AuthenticatedUser {
//         user_id: admin_guard.user_id,
//         is_admin: true,
//         username: "adminuser".into(),
//     };
//     let user_auth = AuthenticatedUser {
//         user_id: user_guard.user_id,
//         is_admin: false,
//         username: "regularuser".into(),
//     };

//     // Kasus Sukses: Admin bisa membuat log book
//     let new_log_data = NewLogBook {
//         user_id: 0,                                      // Akan ditimpa oleh service
//         date: chrono::Local::now().naive_local().date(), // Akan ditimpa oleh service
//         title: "Log Book oleh Admin".to_string(),
//         body: "Isi log...".to_string(),
//     };
//     let result = service.create_log_book(admin_auth, new_log_data);
//     assert!(result.is_ok());
//     let created_log = result.unwrap();
//     assert_eq!(created_log.title, "Log Book oleh Admin");
//     // Pastikan log book yang dibuat dihapus setelah tes
//     let _log_guard = TestLogBookGuard {
//         log_book_id: created_log.id,
//         pool,
//     };

//     // Kasus Gagal: User biasa tidak bisa membuat log book
//     let new_log_data_fail = NewLogBook {
//         user_id: 0,
//         date: chrono::Local::now().naive_local().date(),
//         title: "Coba-coba".into(),
//         body: "".into(),
//     };
//     let result_fail = service.create_log_book(user_auth, new_log_data_fail);
//     assert!(result_fail.is_err());
//     assert_eq!(
//         result_fail.err().unwrap(),
//         "User does not have admin privileges"
//     );
// }

// #[test]
// fn test_get_paginated_log_books_permissions() {
//     let (pool, admin_guard, user_guard) = setup();
//     let service = LogBookService::new(&pool);

//     // Buat data tes (seed data)
//     let _admin_log = TestLogBookGuard::new(&pool, admin_guard.user_id, "Log Admin");
//     let _user_log1 = TestLogBookGuard::new(&pool, user_guard.user_id, "Log User 1");
//     let _user_log2 = TestLogBookGuard::new(&pool, user_guard.user_id, "Log User 2");

//     // Kasus 1: User biasa hanya melihat log book miliknya
//     let user_auth = AuthenticatedUser {
//         user_id: user_guard.user_id,
//         is_admin: false,
//         username: "regularuser".into(),
//     };
//     let user_view = service.get_paginated_log_books(user_auth, 0, 10, 1);
//     assert_eq!(user_view.pagination.total_items, 2);
//     assert_eq!(user_view.log_books.len(), 2);
//     // Pastikan semua log yang didapat adalah milik user tersebut
//     assert!(user_view
//         .log_books
//         .iter()
//         .all(|log| log.user_id == user_guard.user_id));

//     // Kasus 2: Admin melihat semua log book
//     let admin_auth = AuthenticatedUser {
//         user_id: admin_guard.user_id,
//         is_admin: true,
//         username: "adminuser".into(),
//     };
//     let admin_view = service.get_paginated_log_books(admin_auth, 0, 10, 1);
//     assert_eq!(admin_view.pagination.total_items, 3);
//     assert_eq!(admin_view.log_books.len(), 3);
// }

// #[test]
// fn test_update_and_delete_log_book_permissions() {
//     let (pool, admin_guard, user_guard) = setup();
//     let service = LogBookService::new(&pool);
//     let log_to_modify = TestLogBookGuard::new(&pool, admin_guard.user_id, "Log Awal");

//     let admin_auth = AuthenticatedUser {
//         user_id: admin_guard.user_id,
//         is_admin: true,
//         username: "adminuser".into(),
//     };
//     let user_auth = AuthenticatedUser {
//         user_id: user_guard.user_id,
//         is_admin: false,
//         username: "regularuser".into(),
//     };

//     let update_payload = UpdateLogBook {
//         title: Some("Judul Diperbarui".to_string()),
//         body: None,
//     };

//     // Gagal: User biasa mencoba update
//     let update_fail = service.update_log_book(
//         user_auth.clone(),
//         log_to_modify.log_book_id,
//         update_payload.clone(),
//     );
//     assert!(update_fail.is_err());

//     // Sukses: Admin berhasil update
//     let update_success = service.update_log_book(
//         admin_auth.clone(),
//         log_to_modify.log_book_id,
//         update_payload,
//     );
//     assert!(update_success.is_ok());
//     assert_eq!(update_success.unwrap().title, "Judul Diperbarui");

//     // Gagal: User biasa mencoba hapus
//     let delete_fail = service.delete_log_book(user_auth, log_to_modify.log_book_id);
//     assert!(delete_fail.is_err());

//     // Sukses: Admin berhasil hapus
//     let delete_success = service.delete_log_book(admin_auth.clone(), log_to_modify.log_book_id);
//     assert!(delete_success.is_ok());
//     assert_eq!(delete_success.unwrap(), 1); // 1 baris terhapus

//     // Verifikasi: Log book sudah tidak ada
//     let find_deleted = service.get_log_book_by_id(admin_auth, log_to_modify.log_book_id);
//     assert!(find_deleted.is_err());
// }
