use crate::summary::{Tweet, Summary, notify};

mod summary;

#[derive(Debug)]
struct Point<T>(T, T);

fn main() {
    let list = vec![20, 30, 100, 40, 50];
    println!("The largest number is {}", largest(&list));

    let point = Point(5, 10);
    println!("Point is {:?}", point);

    let tweet = Tweet {
        username: String::from("danielcr_"),
        content: String::from("https://yourmom.zip/"),
        reply: false,
        retweet: false
    };
    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

#[allow(dead_code)]
fn foo<T, U>(_t: T, _u: U) -> i32
    where T: std::fmt::Display + Clone,
          U: Clone + std::fmt::Debug
{
    0
}
