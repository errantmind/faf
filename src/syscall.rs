// This particular file is LICENSED AS Unlicense (https://unlicense.org/)

// To use this yourself, see  https://crates.io/crates/faf-syscall

use core::arch::asm;

#[macro_export(local_inner_macros)]
macro_rules! sys_call {
   ($num:ident) => {
      $crate::syscall::sys_call0($num)
   };
   ($num:expr, $arg1:expr) => {
      $crate::syscall::sys_call1($num, $arg1)
   };
   ($num:expr, $arg1:expr, $arg2:expr) => {
      $crate::syscall::sys_call2($num, $arg1, $arg2)
   };
   ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr) => {
      $crate::syscall::sys_call3($num, $arg1, $arg2, $arg3)
   };
   ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) => {
      $crate::syscall::sys_call4($num, $arg1, $arg2, $arg3, $arg4)
   };
   ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) => {
      $crate::syscall::sys_call5($num, $arg1, $arg2, $arg3, $arg4, $arg5)
   };
   ($num:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) => {
      $crate::syscall::sys_call6($num, $arg1, $arg2, $arg3, $arg4, $arg5, $arg6)
   };
}

#[inline(always)]
pub fn sys_call0(mut num: isize) -> isize {
   unsafe {
      asm!(
         "syscall",
         in("rax") num,
         out("rcx") _,
         out("r11") _,
         lateout("rax") num,
         options(nostack));

      num
   }
}

#[inline(always)]
pub fn sys_call1(mut num: isize, arg1: isize) -> isize {
   unsafe {
      asm!(
         "syscall",
         in("rax") num,
         in("rdi") arg1,
         out("rcx") _,
         out("r11") _,
         lateout("rax") num,
         options(nostack));
      num
   }
}

#[inline(always)]
pub fn sys_call2(mut num: isize, arg1: isize, arg2: isize) -> isize {
   unsafe {
      asm!(
         "syscall",
         in("rax") num,
         in("rdi") arg1,
         in("rsi") arg2,
         out("rcx") _,
         out("r11") _,
         lateout("rax") num,
         options(nostack));
      num
   }
}

#[inline(always)]
pub fn sys_call3(mut num: isize, arg1: isize, arg2: isize, arg3: isize) -> isize {
   unsafe {
      asm!(
         "syscall",
         in("rax") num,
         in("rdi") arg1,
         in("rsi") arg2,
         in("rdx") arg3,
         out("rcx") _,
         out("r11") _,
         lateout("rax") num,
         options(nostack));
      num
   }
}

#[inline(always)]
pub fn sys_call4(mut num: isize, arg1: isize, arg2: isize, arg3: isize, arg4: isize) -> isize {
   unsafe {
      asm!(
         "syscall",
         in("rax") num,
         in("rdi") arg1,
         in("rsi") arg2,
         in("rdx") arg3,
         in("r10") arg4,
         out("rcx") _,
         out("r11") _,
         lateout("rax") num,
         options(nostack));
      num
   }
}

#[inline(always)]
pub fn sys_call5(mut num: isize, arg1: isize, arg2: isize, arg3: isize, arg4: isize, arg5: isize) -> isize {
   unsafe {
      asm!(
         "syscall",
         in("rax") num,
         in("rdi") arg1,
         in("rsi") arg2,
         in("rdx") arg3,
         in("r10") arg4,
         in("r8") arg5,
         out("rcx") _,
         out("r11") _,
         lateout("rax") num,
         options(nostack));
      num
   }
}

#[inline(always)]
pub fn sys_call6(
   mut num: isize,
   arg1: isize,
   arg2: isize,
   arg3: isize,
   arg4: isize,
   arg5: isize,
   arg6: isize,
) -> isize {
   unsafe {
      asm!(
         "syscall",
         in("rax") num,
         in("rdi") arg1,
         in("rsi") arg2,
         in("rdx") arg3,
         in("r10") arg4,
         in("r8") arg5,
         in("r9") arg6,
         out("rcx") _,
         out("r11") _,
         lateout("rax") num,
         options(nostack));
      num
   }
}
