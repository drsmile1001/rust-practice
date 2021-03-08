use std::collections::HashMap;

/*給予一個整數列表，請使用向量並回傳算數平均數、中位數（排序列表後正中間的值）以及眾數（出現最多次的值，雜湊映射在此應該會很有用） */
pub fn run() {
    println!("依序輸入整數：");
    let mut numbers: Vec<i32> = Vec::new();
    loop {
        let mut input = String::new();
        let result = std::io::stdin().read_line(&mut input);
        if let Err(_) = result {
            break;
        }
        match input.trim().parse::<i32>() {
            Ok(i) => numbers.push(i),
            Err(_) => break,
        }
    }
    let count = numbers[..].len();
    if count < 1 {
        println!("請輸入1個以上數字");
        return;
    }

    let inputs: String = numbers.iter().map(|x| format!("{},", x)).collect();
    println!("你輸入了：{}", inputs);

    let sum = numbers.iter().sum::<i32>();
    let avg = sum as f64 / count as f64;
    println!("avg:{}", avg);
    numbers.sort();
    let inputs: String = numbers.iter().map(|x| format!("{},", x)).collect();
    println!("排序後{}", inputs);
    let middle;
    if count % 2 == 0 {
        middle =
            (numbers[count / (2 as usize)] + numbers[(count / (2 as usize)) - 1]) as f64 / 2 as f64;
    } else {
        middle = numbers[count / (2 as usize)] as f64
    }
    println!("middle:{}", middle);
    let mut number_counts: HashMap<i32, u32> = std::collections::HashMap::new();
    for i in numbers {
        let count = number_counts.entry(i).or_insert(0);
        *count += 1;
    }
    let mut number_counts = number_counts.iter().collect::<Vec<(&i32, &u32)>>();
    number_counts.sort_by(|a, b| b.1.cmp(a.1));
    println!("衆數:{}", number_counts[0].0);
}
