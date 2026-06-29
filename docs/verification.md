# 改良版实验环境验证报告

## 1. 验证目的

本文档记录对改良版 TanGram-rCore-Tutorial 实验环境的实际运行验证结果，重点验证组员 A 已完成的 ch3/ch4 改良功能是否能够通过练习测试。

验证目标包括：

- 确认 ch3 `trace` 系统调用和 syscall 计数功能是否正确。
- 确认 ch4 `mmap/munmap` 系统调用是否正确。
- 确认 ch4 基于页表翻译的 `trace` 功能是否正确。
- 记录编译 warning、运行异常输出与最终测试结果之间的关系。

## 2. 验证环境

| 项目 | 内容 |
|---|---|
| 操作系统 | Linux / Ubuntu 环境 |
| 运行方式 | Bash 终端运行 `cargo`、`test.sh` 与 QEMU |
| 目标架构 | `riscv64gc-unknown-none-elf` |
| 模拟器 | `qemu-system-riscv64` |
| 仓库 | `my-rcore-os` |
| 验证章节 | `tg-rcore-tutorial-ch3`、`tg-rcore-tutorial-ch4` |

说明：Codex 会话运行在 Windows 设备上，实际编译和测试在 Linux 设备上完成。本文档根据 Linux 端导出的运行日志整理。

## 3. 验证命令

### ch3 exercise

```bash
cd ~/my-rcore-os/tg-rcore-tutorial-ch3
./test.sh exercise
```

### ch4 exercise

```bash
cd ~/my-rcore-os/tg-rcore-tutorial-ch4
cargo build --features exercise
./test.sh exercise
```

## 4. ch3 验证结果

### 4.1 测试结论

ch3 exercise 测试通过，最终结果为：

```text
Test PASSED: 7/7
✓ ch3 练习测试通过
```

### 4.2 关键输出

运行日志中出现以下关键输出：

```text
string from task trace test
Test trace OK!
```

这说明 ch3 中 `trace` 相关用户程序已经正常执行，且没有再出现之前的：

```text
trace: not implemented
```

也没有再出现：

```text
assertion failed: 3 <= count_syscall(SYS_CLOCK_GETTIME)
```

### 4.3 功能验证点

ch3 exercise 通过说明以下功能已经被验证：

| 功能点 | 验证情况 |
|---|---|
| `trace_request = 0` 读取用户地址 | 通过 |
| `trace_request = 1` 写入用户地址 | 通过 |
| `trace_request = 2` 查询 syscall 调用次数 | 通过 |
| `SYS_CLOCK_GETTIME` 调用次数统计 | 通过 |
| `SYS_TRACE` 调用自身计数 | 通过 |
| `SYS_WRITE`、`SYS_SCHED_YIELD`、`SYS_EXIT` 计数逻辑 | 通过 |

### 4.4 与原始环境对比

| 项目 | 原始环境 | 改良后环境 |
|---|---:|---:|
| ch3 exercise 通过数量 | 5/7 | 7/7 |
| `trace` 系统调用 | 未完整实现 | 已实现 |
| syscall 计数器 | 未完整实现 | 已实现 |

## 5. ch4 验证结果

### 5.1 测试结论

ch4 exercise 测试通过，最终结果为：

```text
Test PASSED: 16/16
✓ ch4 练习测试通过
```

### 5.2 关键输出

运行日志中出现以下关键输出：

```text
Test trace OK!
Test 04_1 OK!
Test 04_4 test OK!
Test 04_5 ummap OK!
Test 04_6 ummap2 OK!
Test trace_1 OK!
```

这些输出说明 ch4 的时间系统调用、`trace`、`mmap`、`munmap` 相关测试均已通过。

### 5.3 功能验证点

ch4 exercise 通过说明以下功能已经被验证：

| 功能点 | 验证情况 |
|---|---|
| `mmap` 创建用户虚拟页映射 | 通过 |
| `munmap` 解除用户虚拟页映射 | 通过 |
| 非法映射参数处理 | 通过 |
| 映射权限检查 | 通过 |
| 解除映射后访问异常 | 通过 |
| 基于页表翻译的 `trace_read` | 通过 |
| 基于页表翻译的 `trace_write` | 通过 |
| 内核地址和非法用户地址访问拒绝 | 通过 |

### 5.4 与原始环境对比

| 项目 | 原始环境 | 改良后环境 |
|---|---:|---:|
| ch4 exercise 通过数量 | 9/16 | 16/16 |
| `mmap` 系统调用 | 未完整实现 | 已实现 |
| `munmap` 系统调用 | 未完整实现 | 已实现 |
| ch4 安全 `trace` | 未完整实现 | 已实现 |

## 6. Warning 分析

测试过程中出现了若干 Rust 编译 warning，主要类型为：

```text
warning[E0133]: use of inline assembly is unsafe and requires unsafe block
warning[E0133]: dereference of raw pointer is unsafe and requires unsafe block
warning[E0133]: call to unsafe function is unsafe and requires unsafe block
```

这些 warning 主要来自以下底层组件：

| 组件 | warning 类型 |
|---|---|
| `tg-rcore-tutorial-kernel-context` | 内联汇编、裸指针解引用、上下文切换 |
| `tg-rcore-tutorial-kernel-alloc` | unsafe 内存分配器操作 |

这些 warning 与 Rust 2024 的 `unsafe_op_in_unsafe_fn` 兼容性检查有关。它们提示在 `unsafe fn` 内部执行 unsafe 操作时仍建议显式写出 `unsafe {}` 块。

本次验证中，这些 warning 没有导致编译失败，也没有影响 QEMU 运行和测试结果。因此当前阶段可以将其记录为后续代码质量改进项，而不是功能阻塞问题。

后续可改进方向：

- 在底层组件中为内联汇编和裸指针操作补充显式 `unsafe {}` 块。
- 对关键 unsafe 代码补充简短安全性说明。
- 将 warning 清理作为代码质量优化任务，而不是当前测试通过的前置条件。

## 7. Page Fault 输出分析

ch4 exercise 运行过程中出现过类似输出：

```text
[ERROR] unsupported trap: Exception(StorePageFault)
[ERROR] unsupported trap: Exception(LoadPageFault)
```

从最终测试结果看，ch4 exercise 仍然通过 `16/16`。同时，测试脚本确认以下失败标记没有出现：

```text
Should cause error, Test 04_2 fail!
Should cause error, Test 04_3 fail!
```

因此，这些 Page Fault 输出应理解为测试用例故意触发的异常路径，用于验证：

- 只读页写入是否会失败。
- 解除映射后访问是否会失败。
- 非法地址访问是否能被内核识别。

结论：这些 Page Fault 输出不是环境错误，而是 ch4 内存保护测试的一部分。

## 8. 总体验证结论

本次验证确认，组员 A 完成的 ch3/ch4 改良代码能够通过对应 exercise 测试：

| 章节 | 测试命令 | 结果 |
|---|---|---:|
| ch3 | `./test.sh exercise` | 7/7 |
| ch4 | `./test.sh exercise` | 16/16 |

从功能角度看：

- ch3 已补齐 `trace` 系统调用与 syscall 计数器。
- ch4 已补齐 `mmap/munmap` 与基于页表翻译的安全 `trace`。
- 编译 warning 不影响当前功能测试结果。
- ch4 中出现的 Page Fault 输出属于预期异常路径测试。

因此，当前改良版实验环境已经完成 ch3/ch4 关键练习功能的验证闭环，可作为后续三方对比分析和补充测试用例设计的依据。
