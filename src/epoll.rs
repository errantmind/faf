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

use crate::const_config::*;
use crate::const_sys::*;
use crate::http_date;
use crate::http_request_path;
use crate::net;
use crate::sys_call;
use core::intrinsics::{likely, unlikely};
use core::ops::Add;
use core::ops::Sub;

#[repr(C)]
pub union epoll_data {
   pub ptr: isize,
   pub fd: i32,
   pub uint32_t: u32,
   pub uint64_t: u64,
}

#[repr(C, packed)]
pub struct epoll_event {
   pub events: u32,
   pub data: epoll_data,
}

#[repr(align(64))]
struct AlignedHttpDate([u8; 35]);

#[repr(align(64))]
struct AlignedEpollEvents([epoll_event; MAX_EPOLL_EVENTS_RETURNED]);

#[repr(align(64))]
struct AlignedEpollEvent(epoll_event);

#[repr(align(64))]
struct ReqBufAligned([u8; REQ_BUFF_SIZE * MAX_CONN]);

#[repr(align(64))]
struct ResBufAligned([u8; RES_BUFF_SIZE]);


static mut HTTP_DATE: AlignedHttpDate = AlignedHttpDate(http_date::get_buff_with_date());

#[inline(never)]
pub fn go(port: u16, cb: fn(*const u8, usize, *const u8, usize, *mut u8, *const u8) -> usize) {
   // Attempt to set a higher process priority, indicated by a negative number. -20 is the highest possible
   sys_call!(SYS_SETPRIORITY as isize, PRIO_PROCESS as isize, 0, -19);

   // Initialize the DATE before launching workers
   unsafe {
      http_date::get_http_date(&mut HTTP_DATE.0);
   }

   let num_cpu_cores = crate::util::get_num_logical_cpus();
   for core in 0..num_cpu_cores {
      let thread_name = format!("faf{}", core);
      let thread_builder = std::thread::Builder::new().name(thread_name).stack_size(1024 * 1024 * 8);
      let _ = thread_builder.spawn(move || {
         crate::util::set_current_thread_cpu_affinity_to(core);

         // Unshare the file descriptor table between threads to keep the fd number itself low, otherwise all
         // threads will share the same file descriptor table
         sys_call!(SYS_UNSHARE as isize, CLONE_FILES as isize);
         threaded_worker(port, cb, core as i32);
      });
   }

   {
      // Update HTTP date every second

      let sleep_time = http_date::timespec {tv_sec: 1, tv_nsec: 0};
      let sleep_remaining = http_date::timespec {tv_sec: 0, tv_nsec: 0};
      loop {
         unsafe {
            http_date::get_http_date(&mut HTTP_DATE.0);
         }
         sys_call!(SYS_NANOSLEEP as isize, &sleep_time as *const _ as isize, &sleep_remaining as *const _ as isize);
      }
   }

}

