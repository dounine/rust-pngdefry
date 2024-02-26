# pngdefry
[![Latest Version](https://img.shields.io/crates/v/pngdefry.svg)](https://crates.io/crates/pngdefry)

ios pngdefry tool for rust

[![QQ群](https://img.shields.io/badge/QQ%E7%BE%A4-799168925-blue)](http://qm.qq.com/cgi-bin/qm/qr?_wv=1027&k=dLoye8pBcO60zGzqLjGO0l-GgMIaf6wQ&authKey=LfxBdZ5A%2F9eWJbKpzTcuWPjmQu5UdIJ3TVTpqRAQYkCID50WLkYoIXcGxGKzupG3&noverify=0&group_code=799168925)

# 使用指南
判断是不是iphone png
```rust
let result = pngdefry::iphone_png("./iphone.png").unwrap();
if(result){
    println!("iphone png");
}else{
    println!("not iphone png");
}
```
还原iphone png
```
pngdefry::convert("./iphone.png","./big.png").unwrap();
```