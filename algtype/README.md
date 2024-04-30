许多 trait 的实现全是模板代码，如 `Debug` 和 `Eq`。其实现可以自动生成，让使用者免于繁琐。常用的方法是 derive 宏，但是提供者写来依然繁琐。许多语言有反射可以简化此类代码，可惜 Rust 没有。

一些 trait 的实现仅需数据的结构，而无需其名称之类，如 `Eq`、`Ord`、`Hash`。此时，可以通过 `Generic`，使用数据的结构，代替 derive 宏，通用地实现 trait，不增加使用难度而减少编写难度。也可以拿来做别的事情。

这一做法，是被 Haskell 的 [`Generic`](https://wiki.haskell.org/Generics) 启发而来的。它似乎有很多理论基础，而我做得比较朴实，Rust 的类型系统也整不了什么活。

此库是 no_std 的，且没有不安全代码。以下是基于此库的一些功能：

- [count_enum](https://lib.rs/crates/count_enum)
- [power_map](https://lib.rs/crates/power_map)