## More advanced things like Multithreading

Rust ensures that our code is free of data races. Sometimes this means that we explictly have to wrap objects in syncronization primitives to make sure that data is not suceptible to data races.

I've made a thread pool which allows us to do work on many different threads, reusing them after their previous work is done.

It's important to note how we are forced by the Rust compiler to use an Arc and an AtomicUsize in order to add to the count in a thread safe way.
