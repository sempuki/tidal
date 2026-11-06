// A generic function with a trait bound, closures, and an Option result.
fn max_by_key<T, K: PartialOrd, F: Fn(&T) -> K>(items: &[T], key: F) -> Option<&T> {
    let mut best: Option<&T> = None;
    for it in items {
        best = match best {
            Some(b) if key(b) >= key(it) => Some(b),
            _ => Some(it),
        };
    }
    best
}
struct Word { text: &'static str, count: u32 }
fn main() {
    let words = [Word { text: "a", count: 3 }, Word { text: "b", count: 7 }, Word { text: "c", count: 5 }];
    let top = max_by_key(&words, |w| w.count).map(|w| w.text);
    let empty: [Word; 0] = [];
    let none = max_by_key(&empty, |w| w.count).is_none();
    let squares: u32 = (1..=4).filter(|n| n % 2 == 0).map(|n| n * n).sum();
    std::process::exit(if top == Some("b") && none && squares == 20 { 0 } else { 1 });
}
