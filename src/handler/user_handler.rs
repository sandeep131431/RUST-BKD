
use actix_web::{web, HttpResponse};
use mongodb::bson::doc;
use bcrypt::{hash, verify, DEFAULT_COST};
use validator::Validate;

use crate::modals::{User, LoginRequest};
use crate::db::connect_db;

// Helper function for validation errors
fn validation_error_response(errors: &validator::ValidationErrors) -> HttpResponse {
    let error_messages: Vec<String> = errors
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            format!(
                "{}: {}",
                field,
                errors
                    .iter()
                    .map(|e| e.message.as_ref().map(|s| s.to_string()).unwrap_or_default())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        })
        .collect();
    
    HttpResponse::BadRequest().body(format!("Validation errors: {}", error_messages.join("; ")))
}

// Create user with validation and password hashing
pub async fn create_user(user_data: web::Json<User>) -> HttpResponse {
    println!("📨 Received user creation request");
    
    // Step 1: Validate input data
    if let Err(validation_errors) = user_data.validate() {
        println!("❌ Validation failed: {:?}", validation_errors);
        return validation_error_response(&validation_errors);
    }
    
    let collection = connect_db().await;
    
    // Step 2: Check if user already exists
    match collection
        .count_documents(
            doc! { "email": &user_data.email },
            None
        )
        .await
    {
        Ok(count) if count > 0 => {
            println!("❌ User with email {} already exists", user_data.email);
            return HttpResponse::BadRequest().body("User with this email already exists");
        }
        Err(e) => {
            println!("❌ Database error while checking existing user: {}", e);
            return HttpResponse::InternalServerError().body("Database error");
        }
        _ => {} // User doesn't exist, continue
    }
    
    // Step 3: Hash the password
    let hashed_password = match hash(&user_data.password, DEFAULT_COST) {
        Ok(hashed) => {
            println!("✅ Password hashed successfully");
            hashed
        }
        Err(e) => {
            println!("❌ Password hashing failed: {}", e);
            return HttpResponse::InternalServerError().body("Password processing error");
        }
    };
    
    // Step 4: Create user with hashed password
    let user_to_insert = User {
        name: user_data.name.clone(),
        email: user_data.email.clone(),
        password: hashed_password, // Store hashed password instead of plain text
    };
    
    // Step 5: Insert into database
    match collection.insert_one(user_to_insert, None).await {
        Ok(result) => {
            println!("✅ User saved successfully with ID: {:?}", result.inserted_id);
            HttpResponse::Ok().body(format!("User created successfully! ID: {:?}", result.inserted_id))
        }
        Err(e) => {
            println!("❌ Database insertion error: {}", e);
            HttpResponse::InternalServerError().body(format!("Error saving user: {}", e))
        }
    }
}

// Login handler with validation and password verification
pub async fn login_user(login_data: web::Json<LoginRequest>) -> HttpResponse {
    println!("🔐 Received login request for: {}", login_data.email);
    
    // Step 1: Validate input data
    if let Err(validation_errors) = login_data.validate() {
        println!("❌ Login validation failed: {:?}", validation_errors);
        return validation_error_response(&validation_errors);
    }
    
    let collection = connect_db().await;
    
    // Step 2: Find user by email
    let user_doc = match collection
        .find_one(
            doc! { "email": &login_data.email },
            None
        )
        .await
    {
        Ok(Some(user)) => {
            println!("✅ User found in database");
            user
        }
        Ok(None) => {
            println!("❌ User not found with email: {}", login_data.email);
            return HttpResponse::Unauthorized().body("Invalid email or password");
        }
        Err(e) => {
            println!("❌ Database error during login: {}", e);
            return HttpResponse::InternalServerError().body("Database error");
        }
    };
    
    // Step 3: Verify password
    match verify(&login_data.password, &user_doc.password) {
        Ok(true) => {
            println!("✅ Password verified successfully for: {}", login_data.email);
            HttpResponse::Ok().body(format!("Login successful! Welcome back, {}", user_doc.name))
        }
        Ok(false) => {
            println!("❌ Invalid password for: {}", login_data.email);
            HttpResponse::Unauthorized().body("Invalid email or password")
        }
        Err(e) => {
            println!("❌ Password verification error: {}", e);
            HttpResponse::InternalServerError().body("Password verification error")
        }
    }
}

// Get users total count
pub async fn get_users() -> HttpResponse {
    println!("📊 Getting users count");
    
    let collection = connect_db().await;
    
    match collection.count_documents(None, None).await {
        Ok(count) => {
            println!("✅ Total users: {}", count);
            HttpResponse::Ok().body(format!("Total users: {}", count))
        }
        Err(e) => {
            println!("❌ Error getting users count: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

// Root endpoint
pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body(
        "🚀 Server running!\n\
         POST /user - Create user (with validation & password hashing)\n\
         POST /login - User login\n\
         GET /users - Get users count"
    )
}