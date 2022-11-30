// This particular file is LICENSED AS MIT / Apache 2.0

// This is a stripped and heavily optimized version of https://github.com/pyfisch/httpdate 0.3.2

// faf's inspired version of this ubiquitous algorithm is the fastest in all my tests and only takes ~21ns per execution

// To use this yourself, see https://crates.io/crates/faf-http-date

#[inline(always)]
pub const fn get_buff_with_date() -> [u8; 35] {
   let buf: [u8; 35] = [
      // Writing as the following only gives us a reference: b"Date: Thu, 01 Jan 1970 00:00:00 GMT"
      b'D', b'a', b't', b'e', b':', b' ', b' ', b' ', b' ', b',', b' ', b'0', b'0', b' ', b' ', b' ', b' ', b' ', b'0',
      b'0', b'0', b'0', b' ', b'0', b'0', b':', b'0', b'0', b':', b'0', b'0', b' ', b'G', b'M', b'T',
   ];

   buf
}

#[repr(C, align(32))]
pub struct timespec {
   pub tv_sec: i64,
   pub tv_nsec: i64,
}

extern "C" {
   // We use this function instead of a direct syscall because this function uses VDSO, which is faster
   fn clock_gettime(clk_id: i32, tp: *mut timespec) -> i32;
}

const CLOCK_REALTIME: i32 = 0;

#[inline(always)]
pub fn get_http_date(buf: &mut [u8; 35]) {
   let mut ts: timespec = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
   unsafe { clock_gettime(CLOCK_REALTIME, &mut ts as *mut timespec) };

   let secs_since_epoch = ts.tv_sec;

   const LEAPOCH: i64 = 11017;
   const DAYS_PER_400Y: i64 = 365 * 400 + 97;
   const DAYS_PER_100Y: i64 = 365 * 100 + 24;
   const DAYS_PER_4Y: i64 = 365 * 4 + 1;

   let days = (secs_since_epoch / 86400) - LEAPOCH;
   let secs_of_day = secs_since_epoch % 86400;

   let mut qc_cycles = days / DAYS_PER_400Y;
   let mut remdays = days % DAYS_PER_400Y;

   if remdays < 0 {
      remdays += DAYS_PER_400Y;
      qc_cycles -= 1;
   }

   let mut c_cycles = remdays / DAYS_PER_100Y;
   if c_cycles == 4 {
      c_cycles -= 1;
   }
   remdays -= c_cycles * DAYS_PER_100Y;

   let mut q_cycles = remdays / DAYS_PER_4Y;
   if q_cycles == 25 {
      q_cycles -= 1;
   }
   remdays -= q_cycles * DAYS_PER_4Y;

   let mut remyears = remdays / 365;
   if remyears == 4 {
      remyears -= 1;
   }
   remdays -= remyears * 365;

   let mut year = 2000 + remyears + 4 * q_cycles + 100 * c_cycles + 400 * qc_cycles;

   const MONTHS: [i64; 12] = [31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29];
   let mut mon = 0;
   for mon_len in MONTHS.iter() {
      mon += 1;
      if remdays < *mon_len {
         break;
      }
      remdays -= *mon_len;
   }
   let mday = remdays + 1;
   let mon = if mon + 2 > 12 {
      year += 1;
      mon - 10
   } else {
      mon + 2
   };

   let mut wday = (3 + days) % 7;
   if wday <= 0 {
      wday += 7
   };

   let sec = (secs_of_day % 60) as u8;
   let min = ((secs_of_day % 3600) / 60) as u8;
   let hour = (secs_of_day / 3600) as u8;
   let day = mday as u8;
   let mon = mon as u8;
   let year = year as u16;
   let wday = wday as u8;

   const WDAY_LOOKUP: [&[u8; 3]; 8] = [b"NOP", b"Mon", b"Tue", b"Wed", b"Thu", b"Fri", b"Sat", b"Sun"];

   const MON_LOOKUP: [&[u8; 3]; 13] =
      [b"NOP", b"Jan", b"Feb", b"Mar", b"Apr", b"May", b"Jun", b"Jul", b"Aug", b"Sep", b"Oct", b"Nov", b"Dec"];

   let wday = unsafe { *WDAY_LOOKUP.get_unchecked(wday as usize) };
   let mon = unsafe { *MON_LOOKUP.get_unchecked(mon as usize) };

   unsafe {
      core::ptr::copy_nonoverlapping(wday.as_ptr(), buf.as_mut_ptr().add(6), 3);
   }

   buf[11] = b'0' + (day / 10) as u8;
   buf[12] = b'0' + (day % 10) as u8;
   unsafe {
      core::ptr::copy_nonoverlapping(mon.as_ptr(), buf.as_mut_ptr().add(14), 3);
   }

   buf[18] = b'0' + (year / 1000) as u8;
   buf[19] = b'0' + (year / 100 % 10) as u8;
   buf[20] = b'0' + (year / 10 % 10) as u8;
   buf[21] = b'0' + (year % 10) as u8;
   buf[23] = b'0' + (hour / 10) as u8;
   buf[24] = b'0' + (hour % 10) as u8;
   buf[26] = b'0' + (min / 10) as u8;
   buf[27] = b'0' + (min % 10) as u8;
   buf[29] = b'0' + (sec / 10) as u8;
   buf[30] = b'0' + (sec % 10) as u8;
}
