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

// _mm_prefetch
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[inline]
fn prefetch(p: *const u8, offset: isize) {
   unsafe { core::intrinsics::prefetch_read_data(p.offset(offset) as *const u8, 3) };
}
