#[cfg(not(target_env = "msvc"))]
use mimalloc::MiMalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod config;
pub mod errors;
pub mod models;
pub mod repositories;
pub mod services;
pub mod routes;
