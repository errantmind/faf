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

use crate::const_config::MAX_CONN;
use crate::const_sys::*;
use crate::sys_call;

#[inline(always)]
pub fn htons(u: u16) -> u16 {
   u.to_be()
}

#[inline(always)]
pub fn htonl(u: u32) -> u32 {
   u.to_be()
}

#[repr(C)]
pub struct in_addr {
   pub s_addr: u32,
}

#[repr(C, align(16))]
pub struct sockaddr_in {
   pub sin_family: u16,
   pub sin_port: u16,
   pub sin_addr: in_addr,
   pub sin_zero: [u8; 8],
}

#[repr(C, align(16))]
pub struct linger {
   pub l_onoff: i32,
   pub l_linger: i32,
}

#[repr(C)]
pub struct sock_filter {
   pub code: u16,
   pub jt: u8,
   pub jf: u8,
   pub k: u32,
}

#[repr(C)]
pub struct sock_fprog {
   pub len: u16,
   pub filter: *mut sock_filter,
}

const OPTVAL: isize = 1;
const OPTVAL_BUSYPOLL: isize = 50;
pub const O_NONBLOCK: isize = 2048;
const F_SETFL: isize = 4;

#[inline]
pub fn get_listener_fd(port: u16, cpu_core: i32) -> (isize, sockaddr_in, u32) {
   const _OPTVAL_TCPDEFERACCEPT_TIMEOUT: isize = 10;
   const OPTVAL_TCPFASTOPEN_QUEUE_LEN: isize = MAX_CONN as isize;
   const AF_INET: i32 = 2;

   unsafe {
      let fd_listener = sys_call!(SYS_SOCKET as isize, AF_INET as isize, SOCK_STREAM as isize, 0);
      let size_of_optval = core::mem::size_of_val(&OPTVAL) as u32;

      sys_call!(
         SYS_SETSOCKOPT as isize,
         fd_listener,
         SOL_SOCKET as isize,
         SO_REUSEADDR as isize,
         &OPTVAL as *const _ as _,
         size_of_optval as isize
      );

      // https://elixir.bootlin.com/linux/latest/source/include/uapi/linux/bpf_common.h
      // BPF_CLASS
      const BPF_LD: u16 = 0x00;
      const BPF_LDX: u16 = 0x01;
      const BPF_ST: u16 = 0x02;
      const BPF_STX: u16 = 0x03;
      const BPF_ALU: u16 = 0x04;
      const BPF_JMP: u16 = 0x05;
      const BPF_RET: u16 = 0x06;
      const BPF_MISC: u16 = 0x07;

      // BPF_SIZE
      const BPF_W: u16 = 0x00;
      const BPF_H: u16 = 0x08;
      const BPF_B: u16 = 0x10;

      // BPF_MODE
      const BPF_IMM: u16 = 0x00;
      const BPF_ABS: u16 = 0x20;
      const BPF_IND: u16 = 0x40;
      const BPF_MEM: u16 = 0x60;
      const BPF_LEN: u16 = 0x80;
      const BPF_MSH: u16 = 0xa0;

      // BPF_OP
      const BPF_ADD: u16 = 0x00;
      const BPF_SUB: u16 = 0x10;
      const BPF_MUL: u16 = 0x20;
      const BPF_DIV: u16 = 0x30;
      const BPF_OR: u16 = 0x40;
      const BPF_AND: u16 = 0x50;
      const BPF_LSH: u16 = 0x60;
      const BPF_RSH: u16 = 0x70;
      const BPF_NEG: u16 = 0x80;
      const BPF_MOD: u16 = 0x90;
      const BPF_XOR: u16 = 0xa0;

      const BPF_JA: u16 = 0x00;
      const BPF_JEQ: u16 = 0x10;
      const BPF_JGT: u16 = 0x20;
      const BPF_JGE: u16 = 0x30;
      const BPF_JSET: u16 = 0x40;

      // BPF_SRC
      const BPF_K: u16 = 0x00;
      const BPF_X: u16 = 0x08;

      // https://elixir.bootlin.com/linux/latest/source/include/uapi/linux/filter.h
      // BPF_RVAL
      const BPF_A: u16 = 0x10;

      // SKF
      const SKF_AD_OFF: i32 = -0x1000;
      const SKF_AD_PROTOCOL: i32 = 0;
      const SKF_AD_PKTTYPE: i32 = 4;
      const SKF_AD_IFINDEX: i32 = 8;
      const SKF_AD_NLATTR: i32 = 12;
      const SKF_AD_NLATTR_NEST: i32 = 16;
      const SKF_AD_MARK: i32 = 20;
      const SKF_AD_QUEUE: i32 = 24;
      const SKF_AD_HATYPE: i32 = 28;
      const SKF_AD_RXHASH: i32 = 32;
      const SKF_AD_CPU: i32 = 36;
      const SKF_AD_ALU_XOR_X: i32 = 40;
      const SKF_AD_VLAN_TAG: i32 = 44;
      const SKF_AD_VLAN_TAG_PRESENT: i32 = 48;
      const SKF_AD_PAY_OFFSET: i32 = 52;
      const SKF_AD_RANDOM: i32 = 56;
      const SKF_AD_VLAN_TPID: i32 = 60;
      const SKF_AD_MAX: i32 = 64;

      const SKF_NET_OFF: i32 = -0x100000;
      const SKF_LL_OFF: i32 = -0x200000;

      const BPF_NET_OFF: i32 = SKF_NET_OFF;
      const BPF_LL_OFF: i32 = SKF_LL_OFF;

      sys_call!(
         SYS_SETSOCKOPT as isize,
         fd_listener,
         SOL_SOCKET as isize,
         SO_REUSEPORT as isize,
         &OPTVAL as *const isize as _,
         size_of_optval as isize
      );

      let mut code: [sock_filter; 2] = [
         sock_filter { code: BPF_LD | BPF_W | BPF_ABS, jt: 0, jf: 0, k: (SK_D_FD_SD) as u32 },
         sock_filter { code: BPF_RET | BPF_A, jt: 0, jf: 0, k: 0 },
      ];
      let prog = sock_fprog { len: code.len() as u16, filter: code.as_mut_ptr() };

      let ret = sys_call!(
         SYS_SETSOCKOPT as isize,
         fd_listener,
         SOL_SOCKET as isize,
         SO_ATTACH_REUSEPORT_CBPF as isize,
         &prog as *const _ as _,
         core::mem::size_of::<sock_fprog>() as isize
      );

      println!("SO_ATTACH_REUSEPORT_CBPF ret: {}, size = {}", ret, core::mem::size_of::<sock_fprog>() as isize);

      // sys_call!(
      //    SYS_SETSOCKOPT as isize,
      //    fd_listener,
      //    SOL_SOCKET as isize,
      //    SO_REUSEPORT as isize,
      //    &OPTVAL as *const isize as _,
      //    size_of_optval as isize
      // );

      sys_call!(
         SYS_SETSOCKOPT as isize,
         fd_listener,
         IPPROTO_TCP as isize,
         TCP_QUICKACK as isize,
         &OPTVAL as *const _ as _,
         core::mem::size_of_val(&OPTVAL) as isize
      );

      sys_call!(
         SYS_SETSOCKOPT as isize,
         fd_listener,
         IPPROTO_TCP as isize,
         TCP_FASTOPEN as isize,
         &MAX_CONN as *const _ as _,
         core::mem::size_of_val(&MAX_CONN) as isize
      );

      // Does not add much throughput, if any. Also, can hide dead connections. Not useful.
      // sys_call!(
      //    SYS_SETSOCKOPT as isize,
      //    fd_listener,
      //    IPPROTO_TCP as isize,
      //    TCP_DEFER_ACCEPT as isize,
      //    &_OPTVAL_TCPDEFERACCEPT_TIMEOUT as *const _ as _,
      //    core::mem::size_of_val(&_OPTVAL_TCPDEFERACCEPT_TIMEOUT) as isize
      // );

      //https://stackoverflow.com/a/49900878
      // sys_call!(
      //    SYS_SETSOCKOPT as isize,
      //    fd_listener,
      //    SOL_SOCKET as isize,
      //    SO_ZEROCOPY as isize,
      //    &OPTVAL as *const _ as _,
      //    core::mem::size_of_val(&OPTVAL) as isize
      // );

      let addr = sockaddr_in {
         sin_family: AF_INET as u16,
         sin_port: htons(port),
         sin_addr: in_addr { s_addr: htonl(INADDR_ANY) },
         sin_zero: core::mem::zeroed(),
      };

      sys_call!(SYS_BIND as isize, fd_listener, &addr as *const _ as _, core::mem::size_of_val(&addr) as isize);

      sys_call!(SYS_LISTEN as isize, fd_listener, OPTVAL_TCPFASTOPEN_QUEUE_LEN);

      let sock_len: u32 = core::mem::size_of::<sockaddr_in>() as u32;
      (fd_listener, addr, sock_len)
   }
}

