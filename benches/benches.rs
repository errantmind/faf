use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rand::{self, Rng};

mod perf;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(10));
    targets = criterion_benchmark
}

criterion_main!(benches);

pub fn criterion_benchmark(c: &mut Criterion) {
   // c.bench_function("u64toa_unchecked", |b| {
   //    b.iter(|| {
   //       unsafe {
   //          let mut rand_rng = rand::thread_rng();
   //          let rand_num: u64 = rand_rng.gen_range(1..99000000);
   //          let mut test_buff: [i8; 8] = std::mem::zeroed();
   //          black_box(faf::util::u64toa_unchecked(&mut test_buff, rand_num));
   //       }
   //    })
   // });

   // c.bench_function("u64toa", |b| {
   //    b.iter(|| {
   //       let mut rand_rng = rand::thread_rng();
   //       let mut test_buff: [i8; 8] = unsafe { std::mem::zeroed() };
   //       let rand_num: u64 = rand_rng.gen_range(1..99000000);
   //       black_box(faf::util::u64toa(&mut test_buff, rand_num));
   //    })
   // });

   // c.bench_function("get_time", |b| {
   //    let mut tm: faf::el_epoll_ds::tm = unsafe { std::mem::zeroed() };
   //    let mut buff: [u8; 30] = unsafe { std::mem::zeroed() };
   //    b.iter(|| {
   //       faf::util::get_date(black_box(&mut buff), &mut tm);
   //       black_box(unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(buff.as_ptr(), 29)) });
   //    })
   // });

   // c.bench_function("httpdate", |b| {
   //    b.iter(|| {
   //       let systime = std::time::SystemTime::now();
   //       black_box(faf::extern_httpdate::fmt_http_date(systime));
   //    })
   // });

   c.bench_function("httpdate_faster", |b| {
      b.iter(|| {
         let mut buf = faf::extern_httpdate::get_buff();
         black_box(faf::extern_httpdate::get_http_date(&mut buf));
      })
   });

   c.bench_function("httpdate", |b| {
      b.iter(|| {
         black_box(httpdate::fmt_http_date(std::time::SystemTime::now()));
      })
   });
}
