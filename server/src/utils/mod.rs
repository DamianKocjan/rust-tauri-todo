pub mod pagination {
    #[derive(serde::Deserialize)]
    pub struct PaginationParams {
        pub page: Option<i64>,
        pub page_size: Option<i64>,
    }

    impl PaginationParams {
        pub const MAX_PAGE_SIZE: u16 = 100;
    }
}
