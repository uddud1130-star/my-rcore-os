# 项目研发说明文档

> **项目名称：** 基于 AI 协作的组件化 rCore 操作系统教学实验环境改良
> **队伍：** 梦想是一觉醒来拥有百亿token
> **学校：** 西安电子科技大学

---

## 一、设计思路

### 1.1 问题出发点

当前操作系统内核教学存在以下核心问题：
- 学生缺乏一对一指导，原理与实践脱节
- 实验环境部分功能未实现，学生遇到障碍时无法判断是自身问题还是环境问题
- 缺乏即时反馈机制，实现完成后不知道是否正确

### 1.2 设计目标

在 TanGram-rCore-Tutorial 组件化参考环境基础上：
1. **补齐关键练习功能**：实现参考环境中未完成的 trace、mmap/munmap 系统调用
2. **增强错误可读性**：改进 Trap 错误信息，帮助学生快速定位问题
3. **完善测试体系**：新增回归测试，形成"实现-测试-反馈"闭环
4. **AI 深度协作**：全程与 Claude 协作，验证 AI 辅助 OS 学习的可行性

### 1.3 技术方案
参考环境（TanGram-rCore-Tutorial）

↓ 改良

我们的环境

├── ch2：改进 Trap 错误信息（1行→4行详细输出）

├── ch3：实现 trace 系统调用 + syscall 计数器

│         练习测试 5/7 → 8/8

├── ch4：实现 mmap/munmap + 基于页表的安全 trace

│         练习测试 9/16 → 17/17

├── ch5：验证进程管理 Basic usertests passed

├── tg-rcore-trace：独立 crate，发布至 crates.io

└── docs/：完整文档体系

---

## 二、实现描述

### 2.1 ch2 改进 Trap 错误信息

**改良前：**
[ERROR] app1 was killed because of Exception(StoreFault)

**改良后：**
[ERROR] app1 异常退出

异常类型: StoreFault

触发地址: 0x00000000

指令地址: 0x8040011e

可能原因: 程序尝试写入非法内存地址

**实现位置：** `tg-rcore-tutorial-ch2/src/main.rs`、`tg-rcore-tutorial-ch3/src/main.rs`

**核心修改：** 在 Trap 处理分支中读取 `stval` 和 `sepc` 寄存器，根据异常类型匹配中文原因说明。

### 2.2 ch3 trace 系统调用

**实现位置：** `tg-rcore-tutorial-ch3/src/main.rs`、`tg-rcore-tutorial-ch3/src/task.rs`

**核心设计：**

| trace_request | 功能 | 实现方式 |
|---|---|---|
| 0 | 读取用户内存 | 直接解引用用户指针 |
| 1 | 写入用户内存 | 直接写入用户指针 |
| 2 | 查询 syscall 次数 | 读取 TCB 的 syscall_times 数组 |

**关键改动：**
- `TaskControlBlock` 新增 `syscall_times: [u32; 512]` 字段
- 为避免栈溢出，将 TCB 数组改为静态全局存储
- syscall 计数在分发前完成，确保 trace 查询自身也被计入

### 2.3 ch4 mmap/munmap 系统调用

**实现位置：** `tg-rcore-tutorial-ch4/src/main.rs`

**mmap 实现流程：**
1. 检查地址页对齐（`addr % PAGE_SIZE == 0`）
2. 检查权限参数合法性（`prot != 0 && prot & !0x7 == 0`）
3. 检查地址范围是否已映射（避免重复映射）
4. 根据 prot 构建带 U 标志的页表权限（`URV`/`URWV` 等）
5. 调用 `address_space.map()` 分配物理页并建立映射

**munmap 实现：**
1. 检查地址对齐
2. 遍历范围内每一页，确认都已映射
3. 调用 `address_space.unmap()` 取消映射

### 2.4 ch4 基于页表的安全 trace

**与 ch3 trace 的区别：** ch4 引入虚拟内存后，用户地址不能直接解引用，必须通过页表翻译：

```rust
// ch4 trace 读取用户内存
if let Some(ptr) = process.address_space
    .translate::<u8>(VAddr::new(id), build_flags("URV")) {
    unsafe { *ptr.as_ptr() as isize }
} else {
    -1  // 地址非法或无读权限
}
```

### 2.5 独立 crate：tg-rcore-trace

**发布链接：** https://crates.io/crates/tg-rcore-trace

**提供的功能：**
- `SyscallCounter`：syscall 调用计数器
- `TraceRequest`：trace 请求类型枚举
- `handle_trace()`：统一处理 trace 请求

**单元测试：** 5个，全部通过

---

## 三、研发过程中遇到的问题和解决方法

### 问题1：WSL2 环境下 Rust 安装卡住

**现象：** `curl | sh` 安装 rustup 时卡在 downloading installer
**原因：** 国内访问官方服务器网速慢
**解决：** 改用中科大镜像源
```bash
RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup ./rustup-init.sh
```

### 问题2：ch3 trace 实现后内核启动卡死

**现象：** 加入 `syscall_times: [u32; 512]` 后内核无输出
**原因：** 每个 TCB 增加 2KB，32个 TCB 栈上分配超出 272KB 栈空间
**解决：** 将 TCB 数组改为静态全局变量
```rust
static mut TCBS_STORAGE: [TaskControlBlock; 32] = [TaskControlBlock::ZERO; 32];
```

