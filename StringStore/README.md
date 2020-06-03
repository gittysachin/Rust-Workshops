## Rust's type system, standard library, and borrow checker to create a fluent API

StringStore is a struct that allows us to allocate a certain number of strings with a certain preset capacity. We can get a "string" from the story and when the "string" is dropped and we get another string, the underlying memory is reused.
