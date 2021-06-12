use core::num;

/**
泛型練習
*/
pub(crate) fn run() -> () {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("最大數字為 {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("最大數字為 {}", result);
}

///查詢列表中最大的數字
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
