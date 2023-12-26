// 1.
// FUNCTIONAL DUPLICATION HANDLING BUT NOT TYPE AGNOSTIC
// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest(&number_list);
//     println!("the largest number is {}", result);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
//     let result = largest(&number_list);
//     println!("the largest number is {}", result);
// }

// 2.
// FUNCTIONAL DUPLICATION HANDLING WITH TYPE HANDLING
// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest_char(list: &[char]) -> &char{
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let result = largest_i32(&number_list);
//     println!("the largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];
//     let result = largest_char(&char_list);
//     println!("the largest number is {}", result);
// }

// 3.
// GENERICS
// fn foo<T>(bar: T) -> T {
//     bar
// }
// fn main() {
//     println!("this what i got {}", foo("sup"));
//     println!("this what i got {}", foo(3));
//     println!("this what i got {}", foo(3.4));
// }
//

// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }
// fn main() {
//     let l1 = vec!['y', 'm', 'a', 'q'];
//     println!("largest is....{}", largest(&l1));
//     let l1 = vec![34, 50, 25, 100, 65];
//     println!("largest is....{}", largest(&l1))
// }
//
// 4. GENERICS & IMPLEMENTATIONS

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// x1, y1 here are for the struct
impl<X1, Y1> Point<X1, Y1> {
    // the x2 y2 are only relevant for the method only
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let pt1 = Point { x: 4, y: 10.4 };
    let pt2 = Point { x: "Hello", y: 'c' };
    let pt3 = pt1.mixup(pt2);
    println!("p3.x = {}, p3.y = {}", pt3.x, pt3.y);
}
