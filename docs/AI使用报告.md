# AI 工具使用报告

> 项目名称：基于 AI 协作的组件化 rCore 操作系统教学实验环境改良
> 队伍 ID：T2026107019910936
> 队伍名称：梦想是一觉醒来拥有百亿token
> 学校：西安电子科技大学
> 报告版本：v1.0

---

## 一、AI 工具使用声明

本项目全程合理使用 AI 工具辅助开发，符合大赛组委会关于 AI 工具使用的相关规定。所有 AI 工具的使用场景、交互过程及成果均在本报告中如实披露。

---

## 二、使用的 AI 工具清单

| 工具名称 | 工具类型 | 使用场景 |
|---------|---------|---------|
| Claude (Anthropic) | 大语言模型对话助手 | 核心开发辅助工具，贯穿全程 |
| GitHub Copilot | IDE 代码补全插件 | Rust 代码补全与语法提示 |

---

## 三、各成员 AI 工具使用场景

### 内核功能实现阶段


1、环境搭建            |Claude | 解决 WSL2 安装、QEMU 编译、Rust 工具链配置报错 | 成功搭建完整开发环境 |
2、理解 ch3 trace 设计 | Claude | 逐行解释 TaskControlBlock 结构和系统调用分发逻辑 | 理解 trace 实现框架 |
3、实现 syscall 计数器 | Claude | 生成 syscall_times 字段添加方案，修复栈溢出 Bug  | ch3 trace 实现完成 |
4、理解 ch4 mmap 原理  | Claude | 解释 Sv39 三级页表、VPN→PPN 映射机制 | 理解虚拟内存映射原理 |
5、实现 mmap/munmap    | Claude | 分析 address_space.map() API，生成权限标志方案| ch4 mmap/munmap 实现完成  |
6、Debug              | Claude  | 粘贴报错信息，分析 PageFault、借用冲突等问题 | 解决多个关键 Bug |
7、改进 Trap 错误信息  | Claude | 生成详细错误输出代码，覆盖 StoreFault/IllegalInstruction 等 | ch2/ch3 错误信息改良完成   |
8、发布 crates.io     | Claude | 生成 tg-rcore-trace crate 结构、Cargo.toml 配置 | 成功发布独立 crate |

### 测试用例与验证阶段


1、设计测试用例 | Claude | 枚举 trace/mmap 的边界条件，生成测试场景列表 | 设计完整测试用例体系 |
2、编写回归测试程序 | Claude | 生成 ch3_trace_extra.rs、ch4_mmap_extra.rs 初始代码 | 新增 2 个回归测试程序 |
3、更新 checker 配置 | Claude | 解释 expected patterns 格式，生成更新配置方案 | checker 成功纳入新测试 |
4、分析测试失败原因 | Claude | 粘贴失败日志，分析 PageFault 是预期还是异常 | 正确区分预期异常与真实错误 |
5、撰写对比分析文档 | Claude | 生成三方环境定量对比表格初稿 | docs/comparison.md 完成 |

### 文档整合与分析阶段

1、概念理解 | Claude | 解释 RISC-V 特权级、SBI、Sv39 页表等核心概念 | 建立 OS 理论知识体系 |
2、Mermaid 图生成 | Claude | 生成实验环境架构图、测试闭环图、学习效率评估图 | 完成全部 Mermaid 图表 |
3、设计报告撰写 | Claude | 生成报告各章节初稿，人工核对数据与逻辑后完善 | docs/design_report.md 完成 |
4、Prompt Engineering 总结 | Claude | 整理本项目使用的 5 类 Prompt 模板 | 形成可复用的 Prompt 方法论 |
5、学习效率评估 | Claude | 辅助估算传统方式与 AI 协作的耗时对比 | 形成量化效率评估数据 |

---

## 四、典型 Prompt 示例

以下列举项目中使用效果最佳的 5 类 Prompt 模板。

### 4.1 概念理解类

```
你是一位熟悉 rCore-Tutorial 的操作系统内核专家。
我正在学习 ch4 的虚拟内存机制，请：
1. 用通俗语言解释 Sv39 三级页表的工作原理
2. 用 ASCII 图示意虚拟地址到物理地址的翻译过程
3. 说明 mmap 系统调用在内核侧需要做哪些操作
```

### 4.2 代码生成类

```
以下是现有的 TaskControlBlock 结构（Rust）：
[粘贴代码]

请在此基础上：
1. 添加 syscall_times: [u32; 512] 字段用于计数
2. 修改 handle_syscall 函数，在分发前对对应编号计数 +1
3. 注意避免栈溢出问题，说明原因并给出解决方案
```

### 4.3 Debug 类

```
我在实现 ch4 mmap 时遇到以下问题：
- mmap 返回 0（成功）
- 但用户态访问该地址触发 Exception(StorePageFault)
- 报错地址：stval = 0x10000000

请分析原因，并给出修复方案。
结合 RISC-V 页表权限位说明。
```

### 4.4 测试设计类

```
我实现了 sys_trace 系统调用，支持以下功能：
- trace_request=0：读取用户内存
- trace_request=1：写入用户内存
- trace_request=2：查询 syscall 调用次数

请帮我生成用户态 Rust 测试程序，覆盖：
- 正常路径测试
- 非法地址测试
- 只读页写入测试
- 解除映射后访问测试
- 非法 trace_request 测试
```

### 4.5 日志分析类

```
以下是 QEMU 运行 ch4 时的输出：
[ERROR] unsupported trap: Exception(StorePageFault), stval = 0x10000000

这是测试 mmap 只读页写入场景时出现的。
请解释：
1. 这是预期现象还是 Bug？
2. checker 测试通过说明了什么？
3. 在报告中应该如何描述这个现象？
```

---



## 五、AI 工具使用的局限性说明

1. AI 生成的代码必须经过人工理解和验证**：本项目所有 AI 生成的代码，均由成员 A 逐行阅读理解后方才提交，未直接盲目使用。

2. 数据真实性保证：报告中的所有测试通过率（ch3 7/7、ch4 16/16 等）均来自实际运行的 `bash test.sh` 输出，未使用 AI 生成或推测的数字。

3. 创新点来自团队思考：trace 系统调用、mmap/munmap、改进 Trap 错误信息等改良方向，均由团队讨论决定，AI 仅负责辅助实现，不是改良方向的来源。

---

## 六、AI 交互记录说明

本项目与 Claude 的完整交互记录保存于以下渠道：

- 主要交互平台：Claude.ai 对话界面（https://claude.ai）
- 交互内容涵盖：环境搭建、代码实现、Bug 修复、文档撰写的全部过程
- 记录完整性：从 ch1 Hello World 到 ch5 进程管理的完整开发过程均有对话记录

如评委需要查看具体交互记录，可联系团队提供导出的对话内容。

---

## 七、总结

本项目以"AI 深度协作"为核心方法论，验证了以下结论：

1. AI 可以显著降低 OS 内核学习门槛，尤其在概念理解和初始代码生成环节，效率提升约 3-4×
2. AI 协作不等于 AI 替代，调试、创新方向决策、数据验证仍需人工主导
3. 有效的 Prompt 设计是关键，明确角色、提供上下文、分步要求是三个最重要的原则
4. AI 协作与文档并重，才能使实验环境真正具备教学价值和可复现性

---

*本报告由成员 贾天纬 负责整合，基于三位成员的实际 AI 使用记录撰写，所有数据真实可查。*
