#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

use std::convert::TryInto;

/*fn print_point(point: &Point) {
    println!("x: {}, y: {}", point.x, point.y);
}
fn main() {
    let point = Point {
        x: 24,
        y: 42,
    };
    println!("({}, {})", point.x, point.y);
    println!("{:#?}", point);

    let p1 = Point { x: 1, y: 2 };
//    let p2 = &p1;
//    println!("{}", p1.x);
    print_point(&p1);
    println!("x of point1 is: {}", p1.x);
}
*/
fn main() {
    fn print_point(point: Point) {
        println!("x: {}, y: {}", point.x, point.y);
    }
    let p1 = Point { x: 105, y: 28 };
    let p2 = p1.clone();
    print_point(p1.clone());
    println!("{}", p2.x);
    println!("my test");
    test(p1.x.try_into().unwrap());
}

fn test(p: u8) {
    let x: bool;

    if p > 55 {
        x = false;
    }
    else {
        x = true;
    }
    println!("x = {}", x)
}
