#[derive(Debug, Serialize, Deserialize)]
pub struct TokenFiles {
    #[serde(default = "_client")]
    pub client: String,
    #[serde(default = "_server")]
    pub server: String,
    #[serde(default = "_admin")]
    pub admin: String,
}

fn _client() -> String {
    String::from("./client_tokens.txt")
}

fn _server() -> String {
    String::from("./server_tokens.txt")
}

fn _admin() -> String {
    String::from("./admin_tokens.txt")
}

impl Default for TokenFiles {
    fn default() -> Self {
        Self {
            client: _client(),
            server: _server(),
            admin: _admin(),
        }
    }
}
