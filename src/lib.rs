/*!
# pngdefry
[![Latest Version](https://img.shields.io/crates/v/pngdefry.svg)](https://crates.io/crates/pngdefry)

ios pngdefry tool for rust

[![QQ群](https://img.shields.io/badge/QQ%E7%BE%A4-799168925-blue)](http://qm.qq.com/cgi-bin/qm/qr?_wv=1027&k=dLoye8pBcO60zGzqLjGO0l-GgMIaf6wQ&authKey=LfxBdZ5A%2F9eWJbKpzTcuWPjmQu5UdIJ3TVTpqRAQYkCID50WLkYoIXcGxGKzupG3&noverify=0&group_code=799168925)

# 使用指南
```
let result = pngdefry::convert("./iphone.png").unwrap();
let mut is_png = pngdefry::iphone_png("./iphone.png").unwrap();
println!("iphone is_png: {}", is_png);
is_png = pngdefry::iphone_png("./img.png").unwrap();
println!("niphone is_png: {}", is_png);
```
*/
pub mod pngdefry;
pub mod error;

pub use pngdefry::*;