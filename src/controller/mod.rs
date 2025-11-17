pub mod routes_controller; // Add this line to declare the routes_controller module
mod auth_controller;
pub use auth_controller::{AuthController,LoginForm};
mod register_controller;
pub use register_controller::RegisterController;