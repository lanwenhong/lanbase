# lanbase

Rust 基础库，提供 TOML 配置加载和基于 `tracing` 的日志初始化。

- `config::load::<T>(path)` 将 TOML 文件读成应用自己的配置结构体（实现 `serde::Deserialize`）。
- `logger::init(&settings.log)` 在应用入口调用一次。日志级别支持 `trace/debug/info/warn/error`，输出格式支持 `text/json`。业务代码直接使用 `tracing` 的日志宏。

从仓库根目录运行示例：

```sh
cargo run --example basic -- conf/app.toml
```

[配置文件](conf/app.toml) 中的 `flush_interval = "3s"` 在[示例程序](examples/basic.rs)中解析为 `Duration`。可以把日志格式改成 `json` 查看结构化输出，或将级别改成 `warn` 过滤示例中的 INFO 事件。

其他项目可在 `Cargo.toml` 的 `[dependencies]` 中加入：

```toml
lanbase = { git = "https://github.com/lanwenhong/lanbase.git" }
```

使用示例中的派生宏、时长解析或日志宏时，调用方也需声明相应的 `serde`、`humantime-serde`、`tracing` 依赖。
