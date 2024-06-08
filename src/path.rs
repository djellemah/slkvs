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

#[allow(dead_code,unused_variables)]
fn min_bits(v : usize) -> usize {
  v.next_power_of_two()
}

#[test]
fn sizes() {
  assert_eq!(15usize.ilog2(), 3); // ie max bits - 1
  assert_eq!(16usize.ilog2(), 4); // ie max bits
  assert_eq!(55usize.ilog2(), 5); // ie max bits - 1
  assert_eq!(64usize.ilog2(), 6); // ie max bits

  assert_eq!(16,min_bits(15));
  assert_eq!(16,min_bits(16));
  assert_eq!(64,min_bits(55));
  assert_eq!(64,min_bits(64));

  use std::simd::*;
  let slash_sep = "on/tw/t";
  let len = slash_sep.len();
  let bytes = slash_sep.as_bytes();
  let mut simd_buf = u8x8::splat(0);
  let index_diff = 8 - len;
  // fill indices with
  for i in 0..len {
    simd_buf[i+index_diff] = bytes[i] as u8;
  }

  assert_eq!("[0, 111, 110, 47, 116, 119, 47, 116]", format!("{simd_buf:?}"));
  assert_eq!("47", format!("{:?}", '/' as c_char as u8));
  let zeroes = u8x8::splat(0u8);
  let slashes = u8x8::splat('/' as std::ffi::c_char as u8);

  let founds = simd_buf ^ slashes;
  assert_eq!("[47, 64, 65, 0, 91, 88, 0, 91]", format!("{founds:?}"));

  // for simd_eq and most of the other useful stuff
  use std::simd::prelude::*;
  let founds_mask = simd_buf.simd_eq(slashes);
  assert_eq!([false, false, false, true, false, false, true, false], founds_mask.to_array());

  let indices = (0..8).collect::<Vec<_>>();
  let indices_simd = Simd::<u8,8>::from_slice(&indices);
  let slash_indices = founds_mask.select(indices_simd, zeroes);
  assert_eq!("", format!("{slash_indices:?}"));

  let indices_usize = (0..8).collect::<Vec<_>>();
  let indices_usize = Simd::<usize,8>::from_slice(&indices_usize);
  let zeroed_indices = Simd::gather_select(indices.as_ref(), founds_mask.into(), indices_usize, zeroes);
  assert_eq!("", format!("{zeroed_indices:?}"));

  let zeroed_indices = Simd::gather_select(indices.as_ref(), founds_mask.into(), indices_usize, zeroes);
  assert_eq!("", format!("{zeroed_indices:?}"));
}

