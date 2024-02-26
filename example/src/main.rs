fn main() {
    pngdefry::convert("./iphone.png","./iphone_pngdefry.png").unwrap();
    //remove output_path
    // std::fs::remove_file("./iphone_pngdefry.png").unwrap_or_default();
    let mut is_png = pngdefry::iphone_png("./iphone.png").unwrap();
    println!("iphone is_png: {}", is_png);
    is_png = pngdefry::iphone_png("./img.png").unwrap();
    println!("niphone is_png: {}", is_png);
}
