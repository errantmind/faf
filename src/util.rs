/*
FaF is a cutting edge, high performance web server
Copyright (C) 2021  James Bates

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use crate::const_sys::*;
use crate::http_content_length;
use crate::sys_call;

const DOT: u8 = b'.';

#[derive(Debug)]
#[repr(C)]
pub struct rlimit {
   pub rlim_cur: u32, /* Soft limit */
   pub rlim_max: u32, /* Hard limit */
}

const _SC_NPROCESSORS_ONLN: i32 = 84;

extern "C" {
   pub fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;
   fn sysconf(name: i32) -> isize;

   fn sched_getaffinity(pid: i32, cpusetsize: usize, cpuset: *mut cpu_set_t) -> i32;
   fn sched_setaffinity(pid: i32, cpusetsize: usize, cpuset: *const cpu_set_t) -> i32;
}

const POINTER_WIDTH_IN_BITS: usize = core::mem::size_of::<usize>() * 8;

// We always want a total of 1024 bits, so 16 segments on 64-bit platforms, 32 segments on 32-bit platforms
const CPU_SET_LEN: usize = 1024 / POINTER_WIDTH_IN_BITS;

#[repr(C, align(64))]
struct cpu_set_t([usize; CPU_SET_LEN]);

#[inline]
fn cpu_isset(cpu_num: usize, set: &cpu_set_t) -> bool {
   let chunk_index = cpu_num / POINTER_WIDTH_IN_BITS;
   let chunk_offset = cpu_num % POINTER_WIDTH_IN_BITS;
   ((1 << chunk_offset) & set.0[chunk_index]) != 0
}

#[inline]
fn cpu_set(cpu_num: usize, set: &mut cpu_set_t) {
   let chunk_index = cpu_num / POINTER_WIDTH_IN_BITS;
   let chunk_offset = cpu_num % POINTER_WIDTH_IN_BITS;
   set.0[chunk_index] |= 1 << chunk_offset;
}

// 0 indicates the current thread's PID for this API
const CURRENT_THREAD_CONTROL_PID: i32 = 0;

#[inline]
pub fn set_current_thread_cpu_affinity_to(cpu_num: usize) {
   let mut set: cpu_set_t = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
   unsafe { sched_getaffinity(CURRENT_THREAD_CONTROL_PID, core::mem::size_of::<cpu_set_t>(), &mut set) };
   if !cpu_isset(cpu_num, &set) {
      eprintln!("Cannot set affinity for cpu {}", cpu_num);
   } else {
      let mut set_control: cpu_set_t = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
      cpu_set(cpu_num, &mut set_control);
      unsafe { sched_setaffinity(0, core::mem::size_of::<cpu_set_t>(), &set_control) };
   }
}

#[inline]
pub fn get_num_logical_cpus() -> usize {
   let cpus = unsafe { sysconf(_SC_NPROCESSORS_ONLN) };
   if cpus <= 0 {
      eprintln!("Cannot determine the number of logical cpus with sysconf, performance will be severely impacted");
      1
   } else {
      cpus as usize
   }
}

#[inline]
pub const unsafe fn transmute<From, To>(from: From) -> To {
   union Transmute<From, To> {
      from: core::mem::ManuallyDrop<From>,
      to: core::mem::ManuallyDrop<To>,
   }

   core::mem::ManuallyDrop::into_inner(Transmute { from: core::mem::ManuallyDrop::new(from) }.to)
}

#[inline]
pub const unsafe fn concat<First, Second, Out>(a: &[u8], b: &[u8]) -> Out
where
   First: Copy,
   Second: Copy,
   Out: Copy,
{
   #[repr(C)]
   #[derive(Copy, Clone)]
   struct Both<A, B>(A, B);

   let arr: Both<First, Second> =
      Both(*transmute::<_, *const First>(a.as_ptr()), *transmute::<_, *const Second>(b.as_ptr()));

   transmute(arr)
}

#[inline]
pub const fn const_len<T>(con: &[T]) -> usize {
   con.len()
}

/// _mm_prefetch
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[inline]
fn prefetch(p: *const u8, offset: isize) {
   debug_assert_ne!(p as usize, 0);
   debug_assert!(offset >= 0);

   unsafe { core::intrinsics::prefetch_read_data(p.offset(offset) as *const u8, 3) };
}

#[inline]
pub fn get_limits(resource: u32, limits: &mut rlimit) -> isize {
   debug_assert!(resource <= 16);

   sys_call!(SYS_GETRLIMIT as isize, resource as isize, &limits as *const _ as _)
}

