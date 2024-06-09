/*
  convert a path like "root/things/3/name/first"
  into &[Key("root"), Key("things"), Index(3), Key("name"), Key("first")]
  Using simd, kinda for fun

  This uses Stream Compaction, aka Left Packing.

*/ /*
  See also http://arxiv.org/pdf/1611.07612

  See https://www.cse.chalmers.se/~uffe/streamcompaction.pdf
  which uses a POPC (count of set bits) to provide target index for each found element.

  procedure compactSIMD (a[0..S) )
   m ← 0
   in parallel for each SIMD lane s
    if valid a[s]
     m ← m | (1 << s)

   in parallel for each SIMD lane s
    if valid a[s]
     m' ← m & ((1 << s) - 1)
     s' ← POPC m'
     result[s'] ← a[s]

   numValid ← POPC m

   return (result[0..S) , numValid)

*/ /*

  Operation         (vector) Values
  s                 0  1  2  3  4  5  6
  s^2               1  2  4  8 16 32 64
  chr               o  n  /  t  w  /  t
  idx               0  0  2  0  0  5  0
  bool              f  f  t  f  f  t  f
  m (18)            0  0  1  0  0  1  0 # convert this to a register/bitmask
  m': ((1<<s)-1)    0  1  3  7 15 31 63

  desired           0  0  1  1  1  2  2
  m&m'              0  0  0  4  4  4 36
  popc              0  0  0  1  1  1  2 # ie bit count of m&m', and dst index for scatter assuming multiple overwrite
                          |        |  |
                    ---=============  > # but this one is a problem? Oh. Note a[0..s) rhs open range
                    v  v
  scatter           2  5 (6)            # so the contiguous list of indexes from row [s]

  Also, at scatter step,
*/
/*
Alternatively as matrix multiplication. Fortunately x is close to &

                    Each col is (2^n)-1
[0 0 1 0 0 1 0 0] x [ 0 1 1 1 1 1 1 1      [0 0 0 1 1 1 2 2]
                      0 0 1 1 1 1 1 1
                      0 0 0 1 1 1 1 1
                      0 0 0 0 1 1 1 1   =>
                      0 0 0 0 0 1 1 1
                      0 0 0 0 0 0 1 1
                      0 0 0 0 0 0 0 1
                      0 0 0 0 0 0 0 0 ]
*/

// fn make_simd<const N : usize>(slash_sep : &str) -> std::simd::Simd<u8,N> {
//   std::simd::Simd::splat(0)
// }

use std::ffi::c_char;

#[allow(dead_code,unused_variables)]
// use std::slice::SliceIndex;
// fn fast_split_path<S>(slash_sep : S) -> Vec<crate::tree::Step>
// where S : AsRef<str> + ExactSizeIterator + SliceIndex<u8> + std::ops::Index
fn fast_split_path(slash_sep : &str) -> Vec<crate::tree::Step>
{
  use std::simd::*;
  let len = slash_sep.len();
  let simd_buf  = match len.next_power_of_two() {
    8  => {
      let bytes = slash_sep.as_bytes();
      let simd_buf = u8x8::splat(0);
      // fill indices with chars
      // will need scatter later?
      // u8x8::gather_or_default(bytes, indices);
      let slashes = u8x8::splat('/' as std::ffi::c_char as u8);
    }
    16 => todo!(),
    32 => todo!(),
    64 => todo!(),
    n if n < 8  => return crate::tree::split_slash_path(slash_sep),
    n if n > 64 => return crate::tree::split_slash_path(slash_sep),
    unknown => panic!("unknown size in fast_split_path: {unknown}"),
  };
  vec![]
}
