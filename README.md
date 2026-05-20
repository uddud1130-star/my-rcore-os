# My rCore OS 教学实验环境

基于 rCore-Tutorial 的改良版操作系统教学实验环境。

## 实验列表

| 实验 | 主题 | 状态 | 测试结果 |
|------|------|------|---------|
| ch1 | 裸机启动+最小执行环境 | ✅ 完成 | 通过 |
| ch2 | 批处理系统+特权级切换 | ✅ 完成 | 基础 5/5 |
| ch3 | 多道程序+调度+trace系统调用 | ✅ 完成 | 练习 7/7 |
| ch4 | 虚拟内存+mmap/munmap+trace | ✅ 完成 | 练习 16/16 |
| ch5 | 进程管理+Shell | ✅ 完成 | 基础全过 |

## 改良点

- ✅ 实现了 trace 系统调用，支持用户内存读写和syscall计数
- ✅ 实现了 mmap 系统调用，支持用户态动态内存映射
- ✅ 实现了 munmap 系统调用，支持取消内存映射及合法性检查
- ✅ 封装独立 crate 并发布到 crates.io

## 对比参考环境的提升

| 指标 | 参考环境 | 本环境 | 提升 |
|------|---------|--------|------|
| ch3练习测试通过率 | 5/7 (71%) | 7/7 (100%) | +29% |
| ch4练习测试通过率 | 9/16 (56%) | 16/16 (100%) | +44% |
| trace功能 | ❌ 未实现 | ✅ 完整实现 | - |
| mmap/munmap | ❌ 未实现 | ✅ 完整实现 | - |

## 发布的 Crate

| Crate | 说明 | 链接 |
|-------|------|------|
| tg-rcore-trace | trace系统调用封装库 | https://crates.io/crates/tg-rcore-trace |

## 技术栈

- 编程语言：Rust（nightly/stable）
- 硬件架构：RISC-V 64（riscv64gc）
- 运行环境：QEMU virt 虚拟机
- 文档格式：Markdown + Mermaid

## 参考资料

- [rCore-Tutorial 参考环境](https://github.com/rcore-os/rCore-Tutorial-in-single-workspace)
- [rCore-Tutorial Book](https://rcore-os.cn/rCore-Tutorial-Book-v3/)
- [OSTEP 中文版](https://pages.cs.wisc.edu/~remzi/OSTEP/Chinese/)
