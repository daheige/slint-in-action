// 引入从构建脚本中使用slint-build crate生成的rust代码
slint::include_modules!();
fn main() {
    println!("Hello, world!");
    // HelloWorld 名字是hello.slint文件指定的
    HelloWorld::new().unwrap().run().unwrap();
}
