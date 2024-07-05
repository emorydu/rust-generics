// #[derive(Debug)]
// struct Point<T> {
//     x: T,
//     y: T,
// }
// let integer = Point { x: 5, y: 10 };
// let float = Point { x: 1.0, y: 4.0 };
// println!("integer: {:?}", integer);
// println!("float: {:?}", float);

// #[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// let both_integer = Point { x: 5, y: 10 };
// let both_float = Point { x: 1.0, y: 5.0 };
// let integer_and_float = Point {x: 5, y: 10.9 };

// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
// impl Point<i32> {
//     fn distance_from_origin(&self) -> f32 {
//        (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }
// let p = Point {x: 5, y: 10 };
// println!("p.x = {}", p.x());

// struct Point<X1, Y1> {
//     x: X1,
//     y: Y1,
// }
// impl <X1, Y1> Point <X1, Y1> {
//     fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point <X1, Y2> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
// let p1 = Point { x: 5, y: 0.4 };
// let p2 = Point { x: "hello", y: 'c' };
// let p3 = p1.mixup(p2);
// println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("The largest number is: {}", largest(&numbers));

    let chars = vec!['h', 'e', 'l', 'o'];
    println!("The largest char is: {}", largest(&chars));
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}