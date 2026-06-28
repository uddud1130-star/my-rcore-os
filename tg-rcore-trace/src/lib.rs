//! # tg-rcore-trace
//!
//! rCore教学实验环境的trace系统调用扩展库。
//!
//! 提供以下功能：
//! - 系统调用计数器（统计每个syscall的调用次数）
//! - 用户内存读取（trace_request=0）
//! - 用户内存写入（trace_request=1）
//! - syscall计数查询（trace_request=2）
//!
//! ## 使用场景
//!
//! 本库设计用于操作系统教学实验，帮助学生理解：
//! - 系统调用的执行流程
//! - 用户态与内核态的内存访问差异
//! - 特权级保护机制

#![no_std]

/// 系统调用计数器容量
pub const SYSCALL_CAPACITY: usize = 512;

/// Trace请求类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceRequest {
    /// 读取用户内存（返回该地址的字节值）
    ReadMemory = 0,
    /// 写入用户内存（将data写入id地址）
    WriteMemory = 1,
    /// 查询系统调用计数（返回syscall id的调用次数）
    CountSyscall = 2,
}

impl TryFrom<usize> for TraceRequest {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ReadMemory),
            1 => Ok(Self::WriteMemory),
            2 => Ok(Self::CountSyscall),
            _ => Err(()),
        }
    }
}

/// 系统调用计数器
///
/// 记录每个syscall ID被调用的次数，支持最多512个不同的syscall。
pub struct SyscallCounter {
    times: [u32; SYSCALL_CAPACITY],
}

impl SyscallCounter {
    /// 创建新的计数器（全部清零）
    pub const fn new() -> Self {
        Self {
            times: [0; SYSCALL_CAPACITY],
        }
    }

    /// 记录一次系统调用
    pub fn record(&mut self, syscall_id: usize) {
        if syscall_id < SYSCALL_CAPACITY {
            self.times[syscall_id] += 1;
        }
    }

    /// 查询系统调用次数
    pub fn count(&self, syscall_id: usize) -> u32 {
        if syscall_id < SYSCALL_CAPACITY {
            self.times[syscall_id]
        } else {
            0
        }
    }

    /// 重置所有计数
    pub fn reset(&mut self) {
        self.times = [0; SYSCALL_CAPACITY];
    }
}

impl Default for SyscallCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// 处理trace系统调用
///
/// # 参数
/// - `request`：trace请求类型
/// - `id`：地址（读写时）或syscall ID（计数查询时）
/// - `data`：写入的数据（仅WriteMemory时使用）
/// - `counter`：系统调用计数器引用
///
/// # 返回值
/// - ReadMemory：成功返回字节值，失败返回-1
/// - WriteMemory：成功返回0，失败返回-1
/// - CountSyscall：返回调用次数
pub fn handle_trace(
    request: TraceRequest,
    id: usize,
    data: usize,
    counter: &SyscallCounter,
) -> isize {
    match request {
        TraceRequest::ReadMemory => {
            if id == 0 {
                return -1;
            }
            unsafe { *(id as *const u8) as isize }
        }
        TraceRequest::WriteMemory => {
            if id == 0 {
                return -1;
            }
            unsafe { *(id as *mut u8) = data as u8 };
            0
        }
        TraceRequest::CountSyscall => {
            counter.count(id) as isize
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_new() {
        let c = SyscallCounter::new();
        assert_eq!(c.count(64), 0);
    }

    #[test]
    fn test_counter_record() {
        let mut c = SyscallCounter::new();
        c.record(64);
        c.record(64);
        assert_eq!(c.count(64), 2);
    }

    #[test]
    fn test_counter_reset() {
        let mut c = SyscallCounter::new();
        c.record(64);
        c.reset();
        assert_eq!(c.count(64), 0);
    }

    #[test]
    fn test_trace_request_parse() {
        assert_eq!(TraceRequest::try_from(0), Ok(TraceRequest::ReadMemory));
        assert_eq!(TraceRequest::try_from(1), Ok(TraceRequest::WriteMemory));
        assert_eq!(TraceRequest::try_from(2), Ok(TraceRequest::CountSyscall));
        assert!(TraceRequest::try_from(99).is_err());
    }

    #[test]
    fn test_out_of_range() {
        let mut c = SyscallCounter::new();
        c.record(9999);
        assert_eq!(c.count(9999), 0);
    }
}
