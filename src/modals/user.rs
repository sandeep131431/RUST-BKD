use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct User {
    #[validate(length(min = 2, max = 50, message = "Name must be between 2 and 50 characters"))]
    pub name: String,
    
    #[validate(email(message = "Please provide a valid email address"))]
    pub email: String,
    
    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
}

// Login ke liye alag struct
#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Please provide a valid email address"))]
    pub email: String,
    
    pub password: String,
}