#[inline]
pub fn setup_connection(fd: isize, core: i32) {
   //Doesn't help with throughput, just latency per request, and may actually reduce throughput.
   //May be Useful for this test. I'm not entirely convinced though
   sys_call!(
      SYS_SETSOCKOPT as isize,
      fd,
      IPPROTO_TCP as isize,
      TCP_NODELAY as isize,
      &OPTVAL as *const _ as _,
      core::mem::size_of_val(&OPTVAL) as isize
   );

   sys_call!(
      SYS_SETSOCKOPT as isize,
      fd,
      IPPROTO_TCP as isize,
      TCP_QUICKACK as isize,
      &OPTVAL as *const _ as _,
      core::mem::size_of_val(&OPTVAL) as isize
   );

   // This can be disabled if we are passed, say, '-1' for times we don't want to assign a core affinity
   if core >= 0 {
      sys_call!(
         SYS_SETSOCKOPT as isize,
         fd,
         SOL_SOCKET as isize,
         SO_INCOMING_CPU as isize,
         &core as *const _ as _,
         core::mem::size_of_val(&core) as isize
      );
   }

   //https://stackoverflow.com/a/49900878
   // sys_call!(
   //    SYS_SETSOCKOPT as isize,
   //    fd as isize,
   //    SOL_SOCKET as isize,
   //    SO_ZEROCOPY as isize,
   //    &OPTVAL as *const isize as _,
   //    core::mem::size_of_val(&OPTVAL) as isize
   // );

   // Only useful when using blocking reads, not non-blocking reads as I am
   // sys_call!(
   //    SYS_SETSOCKOPT as isize,
   //    fd,
   //    SOL_SOCKET as isize,
   //    SO_BUSY_POLL as isize,
   //    &OPTVAL_BUSYPOLL as *const _ as _,
   //    core::mem::size_of_val(&OPTVAL_BUSYPOLL) as isize
   // );

   sys_call!(SYS_FCNTL as isize, fd, F_SETFL, O_NONBLOCK);
}