#[inline]
pub fn set_limits(resource: u32, value: u32) -> isize {
   debug_assert!(resource <= 16);

   let mut limits: rlimit = unsafe { core::mem::MaybeUninit::uninit().assume_init() };

   let ret = get_limits(resource, &mut limits);

   if ret < 0 {
      panic!("Failed to fetch limits for resource {}", resource);
   }

   limits.rlim_cur = value;
   sys_call!(SYS_SETRLIMIT as isize, resource as isize, &limits as *const _ as _)
}

/// Attempt to set a higher process priority. -20 is the highest we can set.
/// Threads inherit this priority
#[inline]
pub fn set_maximum_process_priority() {
   sys_call!(SYS_SETPRIORITY as isize, PRIO_PROCESS as isize, 0, -20);
}

/// Unshare the file descriptor table between threads to keep the fd number itself low, otherwise all
/// threads will share the same file descriptor table. A single file descriptor table is problematic if
/// we use file descriptors to index data structures
pub fn unshare_file_descriptors() {
   sys_call!(SYS_UNSHARE as isize, CLONE_FILES as isize);
}

/// Converts the internet host which is in network byte order, represented as a 32bit int
/// to a str (in a byte buffer) in IPv4 dotted-decimal notation.
///
/// A maximum of 15 bytes (XXX.XXX.XXX.XXX) are written to the output buffer which is enough to represent any
/// valid IPV4 address from 0.0.0.0 to 255.255.255.255
///
/// Most often, the first argument will be a `sockaddr_in.sin_addr.s_addr`
///
/// Example:
///
/// ```
/// use faf::util::inet_ntoa;
///
/// unsafe {
///    let mut ip_buff: [u8; 15] = core::mem::zeroed();
///    let ip_buff_ptr = &mut ip_buff as *mut u8;
///    let ip_buff_len = inet_ntoa(16777343u32, ip_buff_ptr);
///
///    // Optional conversion to &str
///    let ip_slice = core::slice::from_raw_parts(ip_buff_ptr, ip_buff_len);
///    let ip_str = core::str::from_utf8_unchecked(ip_slice);
///    println!("{} -> {}", 16777343u32, ip_str);
///
///    // Trim the null characters off the end of the &str
///    assert_eq!(ip_str, "127.0.0.1");
/// }
/// ```

#[inline]
pub fn inet_ntoa(s_addr: u32, out_buff_start: *mut u8) -> usize {
   debug_assert!(out_buff_start as usize != 0);

   unsafe {
      let s_addr_start_ptr = &s_addr as *const u32 as *const u8;
      let mut output_byte_walker = out_buff_start;

      output_byte_walker = output_byte_walker.add(http_content_length::u8toa(output_byte_walker, *s_addr_start_ptr));

      *output_byte_walker = DOT;
      output_byte_walker = output_byte_walker.add(1);

      output_byte_walker =
         output_byte_walker.add(http_content_length::u8toa(output_byte_walker, *(s_addr_start_ptr.add(1))));

      *output_byte_walker = DOT;
      output_byte_walker = output_byte_walker.add(1);

      output_byte_walker =
         output_byte_walker.add(http_content_length::u8toa(output_byte_walker, *(s_addr_start_ptr.add(2))));

      *output_byte_walker = DOT;
      output_byte_walker = output_byte_walker.add(1);

      output_byte_walker =
         output_byte_walker.add(http_content_length::u8toa(output_byte_walker, *(s_addr_start_ptr.add(3))));

      output_byte_walker as usize - out_buff_start as usize
   }
}

#[test]
fn test_inet_ntoa() {
   unsafe {
      const TEST1: u32 = 16777343;
      let mut ip_buff: [u8; 15] = core::mem::zeroed();
      let ip_buff_ptr = &mut ip_buff as *mut u8;
      let len = inet_ntoa(TEST1, ip_buff_ptr);
      let byte_slice = core::slice::from_raw_parts(ip_buff_ptr, len);
      let ip_str = core::str::from_utf8_unchecked(byte_slice);
      assert_eq!(ip_str, "127.0.0.1");

      const TEST2: u32 = 3170937024;
      let mut ip_buff: [u8; 15] = core::mem::zeroed();
      let ip_buff_ptr = &mut ip_buff as *mut u8;
      let len = inet_ntoa(TEST2, ip_buff_ptr);
      let byte_slice = core::slice::from_raw_parts(ip_buff_ptr, len);
      let ip_str = core::str::from_utf8_unchecked(byte_slice);
      assert_eq!(ip_str, "192.168.0.189");
   }
}
