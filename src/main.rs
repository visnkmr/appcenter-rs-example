use app_center::AppCenter;
fn main() {
    app_center::start!("<appsecret>");
    println!("Hello, world!");
    panic!("test");
}
