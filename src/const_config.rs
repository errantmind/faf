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

#[macro_export(local_inner_macros)]
macro_rules! const_concat_bytes {
    () => {
        ""
    };
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {{
        let bytes: &'static [u8] = unsafe {
            &$crate::util::concat::<
                [u8; $a.len()],
                [u8; $b.len()],
                [u8; $a.len() + $b.len()],
            >($a, $b)
        };

        unsafe { $crate::util::transmute(bytes) }
    }};
    ($a:expr, $($rest:expr),*) => {{
        pub const TAIL: &[u8] = const_concat_bytes!($($rest),*);
        const_concat_bytes!($a, TAIL)
    }};
    ($a:expr, $($rest:expr),*,) => {
        const_concat_bytes!($a, $($rest),*)
    };
}

/// faf spawns one thread per core, meaning each thread can handle 1024 connections
pub const MAX_CONN: usize = 1024;

/// the buffer size of the request buffer. Currently set to 4096 bytes (most common page size)
pub const REQ_BUFF_SIZE: usize = 4096;

/// the buffer size of both the response buffers. Currently set to 4096 bytes (most common page size)
pub const RES_BUFF_SIZE: usize = 4096;

/// our syscall to wait for epoll events will block indefinitely when this value is used
pub const EPOLL_TIMEOUT_BLOCKING: isize = -1;

/// our syscall to wait for epoll events will return immediately when this value is used
pub const EPOLL_TIMEOUT_IMMEDIATE_RETURN: isize = 0;

/// 4096 bytes page size / 12 byte epoll_event size = ~340. This size reduces page faults
pub const MAX_EPOLL_EVENTS_RETURNED: usize = 340;
