use std::collections::HashMap;

/*使用雜湊映射與向量來建立文字介面，讓使用者能新增員工名字到公司內的一個部門。
舉來來說「將莎莉加入工程部門」或「將阿米爾加入業務部門」。然後讓使用者可以索取一個
部門所有的員工列表，或是依據部門用字點順序排序，取得公司內所有的員工。 */

pub fn run() {
    let mut departments: HashMap<String, Vec<String>> = std::collections::HashMap::new();

    loop {
        println!("輸入指令：");
        let mut input = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut input) {
            println!("讀取輸入失敗");
            return;
        }
        let input = input.trim();

        let mut parms = input
            .split(' ')
            .map(|x| String::from(x))
            .collect::<Vec<String>>();
        if parms[0] == "add" && parms.len() == 4 {
            let empolyee = parms.remove(1);
            let department = parms.remove(2);
            let department_empolyees = departments.entry(department).or_insert(Vec::new());
            department_empolyees.push(empolyee);
            department_empolyees.sort_by(|a, b| a.cmp(b));
        } else if parms[0] == "list" && parms.len() == 1 {
            let mut all = departments.iter().collect::<Vec<(&String, &Vec<String>)>>();
            all.sort_by(|a, b| a.0.cmp(b.0));
            for item in all {
                println!("{}:", item.0);
                for empolyees in item.1 {
                    println!("{}", empolyees);
                }
            }
        } else if parms[0] == "list" && parms.len() == 2 {
            match departments.get(&parms[1]) {
                Some(empolyees) => {
                    for empolyee in empolyees {
                        println!("{}", empolyee);
                    }
                }
                None => {
                    println!("not found");
                }
            }
        } else if parms[0] == "exit" {
            return;
        } else {
            println!("未知命令");
            continue;
        }
    }
}
