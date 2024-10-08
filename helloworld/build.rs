// 构建脚本，在程序运行之前执行一系列的操作
// 这里使用slint_build工具从hello.slint界面，编译生成对应的rust代码
fn main() {
    slint_build::compile("ui/hello.slint").unwrap();
}
