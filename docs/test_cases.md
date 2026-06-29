# 改良功能测试用例设计

## 1. 测试目标

本文档面向改良版 TanGram-rCore-Tutorial，补充设计 ch3/ch4 新增功能的测试用例。重点覆盖：

- ch3 `trace` 系统调用
- ch3 syscall 调用计数
- ch4 `mmap` 系统调用
- ch4 `munmap` 系统调用
- ch4 基于页表翻译的安全 `trace`

这些测试用于验证 A 实现的核心改良功能，也为后续自动化测试和报告展示提供依据。

## 2. ch3 `trace` 系统调用测试

### TC-CH3-TRACE-001: 读取合法用户地址

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证 `trace_request = 0` 能读取当前任务用户地址中的 1 字节数据 |
| 前置条件 | 用户程序中定义 `let var = 111u8;` |
| 操作步骤 | 调用 `trace_read(&var as *const u8)` |
| 预期结果 | 返回 `Some(111)` |
| 覆盖点 | 用户指针传递、内核读用户内存 |

### TC-CH3-TRACE-002: 写入合法用户地址

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证 `trace_request = 1` 能向当前任务用户地址写入 1 字节数据 |
| 前置条件 | 用户程序中定义可变变量 `let mut var = 111u8;` |
| 操作步骤 | 调用 `trace_write(&var as *const u8, 22)` |
| 预期结果 | 返回 `0`，随后读取 `var` 得到 `22` |
| 覆盖点 | 用户内存写入、`data as u8` 截断语义 |

### TC-CH3-TRACE-003: 查询 syscall 调用次数

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证 `trace_request = 2` 能返回当前任务指定 syscall 的调用次数 |
| 前置条件 | 用户程序多次调用 `get_time()` |
| 操作步骤 | 调用 `count_syscall(SYS_CLOCK_GETTIME)` |
| 预期结果 | 返回值大于等于实际调用次数 |
| 覆盖点 | `TaskControlBlock` syscall 计数、系统调用分发路径 |

### TC-CH3-TRACE-004: `trace` 调用自身计数

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证查询 `SYS_TRACE` 时，本次 `trace` 调用也被计入统计 |
| 前置条件 | 用户程序连续调用 `count_syscall(SYS_TRACE)` |
| 操作步骤 | 第一次查询后再次查询 `SYS_TRACE` |
| 预期结果 | `SYS_TRACE` 计数随查询次数增长 |
| 覆盖点 | syscall 计数发生在分发前，而不是处理完成后 |

### TC-CH3-TRACE-005: 非法 `trace_request`

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证未知 `trace_request` 能返回错误 |
| 前置条件 | 无 |
| 操作步骤 | 调用 `trace(99, 0, 0)` |
| 预期结果 | 返回 `-1` |
| 覆盖点 | 异常参数处理 |

## 3. ch4 `mmap` 系统调用测试

### TC-CH4-MMAP-001: 基本读写映射

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证 `mmap` 能创建可读写用户页映射 |
| 前置条件 | 选择页对齐且未映射的用户虚拟地址 |
| 操作步骤 | 调用 `mmap(start, len, PROT_READ | PROT_WRITE)` 后读写该区域 |
| 预期结果 | `mmap` 返回 `0`，读写成功 |
| 覆盖点 | 页表映射、用户页权限、物理页分配 |

### TC-CH4-MMAP-002: 只读映射写入

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证只读页不能被写入 |
| 前置条件 | 使用 `PROT_READ` 创建映射 |
| 操作步骤 | 用户程序尝试写入该页 |
| 预期结果 | 触发 `StorePageFault` 或任务被内核终止 |
| 覆盖点 | 页表写权限检查 |

### TC-CH4-MMAP-003: 重叠地址映射

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证重复映射已映射区域会失败 |
| 前置条件 | 已成功 `mmap(start, len, perm)` |
| 操作步骤 | 再次对相同范围调用 `mmap` |
| 预期结果 | 返回 `-1` |
| 覆盖点 | 地址范围合法性、重复映射检测 |

### TC-CH4-MMAP-004: 非页对齐地址

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证 `mmap` 对非页对齐地址的处理 |
| 前置条件 | 传入未按页大小对齐的 `start` |
| 操作步骤 | 调用 `mmap(start + 1, len, perm)` |
| 预期结果 | 返回 `-1` |
| 覆盖点 | 参数合法性检查 |

### TC-CH4-MMAP-005: 非法权限参数

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证权限参数为空或非法时返回错误 |
| 前置条件 | 无 |
| 操作步骤 | 调用 `mmap(start, len, 0)` 或传入未定义权限位 |
| 预期结果 | 返回 `-1` |
| 覆盖点 | 权限位校验 |

