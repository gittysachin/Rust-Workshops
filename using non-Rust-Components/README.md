## Integrating non-Rust components with our Rust Code

I have used the `libcurl` library to perform an HTTPS request.

In order to call libcurl's C API, we'll first need to define libcurl's data types and fuction signatures in Rust. Fortunately, the `curl-rust` project has already done this for us in curl-sys.
