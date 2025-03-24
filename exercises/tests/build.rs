//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 为 tests7.rs 设置当前时间戳环境变量
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    
    // 为 tests8.rs 启用 "pass" 特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
