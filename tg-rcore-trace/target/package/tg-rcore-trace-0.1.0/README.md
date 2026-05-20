# tg-rcore-trace

rCore教学实验环境的trace系统调用扩展库。

## 功能

- **系统调用计数**：统计每个syscall的调用次数
- **用户内存读取**：安全读取用户态内存地址
- **用户内存写入**：安全写入用户态内存地址

## 背景

本库是2026全国大学生计算机系统能力大赛OS功能挑战赛道的参赛作品，
基于清华大学rCore-Tutorial进行改良，实现了参考环境中未实现的trace系统调用。

## 改良成果

| 指标 | 参考环境 | 改良后 |
|------|---------|--------|
| ch3练习测试 | 5/7 | 7/7 |
| ch4练习测试 | 9/16 | 16/16 |

## 使用方法

```toml
[dependencies]
tg-rcore-trace = "0.1.0"
```

```rust
use tg_rcore_trace::{SyscallCounter, TraceRequest, handle_trace};

let mut counter = SyscallCounter::new();
counter.record(64); // 记录write系统调用
let count = counter.count(64); // 查询次数
```

## License

GPL-3.0
