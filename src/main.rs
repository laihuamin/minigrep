// $ cargo run searchstring example-filename.txt
// 任务目标可以读取参数
// 第一步：引入io，可以读写参数
use std::env;

fn main() {
    // 读取run参数
    // Vec是个什么东西
    // 类型定义都是大写的
    let args: Vec<String> = env::args().collect();
    
    println!("{:?}", args);
}