### 问题3：mmap 返回成功但触发 PageFault

**现象：** `mmap` 返回 0，用户访问触发 `StorePageFault`
**原因：** 页表权限标志缺少用户态访问位 `U`
**解决：** 所有 mmap 权限加 `U` 前缀
```rust
1 => "URV",   // 用户只读
3 => "URWV",  // 用户读写
```

### 问题4：ch4 syscall 计数器引发内核崩溃

**现象：** 添加计数代码后出现 `StorePageFault`
**原因：** `ctx` 持有 `PROCESSES[0]` 的可变引用，再次访问 `PROCESSES` 产生借用冲突
**解决：** 将计数器改为独立全局变量，完全避开 PROCESSES 借用
```rust
static mut SYSCALL_TIMES: [u32; 512] = [0; 512];
```

### 问题5：ch4 syscall 计数不准确

**现象：** `count_syscall(SYS_CLOCK_GETTIME)` 返回值小于预期
**原因：** while 循环每次迭代都重置计数器，导致每次 Trap 后计数清零
**解决：** 将重置时机改为进程退出时
```rust
Id::EXIT => unsafe {
    PROCESSES.get_mut().remove(0);
    SYSCALL_TIMES = [0; 512]; // 进程退出时重置
},
```

### 问题6：回归测试文件位置错误

**现象：** B 提供的 `ch3_trace_extra.rs` 放在根目录，测试无法识别
**原因：** 需要放在对应章节的 `tg-rcore-tutorial-user/src/bin/` 目录，且需要在 `Cargo.toml` 中注册 `[[bin]]`，在 `cases.toml` 中加入测试集合
**解决：** 移动文件并更新配置，重装 checker 后 ch3 从 7/7 提升到 8/8，ch4 从 16/16 提升到 17/17

---

## 四、非本队来源说明

### 4.1 直接使用的外部代码

| 来源 | 使用方式 | 本队改动 |
|------|---------|---------|
| `tg-rcore-tutorial-ch1` | 直接使用，未修改 | 无 |
| `tg-rcore-tutorial-ch2` | 修改了 Trap 错误输出 | ch2/src/main.rs |
| `tg-rcore-tutorial-ch3` | 修改了 trace 实现、TCB 结构 | ch3/src/main.rs、task.rs |
| `tg-rcore-tutorial-ch4` | 修改了 mmap/munmap/trace 实现 | ch4/src/main.rs、process.rs |
| `tg-rcore-tutorial-ch5` | 直接使用，未修改 | 无 |
| `tg-rcore-tutorial-sbi` | 直接使用，未修改 | 无 |
| `tg-rcore-tutorial-linker` | 直接使用，未修改 | 无 |
| `tg-rcore-tutorial-console` | 直接使用，未修改 | 无 |
| `tg-rcore-tutorial-kernel-*` | 直接使用，未修改 | 无 |
| `tg-rcore-tutorial-syscall` | 直接使用，未修改 | 无 |
| `tg-rcore-tutorial-checker` | 修改了 ch3/ch4 判题规则 | checker/src/cases/ch3.rs、ch4.rs |
| `tg-rcore-tutorial-user` | 新增了2个回归测试程序 | ch3_trace_extra.rs、ch4_mmap_extra.rs |

### 4.2 本队原创内容

| 内容 | 说明 |
|------|------|
| `tg-rcore-tutorial-ch2/src/main.rs` 中的 Trap 详细输出 | 本队实现 |
| `tg-rcore-tutorial-ch3/src/task.rs` 中的 syscall_times 字段 | 本队实现 |
| `tg-rcore-tutorial-ch3/src/main.rs` 中的 trace 实现 | 本队实现 |
| `tg-rcore-tutorial-ch4/src/main.rs` 中的 mmap/munmap | 本队实现 |
| `tg-rcore-tutorial-ch4/src/main.rs` 中的 trace | 本队实现 |
| `tg-rcore-trace/` 独立 crate | 本队原创，已发布 crates.io |
| `tg-rcore-tutorial-user/src/bin/ch3_trace_extra.rs` | 本队（成员B）编写 |
| `tg-rcore-tutorial-user/src/bin/ch4_mmap_extra.rs` | 本队（成员B）编写 |
| `docs/` 目录下所有文档 | 本队（成员C）编写 |

### 4.3 参考资料

- rCore-Tutorial Book：https://rcore-os.cn/rCore-Tutorial-Book-v3/
- OSTEP 中文版：https://pages.cs.wisc.edu/~remzi/OSTEP/Chinese/
- RISC-V Reader 中文版：http://riscvbook.com/chinese/RISC-V-Reader-Chinese-v2p1.pdf
- TanGram-rCore-Tutorial 参考仓库：https://github.com/rcore-os/rCore-Tutorial-in-single-workspace

---

## 五、文档清单

| 文件 | 内容 |
|------|------|
| README.md | 项目概述、运行方法、测试结果 |
| docs/project_description.md | 本文档：设计思路、实现描述、问题记录、来源说明 |
| docs/design_report.md | 教学实验环境设计总结报告 |
| docs/AI使用报告.md | AI 工具使用声明与交互记录说明 |
| docs/comparison.md | 三方环境定性与定量对比分析 |
| docs/test_cases.md | 测试用例设计文档 |
| docs/verification.md | 测试验证报告 |
| docs/b-deliverables.md | 成员B交付说明 |
