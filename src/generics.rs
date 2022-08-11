struct Point<T> {
    x: T,
    y: T,
}
struct PointAnother<T, U> {
    x: T,
    y: U,
}
impl<T, U> PointAnother<T, U> {
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = number_list[0];
    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest is {}", largest);
    // println!("{}", largest_i32(number_list));
    // rustでは"で囲うと文字列リテラル、'で囲うとchar型になる
    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("{}", largest(char_list));
    println!("{}", largest(number_list));

    // 構造体を実体化する
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = PointAnother { x: 5, y: 10.4 };
    let p4 = PointAnother { x: "Rust", y: 'a' };
    let p5 = p3.mixup(p4);
    println!("{} {}", p5.x, p5.y);
}

fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// PartialOrd + Copyでtrait境界を設定できる
// これによりgenericsで受け取るデータの型を比較が可能な型に絞ることができる
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
