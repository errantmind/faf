// This particular file is LICENSED AS Unlicense (https://unlicense.org/)

const DIGITS_LUT: [u8; 200] = [
   b'0', b'0', b'0', b'1', b'0', b'2', b'0', b'3', b'0', b'4', b'0', b'5', b'0', b'6', b'0', b'7', b'0', b'8', b'0',
   b'9', b'1', b'0', b'1', b'1', b'1', b'2', b'1', b'3', b'1', b'4', b'1', b'5', b'1', b'6', b'1', b'7', b'1', b'8',
   b'1', b'9', b'2', b'0', b'2', b'1', b'2', b'2', b'2', b'3', b'2', b'4', b'2', b'5', b'2', b'6', b'2', b'7', b'2',
   b'8', b'2', b'9', b'3', b'0', b'3', b'1', b'3', b'2', b'3', b'3', b'3', b'4', b'3', b'5', b'3', b'6', b'3', b'7',
   b'3', b'8', b'3', b'9', b'4', b'0', b'4', b'1', b'4', b'2', b'4', b'3', b'4', b'4', b'4', b'5', b'4', b'6', b'4',
   b'7', b'4', b'8', b'4', b'9', b'5', b'0', b'5', b'1', b'5', b'2', b'5', b'3', b'5', b'4', b'5', b'5', b'5', b'6',
   b'5', b'7', b'5', b'8', b'5', b'9', b'6', b'0', b'6', b'1', b'6', b'2', b'6', b'3', b'6', b'4', b'6', b'5', b'6',
   b'6', b'6', b'7', b'6', b'8', b'6', b'9', b'7', b'0', b'7', b'1', b'7', b'2', b'7', b'3', b'7', b'4', b'7', b'5',
   b'7', b'6', b'7', b'7', b'7', b'8', b'7', b'9', b'8', b'0', b'8', b'1', b'8', b'2', b'8', b'3', b'8', b'4', b'8',
   b'5', b'8', b'6', b'8', b'7', b'8', b'8', b'8', b'9', b'9', b'0', b'9', b'1', b'9', b'2', b'9', b'3', b'9', b'4',
   b'9', b'5', b'9', b'6', b'9', b'7', b'9', b'8', b'9', b'9',
];

// Convert u64 to ascii string representation to bytes. This is useful for Content-Length
#[inline]
pub fn u64toa(buf: &mut [u8], value: u64) -> usize {
   let mut index: usize = 0;
   if value < 100000000 {
      let v: u32 = (value) as u32;
      if v < 10000 {
         let d1: u32 = (v / 100) << 1;
         let d2: u32 = (v % 100) << 1;

         if v >= 1000 {
            buf[index] = DIGITS_LUT[d1 as usize];
            index += 1;
         }

         if v >= 100 {
            buf[index] = DIGITS_LUT[d1 as usize + 1];
            index += 1;
         }

         if v >= 10 {
            buf[index] = DIGITS_LUT[d2 as usize];
            index += 1;
         }

         buf[index] = DIGITS_LUT[d2 as usize + 1];
         index += 1;
      } else {
         let b: u32 = v / 10000;
         let c: u32 = v % 10000;

         let d1: u32 = (b / 100) << 1;
         let d2: u32 = (b % 100) << 1;

         let d3: u32 = (c / 100) << 1;
         let d4: u32 = (c % 100) << 1;

         if value >= 10000000 {
            buf[index] = DIGITS_LUT[d1 as usize];
            index += 1;
         }

         if value >= 1000000 {
            buf[index] = DIGITS_LUT[d1 as usize + 1];
            index += 1;
         }

         if value >= 100000 {
            buf[index] = DIGITS_LUT[d2 as usize];
            index += 1;
         }

         buf[index] = DIGITS_LUT[d2 as usize + 1];
         index += 1;

         buf[index] = DIGITS_LUT[d3 as usize];
         index += 1;

         buf[index] = DIGITS_LUT[d3 as usize + 1];
         index += 1;

         buf[index] = DIGITS_LUT[d4 as usize];
         index += 1;

         buf[index] = DIGITS_LUT[d4 as usize + 1];
         index += 1;
      }
   }

   index
}

// Convert u8 to ascii string representation to bytes.
#[inline]
pub fn u8toa(out_buf_start: *const u8, value: u8) -> usize {
   let mut buf_walker = out_buf_start.as_mut();

   let v: u32 = value as u32;

   let d1: u32 = (v / 100) << 1;
   let d2: u32 = (v % 100) << 1;

   unsafe {
      if v >= 100 {
         *buf_walker = *DIGITS_LUT.get_unchecked(d1 as usize + 1);
         buf_walker = buf_walker.add(1);
      }

      if v >= 10 {
         *buf_walker = *DIGITS_LUT.get_unchecked(d2 as usize);
         buf_walker = buf_walker.add(1);
      }

      *buf_walker = *DIGITS_LUT.get_unchecked(d2 as usize + 1);
      buf_walker = buf_walker.add(1);
   }

   buf_walker as usize - out_buf_start as usize
}
