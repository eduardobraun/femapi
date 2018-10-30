use rocket::Request;
use rocket_contrib::Json;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonErrorResponse {
    pub error: String,
}

#[catch(404)]
pub fn json_404(_req: &Request) -> Json<JsonErrorResponse> {
    // Json("{'error': 'Could not find the requested resource'}".to_string())
    Json(JsonErrorResponse {
        error: "Could not find the requested resource".to_string(),
    })
}
#[catch(500)]
pub fn json_500(_req: &Request) -> Json<JsonErrorResponse> {
    // Json("{'error': 'Server encountered an internal error'}".to_string())
    Json(JsonErrorResponse {
        error: "Server encountered an internal error".to_string(),
    })
}
#[catch(401)]
pub fn json_401(_req: &Request) -> Json<JsonErrorResponse> {
    // Json("{'error': 'User authentication is required'}".to_string())
    Json(JsonErrorResponse {
        error: "User authentication is required".to_string(),
    })
}
#[catch(403)]
pub fn json_403(_req: &Request) -> Json<JsonErrorResponse> {
    // Json("{'error': 'Request refused'}".to_string())
    Json(JsonErrorResponse {
        error: "Request refused".to_string(),
    })
}
