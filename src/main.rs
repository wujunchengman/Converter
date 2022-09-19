use std::io;
fn main() {
    println!("Hello, world!");

    println!("请选择需要进行的转换，直接回车退出程序");

    // 循环输入长度并求出转换后的值
    loop {
        
        hint();

        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("无法读取行");
        if input.trim().len()==0 {
            break;
        }

        let chose:i32 = input.trim().parse().expect("请输入一个数字！");

        if chose == 1 {
            cm_to_px();
        }
        else if chose ==2 {
            cm_to_pt_loop();
        }

        
    }

}

fn hint() {
    println!("1 厘米cm转像素px");
    println!("2 厘米cm转磅pt");
}


fn cm_to_px() ->i32 {

    // 定义DPI

    println!("请输入显示DPI");
    let mut dpi = String::new();
    io::stdin()
        .read_line(&mut dpi)
        .expect("无法读取行");
    
    let dpi:f32 = dpi.trim().parse().expect("Dpi不是一个整数无法转换");

    loop {
        println!("请输入需要转换的长度（单位cm）");
        println!("空行回车返回");

        let mut len = String::new();

        io::stdin()
            .read_line(&mut len)
            .expect("无法读取输入行");

        if len.trim().len()==0 {
            return 0;
        }

        let len:f32 = len.trim().parse().expect("请输入一个数字！");

        let px = len*dpi/2.54;
        print!("转换后的值是{px}");
    }
}

fn cm_to_pt_loop() {
    loop {
        loop {
            println!("请输入需要转换的长度（单位cm）");
    
            let mut len = String::new();
    
            io::stdin()
                .read_line(&mut len)
                .expect("无法读取输入行");
    
            if len.trim().len()==0 {
                return;
            }
    
            let len:f32 = len.trim().parse().expect("请输入一个数字！");
    
            // 一英寸等于72pt，一英寸等于2.54厘米


            // 整倍英寸
            let num:i32 = ((len * 100f32) / 254f32) as i32;
            // 剩下的厘米数*100
            let remainder = (len * 100f32) % 254f32;

            let pt = (num * 72) as f32 + ((remainder*72f32) /254f32);
            print!("转换后的值是{pt}");
        }
    }
}

