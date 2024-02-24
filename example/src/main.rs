fn main() {
    // let result = pngdefry::convert("./iphone.png").unwrap();
    let mut is_png = pngdefry::iphone_png("./iphone.png").unwrap();
    println!("iphone is_png: {}", is_png);
    is_png = pngdefry::iphone_png("./img.png").unwrap();
    println!("niphone is_png: {}", is_png);
}
