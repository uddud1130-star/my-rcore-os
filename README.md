# My rCore OS 教学实验环境

> **学校：** 西安电子科技大学
> **队伍ID：** T2026107019910936
> **队伍名称：** 梦想是一觉醒来拥有百亿token

基于 rCore-Tutorial 的改良版操作系统教学实验环境，以 AI 深度协作为核心方法论，面向计算机专业本科生自学操作系统内核设计。

---

## 实验列表

| 实验 | 主题 | 状态 | 测试结果 |
|------|------|------|---------|
| ch1 | 裸机启动+最小执行环境 | ✅ 完成 | 通过 |
| ch2 | 批处理系统+特权级切换+改进Trap错误信息 | ✅ 完成 | 基础 5/5 |
| ch3 | 多道程序+调度+trace系统调用 | ✅ 完成 | 练习 8/8 |
| ch4 | 虚拟内存+mmap/munmap+trace | ✅ 完成 | 练习 17/17 |
| ch5 | 进程管理+Shell | ✅ 完成 | Basic usertests passed |

---

## 改良点

- ✅ **改进Trap错误信息**：出错时打印异常类型、触发地址、指令地址和可能原因，帮助学生快速定位错误
- ✅ **实现trace系统调用**：支持用户内存读写和syscall调用次数统计
- ✅ **实现mmap系统调用**：支持用户态动态内存映射，含读/写/执行权限控制
- ✅ **实现munmap系统调用**：支持取消内存映射，含地址对齐和范围合法性检查
- ✅ **封装独立crate**：将trace功能封装为独立Rust库，已发布到crates.io
- ✅ **补充回归测试**：新增ch3/ch4额外测试用例，纳入自动化判题

---

## 对比参考环境的提升

| 指标 | 参考环境 | 本环境 | 提升 |
|------|---------|--------|------|
| ch3练习测试通过率 | 5/7 (71%) | 8/8 (100%) | +29% |
| ch4练习测试通过率 | 9/16 (56%) | 17/17 (100%) | +44% |
| Trap错误信息 | 1行简单输出 | 4行详细信息 | ✅ |
| trace功能 | ❌ 未实现 | ✅ 完整实现 | - |
| mmap/munmap | ❌ 未实现 | ✅ 完整实现 | - |
| 独立crate | ❌ 无 | ✅ 已发布 | - |
| 单元测试 | ❌ 无 | ✅ 5/5通过 | - |
| 回归测试 | ❌ 无 | ✅ 新增2个 | - |

---

## 如何运行

```bash
# 环境要求
# - WSL2 + Ubuntu 22.04
# - Rust stable 1.95.0
# - QEMU 8.2.0

# ch1
cd tg-rcore-tutorial-ch1
cargo run

# ch2
cd tg-rcore-tutorial-ch2
cargo run

# ch3 全部测试
cd tg-rcore-tutorial-ch3
cd tg-rcore-tutorial-user && cargo build && cd ..
TG_USER_LOCAL_DIR=./tg-rcore-tutorial-user bash test.sh
# 期望：8/8

# ch4 练习测试
cd tg-rcore-tutorial-ch4
cd tg-rcore-tutorial-user && cargo build && cd ..
TG_USER_LOCAL_DIR=./tg-rcore-tutorial-user bash test.sh exercise
# 期望：17/17

# ch5
cd tg-rcore-tutorial-ch5
TG_USER_LOCAL_DIR=./tg-rcore-tutorial-user cargo run
# 出现Shell后输入：ch5b_usertest

# 单元测试
cd tg-rcore-trace
cargo test
```

---

## 发布的Crate

| Crate | 说明 | 链接 |
|-------|------|------|
| tg-rcore-trace | trace系统调用封装库，含5个单元测试 | https://crates.io/crates/tg-rcore-trace |

---

## 文档目录

| 文档 | 说明 |
|------|------|
| docs/design_report.md | 教学实验环境设计总结报告 |
| docs/AI使用报告.md | AI工具使用报告 |
| docs/comparison.md | 三方环境定性与定量对比分析 |
| docs/test_cases.md | 测试用例设计文档 |
| docs/verification.md | 测试验证报告 |
| docs/b-deliverables.md | 成员B交付说明 |

---

## 技术栈

- 编程语言：Rust（stable 1.95.0，no_std内核模式）
- 硬件架构：RISC-V 64（riscv64gc-unknown-none-elf）
- 运行环境：QEMU 8.2.0 virt虚拟机
- 文档格式：Markdown + Mermaid
- AI协作工具：Claude

---

## 团队分工

| 成员 | 职责 |
|------|------|
| 王雅宁 | 内核功能实现（trace/mmap/munmap/Trap错误信息） |
| 陶靓颖 | 测试用例设计、回归测试、对比分析 |
| 贾天纬 | 文档整合、设计报告、Prompt Engineering |

---

## 参考资料

- [rCore-Tutorial 参考环境](https://github.com/rcore-os/rCore-Tutorial-in-single-workspace)
- [rCore-Tutorial Book](https://rcore-os.cn/rCore-Tutorial-Book-v3/)
- [OSTEP 中文版](https://pages.cs.wisc.edu/~remzi/OSTEP/Chinese/)
- [RISC-V Reader 中文版](http://riscvbook.com/chinese/RISC-V-Reader-Chinese-v2p1.pdf)