## 4. ch4 `munmap` 系统调用测试

### TC-CH4-MUNMAP-001: 解除已映射区域

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证 `munmap` 能解除用户虚拟页映射 |
| 前置条件 | 已成功 `mmap(start, len, perm)` |
| 操作步骤 | 调用 `munmap(start, len)` |
| 预期结果 | 返回 `0` |
| 覆盖点 | 页表取消映射、物理页回收 |

### TC-CH4-MUNMAP-002: 解除映射后访问

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证 `munmap` 后访问该地址会失败 |
| 前置条件 | 已成功映射并随后解除映射 |
| 操作步骤 | 用户程序读取或写入该地址 |
| 预期结果 | 触发 `LoadPageFault` 或 `StorePageFault` |
| 覆盖点 | 映射删除是否真正生效 |

### TC-CH4-MUNMAP-003: 解除未映射区域

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证对未映射区域调用 `munmap` 会失败 |
| 前置条件 | 选择未映射用户地址 |
| 操作步骤 | 调用 `munmap(start, len)` |
| 预期结果 | 返回 `-1` |
| 覆盖点 | 地址范围检查 |

### TC-CH4-MUNMAP-004: 非页对齐参数

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证 `munmap` 对非页对齐地址的处理 |
| 前置条件 | 无 |
| 操作步骤 | 调用 `munmap(start + 1, len)` |
| 预期结果 | 返回 `-1` |
| 覆盖点 | 参数合法性检查 |

## 5. ch4 安全 `trace` 测试

### TC-CH4-TRACE-001: 读取合法用户地址

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证 ch4 `trace_read` 能通过页表读取合法用户地址 |
| 前置条件 | 用户程序定义普通变量 |
| 操作步骤 | 调用 `trace_read(&var as *const u8)` |
| 预期结果 | 返回变量当前值 |
| 覆盖点 | 页表地址翻译、读权限判断 |

### TC-CH4-TRACE-002: 读取非法地址

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证 ch4 `trace_read` 能拒绝非法用户地址 |
| 前置条件 | 构造 `isize::MAX` 或内核地址 `0x80200000` |
| 操作步骤 | 调用 `trace_read(invalid_ptr)` |
| 预期结果 | 返回 `None` |
| 覆盖点 | 用户地址合法性、内核地址隔离 |

### TC-CH4-TRACE-003: 写入只读页

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证 `trace_write` 不能写入只读映射 |
| 前置条件 | 使用 `mmap` 创建只读页 |
| 操作步骤 | 调用 `trace_write(readonly_ptr, value)` |
| 预期结果 | 返回 `-1` |
| 覆盖点 | 写权限检查 |

### TC-CH4-TRACE-004: `munmap` 后 trace 访问

| 项目 | 内容 |
|---|---|
| 测试目标 | 验证解除映射后 `trace` 无法访问该地址 |
| 前置条件 | 映射一页后调用 `munmap` |
| 操作步骤 | 调用 `trace_read(start as *const u8)` 和 `trace_write(start as *const u8, 0)` |
| 预期结果 | `trace_read` 返回 `None`，`trace_write` 返回 `-1` |
| 覆盖点 | 页表解除映射与 trace 翻译一致性 |

## 6. 建议的自动化测试分层

| 层级 | 目标 | 示例 |
|---|---|---|
| smoke | 确认章节能启动并执行测试程序 | `cargo run --features exercise` |
| base | 验证章节基础功能 | `./test.sh base` |
| exercise | 验证练习功能 | `./test.sh exercise` |
| regression | 防止已修复功能退化 | ch3 trace、ch4 mmap/munmap |
| edge | 验证非法参数和边界条件 | 非法地址、非页对齐、重复映射 |

## 7. 当前验证状态

| 章节 | 测试命令 | 通过情况 | 说明 |
|---|---|---:|---|
| ch3 | `./test.sh exercise` | 7/7 | `trace` 与 syscall 计数测试通过 |
| ch4 | `./test.sh exercise` | 16/16 | `mmap/munmap`、page fault、ch4 trace 测试通过 |

## 8. 后续扩展方向

后续可继续为 ch5/ch6/ch8 补充测试：

- ch5：`fork`、`exec`、`wait`、exit code、孤儿进程处理。
- ch6：文件 open/read/write/close、非法 fd、文件偏移。
- ch8：mutex、semaphore、condvar、线程竞争与阻塞唤醒。

这些测试可以沿用本文档的格式，逐步扩展为统一的测试用例库。
