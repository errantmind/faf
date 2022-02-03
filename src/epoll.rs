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

static mut HTTP_DATE: [u8; 35] = http_date::get_buff_with_date();

#[inline(never)]
pub fn go(port: u16, cb: fn(*const u8, usize, *const u8, usize, *mut u8, *const u8) -> usize) {
   // Attempt to set a higher process priority, indicated by a negative number. -20 is the highest possible
   sys_call!(SYS_SETPRIORITY as isize, PRIO_PROCESS as isize, 0, -19);

   // Initialize the DATE before launching workers
   unsafe {
      http_date::get_http_date(&mut HTTP_DATE);
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

   loop {
      unsafe {
         http_date::get_http_date(&mut HTTP_DATE);
      }
      std::thread::sleep(core::time::Duration::from_secs(1));
   }
}

#[inline(never)]
fn threaded_worker(port: u16, cb: fn(*const u8, usize, *const u8, usize, *mut u8, *const u8) -> usize, cpu_core: i32) {
   let (listener_fd, _, _) = net::get_listener_fd(port);
   net::setup_connection(listener_fd, cpu_core);

   let epfd = sys_call!(SYS_EPOLL_CREATE1 as isize, 0);

   // Add listener fd to epoll for monitoring
   {
      let epoll_event_listener = epoll_event { data: epoll_data { fd: listener_fd as i32 }, events: EPOLLIN };

      sys_call!(
         SYS_EPOLL_CTL as isize,
         epfd,
         EPOLL_CTL_ADD as isize,
         listener_fd as isize,
         &epoll_event_listener as *const epoll_event as isize
      );
   }

   let epoll_events: [epoll_event; MAX_EPOLL_EVENTS_RETURNED] = unsafe { core::mem::zeroed() };
   let epoll_events_ptr = &epoll_events as *const _ as isize;

   let mut saved_event: epoll_event = unsafe { core::mem::zeroed() };
   saved_event.events = EPOLLIN;

   let mut reqbuf: [u8; REQ_BUFF_SIZE * MAX_CONN] = unsafe { core::mem::zeroed() };

   // Init state for tracking request buffer position across events
   let mut reqbuf_cur_addr: [isize; MAX_CONN] = unsafe { core::mem::zeroed() };
   {
      let reqbuf_start_address = &mut reqbuf[0] as *mut _ as isize;
      (0..reqbuf_cur_addr.len()).for_each(|i| {
         reqbuf_cur_addr[i] = reqbuf_start_address + i as isize * REQ_BUFF_SIZE as isize;
      });
   }

   let mut resbuf: [u8; RES_BUFF_SIZE] = unsafe { core::mem::zeroed() };
   let resbuf_start_address = &mut resbuf[0] as *mut _ as isize;

   loop {
      let num_incoming_events = sys_call!(
         SYS_EPOLL_WAIT as isize,
         epfd,
         epoll_events_ptr,
         MAX_EPOLL_EVENTS_RETURNED as isize,
         EPOLL_TIMEOUT_MILLIS
      );

      for index in 0..num_incoming_events {
         let cur_fd = unsafe { (*epoll_events.get_unchecked(index as usize)).data.fd } as isize;
         let req_buf_start_address = (&mut reqbuf[0] as *const u8 as isize).add(cur_fd * REQ_BUFF_SIZE as isize);
         let req_buf_cur_position = unsafe { reqbuf_cur_addr.get_unchecked_mut(cur_fd as usize) };

         if cur_fd == listener_fd {
            let incoming_fd = sys_call!(SYS_ACCEPT as isize, listener_fd as isize, 0, 0);

            if likely(incoming_fd >= 0 && incoming_fd < MAX_CONN as isize) {
               *req_buf_cur_position = req_buf_start_address;
               net::setup_connection(incoming_fd, cpu_core as i32);
               saved_event.data.fd = incoming_fd as i32;

               sys_call!(
                  SYS_EPOLL_CTL as isize,
                  epfd,
                  EPOLL_CTL_ADD as isize,
                  incoming_fd as isize,
                  &saved_event as *const epoll_event as isize
               );
            } else {
               net::close_connection(epfd, cur_fd as isize);
            }
         } else {
            let buffer_used = *req_buf_cur_position - req_buf_start_address;
            let buffer_remaining = REQ_BUFF_SIZE as isize - buffer_used;

            let read = sys_call!(SYS_READ as isize, cur_fd, *req_buf_cur_position, buffer_remaining);

            if likely(read > 0) {
               let mut request_buffer_offset = 0;
               let mut response_buffer_filled_total = 0;

               while request_buffer_offset != (read + buffer_used) {
                  let mut method: *const i8 = core::ptr::null_mut();
                  let mut method_len = 0;
                  let mut path: *const i8 = core::ptr::null_mut();
                  let mut path_len = 0;

                  let request_buffer_bytes_parsed = unsafe {
                     http_request_path::parse_request_path_pipelined_simd(
                        req_buf_start_address.add(request_buffer_offset) as *const _,
                        read as usize + buffer_used as usize - request_buffer_offset as usize,
                        &mut method,
                        &mut method_len,
                        &mut path,
                        &mut path_len,
                     )
                  };

                  if request_buffer_bytes_parsed > 0 {
                     request_buffer_offset += request_buffer_bytes_parsed as isize;

                     let response_buffer_filled = unsafe {
                        cb(
                           method as *const u8,
                           method_len,
                           path as *const u8,
                           path_len,
                           resbuf_start_address.add(response_buffer_filled_total) as *mut _,
                           HTTP_DATE.as_ptr(),
                        )
                     };

                     response_buffer_filled_total += response_buffer_filled as isize;
                  } else {
                     break;
                  }
               }

               if request_buffer_offset == 0 || response_buffer_filled_total == 0 {
                  *req_buf_cur_position = req_buf_start_address;
                  net::close_connection(epfd, cur_fd as isize);
                  continue;
               } else if request_buffer_offset == (read + buffer_used) {
                  *req_buf_cur_position = req_buf_start_address;
               } else {
                  *req_buf_cur_position += read;
               }

               let wrote = sys_call!(
                  SYS_WRITE as isize,
                  cur_fd as isize,
                  resbuf_start_address,
                  response_buffer_filled_total as isize
               );

               if likely(wrote == response_buffer_filled_total) {
               } else if unlikely(-wrote == EAGAIN as isize || -wrote == EINTR as isize) {
                  {
                     *req_buf_cur_position = req_buf_start_address;
                     net::close_connection(epfd, cur_fd as isize);
                     break;
                  }
               } else {
                  *req_buf_cur_position = req_buf_start_address;
                  net::close_connection(epfd, cur_fd as isize);
                  continue;
               }
            } else if unlikely(-read == EAGAIN as isize || -read == EINTR as isize) {
            } else {
               *req_buf_cur_position = req_buf_start_address;
               net::close_connection(epfd, cur_fd as isize);
            }
         }
      }
   }
}