#[inline(never)]
fn threaded_worker(port: u16, cb: fn(*const u8, usize, *const u8, usize, *mut u8, *const u8) -> usize, cpu_core: i32) {
   let (listener_fd, _, _) = net::get_listener_fd(port);
   net::setup_connection(listener_fd, cpu_core);

   let epfd = sys_call!(SYS_EPOLL_CREATE1 as isize, 0);

   // Add listener fd to epoll for monitoring
   {
      let epoll_event_listener = AlignedEpollEvent (epoll_event { data: epoll_data { fd: listener_fd as i32 }, events: EPOLLIN });

      sys_call!(
         SYS_EPOLL_CTL as isize,
         epfd,
         EPOLL_CTL_ADD as isize,
         listener_fd,
         &epoll_event_listener.0 as *const epoll_event as isize
      );
   }

   let epoll_events: AlignedEpollEvents = unsafe { core::mem::zeroed() };
   let epoll_events_ptr = &epoll_events.0[0] as *const _ as isize;

   let mut saved_event: AlignedEpollEvent = unsafe { core::mem::zeroed() };
   saved_event.0.events = EPOLLIN;

   let mut reqbuf: ReqBufAligned = unsafe { core::mem::zeroed() };

   // Init state for tracking request buffer position across events
   let mut reqbuf_cur_addr: [isize; MAX_CONN] = unsafe { core::mem::zeroed() };
   {
      let reqbuf_start_address = &mut reqbuf.0[0] as *mut _ as isize;
      (0..reqbuf_cur_addr.len()).for_each(|i| {
         reqbuf_cur_addr[i] = reqbuf_start_address + i as isize * REQ_BUFF_SIZE as isize;
      });
   }

   // If we receive part of a request due to a split read, we track how many bytes were unparsable so
   // this number of bytes can be prepended to the next time we try to parse the buffer
   let mut reqbuf_residual: [usize; MAX_CONN] = unsafe { core::mem::zeroed() };

   let mut resbuf: ResBufAligned = unsafe { core::mem::zeroed() };
   let resbuf_start_address = &mut resbuf.0[0] as *mut _ as isize;

   loop {
      let num_incoming_events = sys_call!(
         SYS_EPOLL_WAIT as isize,
         epfd,
         epoll_events_ptr,
         MAX_EPOLL_EVENTS_RETURNED as isize,
         EPOLL_TIMEOUT_MILLIS
      );

      for index in 0..num_incoming_events {
         let cur_fd = unsafe { (epoll_events.0.get_unchecked(index as usize)).data.fd } as isize;
         let req_buf_start_address = (&mut reqbuf.0[0] as *const u8 as isize).add(cur_fd * REQ_BUFF_SIZE as isize);
         let req_buf_cur_position = unsafe { reqbuf_cur_addr.get_unchecked_mut(cur_fd as usize) };
         let residual = unsafe { reqbuf_residual.get_unchecked_mut(cur_fd as usize) };

         if cur_fd == listener_fd {
            let incoming_fd = sys_call!(SYS_ACCEPT as isize, listener_fd, 0, 0);

            if likely(incoming_fd >= 0 && incoming_fd < MAX_CONN as isize) {
               *req_buf_cur_position = req_buf_start_address;
               *residual = 0;
               net::setup_connection(incoming_fd, cpu_core);
               saved_event.0.data.fd = incoming_fd as i32;

               sys_call!(
                  SYS_EPOLL_CTL as isize,
                  epfd,
                  EPOLL_CTL_ADD as isize,
                  incoming_fd,
                  &saved_event.0 as *const epoll_event as isize
               );
            } else {
               net::close_connection(epfd, cur_fd);
            }
         } else {
            let buffer_remaining = REQ_BUFF_SIZE as isize - (*req_buf_cur_position - req_buf_start_address);

            let read = sys_call!(SYS_RECVFROM as isize, cur_fd, *req_buf_cur_position, buffer_remaining, 0, 0, 0);

            if likely(read > 0) {
               let mut request_buffer_offset = 0;
               let mut response_buffer_filled_total = 0;

               while request_buffer_offset != (read + *residual as isize) {
                  let mut method: *const i8 = core::ptr::null_mut();
                  let mut method_len = 0;
                  let mut path: *const i8 = core::ptr::null_mut();
                  let mut path_len = 0;

                  let request_buffer_bytes_parsed = unsafe {
                     http_request_path::parse_request_path_pipelined_simd(
                        req_buf_cur_position.sub(*residual as isize).add(request_buffer_offset) as *const _,
                        read as usize + *residual - request_buffer_offset as usize,
                        &mut method,
                        &mut method_len,
                        &mut path,
                        &mut path_len,
                     )
                  };

                  if request_buffer_bytes_parsed > 0 {
                     request_buffer_offset += request_buffer_bytes_parsed;

                     let response_buffer_filled = unsafe {
                        cb(
                           method as *const u8,
                           method_len,
                           path as *const u8,
                           path_len,
                           resbuf_start_address.add(response_buffer_filled_total) as *mut _,
                           HTTP_DATE.0.as_ptr(),
                        )
                     };

                     response_buffer_filled_total += response_buffer_filled as isize;
                  } else {
                     break;
                  }
               }

               if request_buffer_offset == 0 || response_buffer_filled_total == 0 {
                  *req_buf_cur_position = req_buf_start_address;
                  *residual = 0;
                  net::close_connection(epfd, cur_fd);
                  continue;
               } else if request_buffer_offset == (read + *residual as isize) {
                  *req_buf_cur_position = req_buf_start_address;
                  *residual = 0;
               } else {
                  *req_buf_cur_position += read;
                  *residual += (read - request_buffer_offset) as usize;
               }

               // let wrote = sys_call!(
               //    SYS_WRITE as isize,
               //    cur_fd,
               //    resbuf_start_address,
               //    response_buffer_filled_total
               // );

               let wrote = sys_call!(
                  SYS_SENDTO as isize,
                  cur_fd,
                  resbuf_start_address,
                  response_buffer_filled_total,
                  0,
                  0,
                  0
               );

               if likely(wrote == response_buffer_filled_total) {
               } else if unlikely(-wrote == EAGAIN as isize || -wrote == EINTR as isize) {
                  *req_buf_cur_position = req_buf_start_address;
                  *residual = 0;
                  net::close_connection(epfd, cur_fd);
                  break;
               } else {
                  *req_buf_cur_position = req_buf_start_address;
                  *residual = 0;
                  net::close_connection(epfd, cur_fd);
                  continue;
               }
            } else if unlikely(-read == EAGAIN as isize || -read == EINTR as isize) {
            } else {
               *req_buf_cur_position = req_buf_start_address;
               *residual = 0;
               net::close_connection(epfd, cur_fd);
            }
         }
      }
   }
}
