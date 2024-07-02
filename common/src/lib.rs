pub mod model {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Login {
        pub email: String,
        pub pwd: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct LoginResponse {
        pub ok: bool,
    }

    pub mod routes {
        pub const LOGIN: &str = "/api/login";
    }
}
