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

// This is faf's approach to parsing the HTTP request buffer, faster than all tested alternatives
// * Extract method
// * Extract path
// * Return a pointer to the buffer position at the end of the request.
//   Why? Because requests are pipelined and come in back to back

// It uses SSE4.2, may be able to be improved with AVX, but this old dev laptop I'm working on doesn't support AVX

// One oddity is how it matches two characters back to back (\r\n). Due to how _mm_cmpestri works, it doesn't actually
// match back-to-back characters, so it looks like this is erroneous, but I don't care if the characters are back-to-back,
// just they are present, because if they are present they 'may' be back-to-back. I test for the back-to-back-ness after the
// characters are found, with the magic number 0x0a0d0a0d which is \r\n\r\n

use core::arch::x86_64::{
   __m128i, _mm_cmpestri, _mm_load_si128, _mm_loadu_si128, _SIDD_CMP_EQUAL_ORDERED, _SIDD_UBYTE_OPS,
};
use core::intrinsics::{likely, unlikely};

const SPACE: i8 = b' ' as i8;

#[repr(C, align(64))]
pub struct aligned_pattern([u8; 2]);
const EOL_PATTERN: aligned_pattern = aligned_pattern(*b"\r\n");

#[inline]
pub unsafe fn find_sequence_simd(buf_start: *const i8, buf_end: *const i8) -> *const i8 {
   const OP: i32 = _SIDD_CMP_EQUAL_ORDERED | _SIDD_UBYTE_OPS;
   let mut buf: *const i8 = buf_start;

   let pattern16: __m128i = _mm_load_si128(EOL_PATTERN.0.as_ptr() as *const __m128i);

   loop {
      let b16: __m128i = _mm_loadu_si128(buf as *const __m128i);
      let r = _mm_cmpestri::<OP>(pattern16, 2, b16, 16);
      if r != 16 {
         //Increment buf by r, which is the position in the 16 byte search
         buf = buf.add(r as usize);
         return buf;
      }

      //Increment by 15 bytes instead of 16 bytes to ensure \r\n is never split/overlaps between searches
      buf = buf.add(15);
      if buf >= buf_end {
         break;
      }
   }

   buf_end
}

#[inline(always)]
pub unsafe fn parse_request_path_pipelined_simd(
   buf_start: *const i8,
   mut len: usize,
   method: *mut *const i8,
   method_len: *mut usize,
   path: *mut *const i8,
   path_len: *mut usize,
) -> isize {
   let mut buf: *const i8 = buf_start;
   let buf_end: *const i8 = buf_start.add(len);
   let mut i = 0;

   while likely(i < 9) {
      if *(buf.add(i)) == SPACE {
         *method = buf;
         *method_len = i;
         i += 1;
         while unlikely(*(buf.add(i)) == SPACE) {
            i += 1
         }
         break;
      }

      i += 1;
   }

   buf = buf.add(i);
   len -= i;
   i = 0;
   while likely(i < len) {
      if *(buf.add(i)) == SPACE {
         *path = buf;
         *path_len = i;
         i += 1;
         while unlikely(*(buf.add(i)) == SPACE) {
            i += 1
         }
         break;
      }

      i += 1;
   }

   if unlikely(*path_len == 0 || *method_len == 0) {
      return -1;
   };

   buf = buf.add(i);
   while likely(buf < buf_end) {
      buf = find_sequence_simd(buf, buf_end);
      if *(buf as *const u32) == 0x0a0d0a0d {
         buf = buf.add(4);
         return (buf as usize - buf_start as usize) as isize;
      } else {
         buf = buf.add(2);
      }
   }

   -2
}
