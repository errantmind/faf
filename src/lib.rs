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

#![allow(clippy::missing_safety_doc, clippy::uninit_assumed_init, dead_code)]
#![feature(const_size_of_val, core_intrinsics, ptr_const_cast)]

pub mod const_config;
pub mod const_http;
mod const_sys;
pub mod epoll;
pub mod http_content_length;
mod http_date;
mod http_request_path;
mod net;
mod syscall;
pub mod util;

/// fn(
///
///   method: *const u8,
///
///   method_len: usize,
///
///   path: *const u8,
///
///   path_len: usize,
///
///   response_buffer: *mut u8,
/// 
///   response_buffer_remaining: usize
///
///   date_buff: *const u8,
///
///   ip_addr: u32
///
/// ) -> bytes_written_to_response_buffer: usize
pub type CallbackFunction = fn(*const u8, usize, *const u8, usize, *mut u8, usize, *const u8, u32) -> usize;
