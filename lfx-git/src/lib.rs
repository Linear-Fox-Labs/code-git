// Copyright © 2023 Linear Fox. This file is under GPLv3 License. 
 
extern crate git2;
  
// Import module
pub mod error;
pub mod signature;
 
// Export functions and types
pub use error::ErrorUtil;
pub use signature::*;