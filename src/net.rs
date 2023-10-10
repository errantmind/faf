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

const OPTVAL: isize = 1;
const OPTVAL_BUSYPOLL: isize = 50;
const O_NONBLOCK: isize = 2048;
const F_SETFL: isize = 4;

#[inline]
pub fn get_listener_fd(port: u16) -> (isize, sockaddr_in, u32) {
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

      sys_call!(
         SYS_SETSOCKOPT as isize,
         fd_listener,
         SOL_SOCKET as isize,
         SO_REUSEPORT as isize,
         &OPTVAL as *const isize as _,
         size_of_optval as isize
      );

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
   #[allow(clippy::collapsible_if)]
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

// #[inline(always)]
// pub fn debug_incoming_cpu(incoming_fd: isize, listener_fd: isize, cpu_core: i32) {
//    let incoming_cpu: i32 = -1;
//    let incoming_ret = sys_call!(
//       SYS_GETSOCKOPT as isize,
//       incoming_fd,
//       SOL_SOCKET as isize,
//       SO_INCOMING_CPU as isize,
//       &incoming_cpu as *const _ as _,
//       &core::mem::size_of_val(&incoming_cpu) as *const _ as _
//    );

//    let listener_cpu: i32 = -1;
//    let listener_ret = sys_call!(
//       SYS_GETSOCKOPT as isize,
//       listener_fd,
//       SOL_SOCKET as isize,
//       SO_INCOMING_CPU as isize,
//       &listener_cpu as *const _ as _,
//       &core::mem::size_of_val(&listener_cpu) as *const _ as _
//    );

//    println!(
//       "fd: {}, received request on core {} with ret value {} and {}, should be core {}, listener_fd is on core {}",
//       incoming_fd, incoming_cpu, incoming_ret, listener_ret, cpu_core, listener_cpu
//    );
// }