#[inline]
pub fn set_blocking(fd: isize) {
   sys_call!(SYS_FCNTL as isize, fd, F_SETFL, 0);
}

#[inline]
pub fn set_nonblocking(fd: isize) {
   sys_call!(SYS_FCNTL as isize, fd, F_SETFL, O_NONBLOCK);
}

#[inline(always)]
pub fn close_connection(epfd: isize, fd: isize) {
   const OPTVAL_SOLINGER_TIMEOUT: linger = linger { l_onoff: 1, l_linger: 0 };
   sys_call!(
      SYS_SETSOCKOPT as isize,
      fd,
      SOL_SOCKET as isize,
      SO_LINGER as isize,
      &OPTVAL_SOLINGER_TIMEOUT as *const _ as _,
      core::mem::size_of_val(&OPTVAL_SOLINGER_TIMEOUT) as isize
   );

   // Could defer deletes for performance reasons. Wouldn't cause problems as fds are reused. Not going to do this as it would
   // require tracking fd state more granually to avoid the wasted EPOLL_CTL_ADD when new connections come in. I don't know
   // how much of a benefit, performance-wise I'd get
   sys_call!(SYS_EPOLL_CTL as isize, epfd, EPOLL_CTL_DEL as isize, fd, 0);

   sys_call!(SYS_CLOSE as isize, fd);
}

#[inline(always)]
pub fn debug_incoming_cpu(incoming_fd: isize, listener_fd: isize, cpu_core: i32) {
   let incoming_cpu: i32 = -1;
   let incoming_ret = sys_call!(
      SYS_GETSOCKOPT as isize,
      incoming_fd,
      SOL_SOCKET as isize,
      SO_INCOMING_CPU as isize,
      &incoming_cpu as *const _ as _,
      &core::mem::size_of_val(&incoming_cpu) as *const _ as _
   );

   let listener_cpu: i32 = -1;
   let listener_ret = sys_call!(
      SYS_GETSOCKOPT as isize,
      listener_fd,
      SOL_SOCKET as isize,
      SO_INCOMING_CPU as isize,
      &listener_cpu as *const _ as _,
      &core::mem::size_of_val(&listener_cpu) as *const _ as _
   );

   let incoming_napi_id: i32 = -1;
   let incoming_napi_id_ret = sys_call!(
      SYS_GETSOCKOPT as isize,
      incoming_fd,
      SOL_SOCKET as isize,
      SO_INCOMING_NAPI_ID as isize,
      &incoming_napi_id as *const _ as _,
      &core::mem::size_of_val(&incoming_napi_id) as *const _ as _
   );

   println!(
      "fd: {}, received request on core {} with ret value {} | {} | {}, should be core {}, listener_fd is on core {} with napi id {}",
      incoming_fd, incoming_cpu, incoming_ret, listener_ret, incoming_napi_id_ret, cpu_core, listener_cpu, incoming_napi_id
   );
}
