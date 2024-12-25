pub fn paginate(page: Option<u32>, limit: Option<u32>) -> (i64, i64) {
    let page = page.unwrap_or(1) as i64;
    let limit = limit.unwrap_or(10) as i64;

    let offset = (page - 1) * limit;
    (offset, limit)
}
