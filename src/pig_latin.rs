/*將字串轉換成 pig latin。每個單字的第一個字母為子音的話，就將該字母移到單字後方，
並加上「ay」，所以「first」會變成「irst-fay」。而單字第一個字母為母音的話，就在單字後
方加上「hay」，所以「apple」會變成「apple-hay」。請注意要考慮到 UTF-8 編碼！ */

pub fn run() {
    loop {
        println!("輸入要 pig latin 的詞:");
        let mut input = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut input) {
            println!("讀取輸入失敗");
            return;
        }
        let input = input.trim();

        let first_letter = match input.chars().next() {
            Some(c) => c,
            None => {
                println!("讀取輸入失敗");
                return;
            }
        };
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let result = match vowels.contains(&first_letter) {
            true => format!("{}-hay", input),
            false => format!(
                "{}-{}ay",
                input.chars().skip(1).collect::<String>(),
                first_letter
            ),
        };
        println!("{}", result);
    }
}