#[test]
fn minimal() {
  const W : usize = 64;
  const SLASH_SEP : &str = "on/tw/t";
  const TW : usize = SLASH_SEP.len().next_power_of_two();

  // build the simd
  use std::simd::*;
  let slash_sep = SLASH_SEP;
  let simd_buf : Simd<u8,W> = match slash_sep.len() {
    W => Simd::from_slice(slash_sep.as_bytes()),
    n if n < W => {
      // TODO this should be a const
      let idxs = (0..W).collect::<Vec<_>>();
      let idxs = Simd::from_slice(&idxs);
      Simd::gather_or_default(&slash_sep.as_bytes(),idxs)
    }
    _ => todo!(),
  };
  assert_eq!([111, 110, 47, 116, 119, 47, 116, 0], simd_buf.to_array()[0..TW]);

  // find the slashes
  use std::simd::prelude::*;
  let slashes : Simd<u8,W> = Simd::splat('/' as std::ffi::c_char as u8);
  let founds_mask = simd_buf.simd_eq(slashes);
  assert_eq!([false, false, true, false, false, true, false, false], founds_mask.to_array()[0..TW]);

  // convert founds_mask to a bitmask
  let founds_bitmask : u64 = founds_mask.to_bitmask() as u64;
  assert_eq!(founds_bitmask, 36);

  // Generate matrix bitmasks.
  // Basically this is a unitriangular matrix with 0s on the main diagonal
  let triangle_bitmasks = (0..(W as u8)).map(|n| (1<<n)-1 ).collect::<Vec<u64>>();
  let triangle_bitmasks: Simd<u64, W> = Simd::from_slice(&triangle_bitmasks);
  assert_eq!(vec![0u64, 1, 3, 7, 15, 31, 63, 127], triangle_bitmasks.to_array()[0..TW]);

  // calculate prefix sum using bitmasking of the founds_mask & each column in the triangle
  let founds_vector : Simd<u64,W> = Simd::splat(founds_bitmask);
  let prefix_mask = triangle_bitmasks & founds_vector;
  assert_eq!(prefix_mask.to_array()[0..TW], [0u64, 0, 0, 4, 4, 4, 36, 36]);

  // unfortunately popcnt will not work here, because prefix_mask is 4096 bits
  //
  // popcnt doesn't work, because we need it per-item
  // called popcnt in x86
  // https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_mask_popcnt_epi8.html
  //
  // _mm512_popcnt_epi8 seems to be the c intrinsic
  // https://en.wikipedia.org/wiki/AVX-512#VPOPCNTDQ_and_BITALG
  // core::arch::x86_64::_mm512_mask_popcnt_epi8
  // let mm512 : core::arch::x86_64::__m512i = prefix_mask.into();
  // assert_eq!("", format!("{mm512:?}"));
  // let prefix_sum = prefix_mask.
  // Simd::<u8,8>::from_slice(&indices);
  // let mut simd_buf = u8x8::splat(0);
  //
  // TODO can maybe split it into 8x 512 blocks?
  // TODO can maybe unroll this since it's a constant (8 or 64) number of loops
  let mut max_count = 0usize;
  let prefix_sum_counts = prefix_mask.as_array().iter()
    .map(|v| v.count_ones() as usize)
    .inspect(|c| max_count = std::cmp::max(*c,max_count))
    .collect::<Vec<usize>>();
  assert_eq!([0usize,0,0,1,1,1,2,2], prefix_sum_counts[0..TW]);
  assert_eq!(max_count, 2);

  // set up indices 0 1 2 3 4 5 6 ... 63
  let indices = (0..(W as u8)).collect::<Vec<u8>>();
  let indices : Simd<u8,W> = Simd::from_slice(&indices);
  assert_eq!(indices[0..8], [0u8,1,2,3,4,5,6,7]);
  assert_eq!(indices[56..], [56u8,57,58,59,60,61,62,63]);

  // copy prefix sum counts into the first max_count indices of an output array
  let prefix_sum_counts = Simd::from_slice(&prefix_sum_counts);
  let mut slash_indices = vec![0u8;64];
  indices.scatter(slash_indices.as_mut(), prefix_sum_counts);
  assert_eq!(slash_indices[0..max_count], vec![2, 5]);

  /////////////////////////////////////////////
  // Now finally split the string into parts

  // results go here
  let mut parts : Vec<&str> = Vec::with_capacity(max_count);
  let valid_indices = &slash_indices[0..max_count];
  let src = SLASH_SEP;

  // Iterate through the identified separators,
  // with adjustments for first and last.
  // first index of a sub-slice
  let mut fix = 0usize;
  // iterate through all splits and create slices
  for ix in valid_indices {
    let ix = *ix as usize; // types @_@
    parts.push(&src[fix..ix]);
    fix = ix+1; // +1 to skip over the separator
  }
  // last (TODO partial - must be prepended to next chunk) part
  // but just push it for now
  parts.push(&src[fix..]);

  assert_eq!(parts, vec!["on","tw","t"]);

  // valid_indices.iter().fold(0, ||)
  // let slash_indices = &slash_indices[0..max_count];
  // slash_indices.iter().map_windows(|[s,e])

}
