# 成员 B 交付说明

## 1. 职责定位

成员 B 负责改良版实验环境设计、测试用例补充和三方实验环境对比分析。本次工作围绕 TanGram-rCore-Tutorial 的 ch3/ch4 练习功能展开，重点完成测试环境可复现、补充测试用例可运行、checker 能正式判定新增测试结果。

## 2. 已完成内容

### 2.1 文档交付

已完成以下文档：

| 文件 | 内容 |
|---|---|
| `docs/comparison.md` | 三方实验环境对比分析，比较 rCore-Tutorial、原始 TanGram-rCore-Tutorial 与本项目改良版 |
| `docs/test_cases.md` | 改良功能测试用例设计，覆盖 `trace`、syscall 计数、`mmap/munmap`、page fault 等测试点 |
| `docs/verification.md` | 实际运行验证报告，记录 ch3/ch4 测试命令、运行结果、warning 与 page fault 解释 |
| `docs/b-deliverables.md` | 成员 B 最终交付说明 |

### 2.2 测试环境纳入仓库

为保证测试环境可复现，本次将以下目录纳入项目仓库：

| 目录 | 作用 |
|---|---|
| `tg-rcore-tutorial-user/` | 用户态测试程序集合 |
| `tg-rcore-tutorial-checker/` | 测试输出检查工具 |

这样其他成员或评审人员 clone 仓库后，可以直接复现新增测试，而不需要手动复制外部测试组件。

### 2.3 新增用户态测试程序

新增两个补充测试程序：

| 文件 | 所属章节 | 测试目标 |
|---|---|---|
| `tg-rcore-tutorial-user/src/bin/ch3_trace_extra.rs` | ch3 | 验证非法 `trace_request`、`SYS_TRACE` 自身计数、单字节读写 |
| `tg-rcore-tutorial-user/src/bin/ch4_mmap_extra.rs` | ch4 | 验证重复 `mmap`、`munmap` 后不可访问、只读页拒绝写入 |

### 2.4 测试集合配置

已在 `tg-rcore-tutorial-user/cases.toml` 中将新增测试加入对应 exercise 测试集合：

| 测试集合 | 新增测试 |
|---|---|
| `ch3_exercise` | `ch3_trace_extra` |
| `ch4_exercise` | `ch4_mmap_extra` |

同时在 `tg-rcore-tutorial-user/Cargo.toml` 中注册新增 bin target，保证构建脚本能够正确识别并编译新增用户程序。

### 2.5 checker 判定规则更新

已更新 checker 规则，将新增测试输出纳入正式判定：

| 文件 | 新增检查项 |
|---|---|
| `tg-rcore-tutorial-checker/src/cases/ch3.rs` | `Test ch3_trace_extra OK!` |
| `tg-rcore-tutorial-checker/src/cases/ch4.rs` | `Test ch4_mmap_extra OK!` |

因此新增测试不只是被运行，也会被 checker 计入最终通过数量。

## 3. 最终验证结果

### 3.1 ch3 exercise

测试命令：

```bash
cd ~/my-rcore-os/tg-rcore-tutorial-ch3
./test.sh exercise
```

最终结果：

```text
Expected patterns: 8, Not expected: 0
Test PASSED: 8/8
✓ ch3 练习测试通过
```

关键新增输出：

```text
Test ch3_trace_extra OK!
```

### 3.2 ch4 exercise

测试命令：

```bash
cd ~/my-rcore-os/tg-rcore-tutorial-ch4
./test.sh exercise
```

最终结果：

```text
Expected patterns: 13, Not expected: 4
Test PASSED: 17/17
✓ ch4 练习测试通过
```

关键新增输出：

```text
Test ch4_mmap_extra OK!
```

## 4. 改良效果总结

| 项目 | 原始结果 | 改良后结果 |
|---|---:|---:|
| ch3 exercise | 5/7 | 8/8 |
| ch4 exercise | 9/16 | 17/17 |

本次改良不仅补齐了 ch3/ch4 的原有练习功能，还进一步增加了边界测试和回归测试，使测试体系从“功能刚好通过”推进到“关键异常路径也被验证”。

## 5. 后续可扩展方向

后续可以继续扩展以下方向：

- ch5：补充 `fork/exec/wait`、exit code、进程调度相关测试。
- ch6：补充文件系统 open/read/write/close、非法 fd、文件偏移测试。
- ch8：补充 mutex、semaphore、condvar、线程竞争与阻塞唤醒测试。
- 测试框架：进一步将 checker 输出结构化，增加测试名称、失败原因和关联模块提示。
