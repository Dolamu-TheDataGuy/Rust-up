fn main() {
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        ok(T),
        Err(E),
    }
}