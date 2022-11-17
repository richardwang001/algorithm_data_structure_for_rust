mod lib;

use std::io::{BufRead, stdin};
use crate::lib::*;

fn main() {
    let mut size_string = String::new();
    let mut max_size = 0;

    loop {
        if max_size == 0 {
            println!("请输入队列最大长度(大于零）：");
            stdin().read_line(&mut size_string).expect("读取长度信息失败!");
            max_size = size_string.trim().parse().expect("请输入大于零的整数");
        } else {
            max_size += 1;
            break;
        }
    }

    let mut queue = CircleQueue::new(max_size);
    let mut action = "";

    loop {
        println!("请输入操作：");
        println!("\t add 添加元素");
        println!("\t rm 取出元素");
        println!("\t show 显示队列");
        println!("\t end 显示队尾元素");
        println!("\t head 显示队首元素");
        println!("\t size 显示队列个数");
        println!("\t quit 退出程序");
        let stdin1 = stdin();

        // 只读取第一行！！！
        let action_string = stdin1.lock().lines().next().unwrap().unwrap();

         action = action_string.trim();
        match action {
            "add" => {
                println!("请输入要加入队列最后的数字：");
                let mut input = String::new();
                stdin().read_line(&mut input).expect("读取输入元素失败!");
                let input_num = input.trim().parse().expect("请输入数字元素");
                queue.add_one(input_num);
            }
            "rm" => {
                println!("进行出队操作");
                let removed = queue.get_one();
                if let Some(removed) = removed {
                    println!("出队的元素是:{}", removed);
                }
            }
            "show" => {
                println!("显示队列的所有元素");
                queue.show_queue();
            }
            "end" => {
                let end = queue.get_ender();
                if let Some(end) = end {
                    println!("队尾元素为：{}", end);
                };
            }
            "head" => {
                let head = queue.get_header();
                if let Some(head) = head {
                    println!("队首元素为：{}", head);
                }
            }
            "size" => {
                println!("队列中已有 {} 个元素", queue.actual_size());
            }
            "quit" => {
                break;
            }
            _ => {
                println!("输入的字符未匹配到操作！");
                continue;
            }
        }
    }

    println!("程序已退出！");
}
