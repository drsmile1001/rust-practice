use core::num;

/**
泛型練習
*/
// pub(crate) fn run() -> () {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("最大數字為 {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("最大字元為 {}", result);
// }

///查詢列表中最大的東西
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

///泛型座標點
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

pub(crate) fn run() -> () {
    let interger = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("p.x = {}", interger.x());
}
