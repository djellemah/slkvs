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

use std::simd::*;
const W : usize = 64;

// Generate matrix bitmasks, once.
// Basically this is a unitriangular matrix with 0s on the main diagonal
// const TRIANGLE_BITMASKS_VEC : Vec<u64> = (0..(W as u8)).map(|n| (1<<n)-1 ).collect::<Vec<u64>>();
const TRIANGLE_BITMASKS_ARY : [u64;64] = [
// 0b0000000000000000000000000000000000000000000000000000000000000000,
0b0000000000000000000000000000000000000000000000000000000000000001,
0b0000000000000000000000000000000000000000000000000000000000000011,
0b0000000000000000000000000000000000000000000000000000000000000111,
0b0000000000000000000000000000000000000000000000000000000000001111,
0b0000000000000000000000000000000000000000000000000000000000011111,
0b0000000000000000000000000000000000000000000000000000000000111111,
0b0000000000000000000000000000000000000000000000000000000001111111,
0b0000000000000000000000000000000000000000000000000000000011111111,
0b0000000000000000000000000000000000000000000000000000000111111111,
0b0000000000000000000000000000000000000000000000000000001111111111,
0b0000000000000000000000000000000000000000000000000000011111111111,
0b0000000000000000000000000000000000000000000000000000111111111111,
0b0000000000000000000000000000000000000000000000000001111111111111,
0b0000000000000000000000000000000000000000000000000011111111111111,
0b0000000000000000000000000000000000000000000000000111111111111111,
0b0000000000000000000000000000000000000000000000001111111111111111,
0b0000000000000000000000000000000000000000000000011111111111111111,
0b0000000000000000000000000000000000000000000000111111111111111111,
0b0000000000000000000000000000000000000000000001111111111111111111,
0b0000000000000000000000000000000000000000000011111111111111111111,
0b0000000000000000000000000000000000000000000111111111111111111111,
0b0000000000000000000000000000000000000000001111111111111111111111,
0b0000000000000000000000000000000000000000011111111111111111111111,
0b0000000000000000000000000000000000000000111111111111111111111111,
0b0000000000000000000000000000000000000001111111111111111111111111,
0b0000000000000000000000000000000000000011111111111111111111111111,
0b0000000000000000000000000000000000000111111111111111111111111111,
0b0000000000000000000000000000000000001111111111111111111111111111,
0b0000000000000000000000000000000000011111111111111111111111111111,
0b0000000000000000000000000000000000111111111111111111111111111111,
0b0000000000000000000000000000000001111111111111111111111111111111,
0b0000000000000000000000000000000011111111111111111111111111111111,
0b0000000000000000000000000000000111111111111111111111111111111111,
0b0000000000000000000000000000001111111111111111111111111111111111,
0b0000000000000000000000000000011111111111111111111111111111111111,
0b0000000000000000000000000000111111111111111111111111111111111111,
0b0000000000000000000000000001111111111111111111111111111111111111,
0b0000000000000000000000000011111111111111111111111111111111111111,
0b0000000000000000000000000111111111111111111111111111111111111111,
0b0000000000000000000000001111111111111111111111111111111111111111,
0b0000000000000000000000011111111111111111111111111111111111111111,
0b0000000000000000000000111111111111111111111111111111111111111111,
0b0000000000000000000001111111111111111111111111111111111111111111,
0b0000000000000000000011111111111111111111111111111111111111111111,
0b0000000000000000000111111111111111111111111111111111111111111111,
0b0000000000000000001111111111111111111111111111111111111111111111,
0b0000000000000000011111111111111111111111111111111111111111111111,
0b0000000000000000111111111111111111111111111111111111111111111111,
0b0000000000000001111111111111111111111111111111111111111111111111,
0b0000000000000011111111111111111111111111111111111111111111111111,
0b0000000000000111111111111111111111111111111111111111111111111111,
0b0000000000001111111111111111111111111111111111111111111111111111,
0b0000000000011111111111111111111111111111111111111111111111111111,
0b0000000000111111111111111111111111111111111111111111111111111111,
0b0000000001111111111111111111111111111111111111111111111111111111,
0b0000000011111111111111111111111111111111111111111111111111111111,
0b0000000111111111111111111111111111111111111111111111111111111111,
0b0000001111111111111111111111111111111111111111111111111111111111,
0b0000011111111111111111111111111111111111111111111111111111111111,
0b0000111111111111111111111111111111111111111111111111111111111111,
0b0001111111111111111111111111111111111111111111111111111111111111,
0b0011111111111111111111111111111111111111111111111111111111111111,
0b0111111111111111111111111111111111111111111111111111111111111111,
0b1111111111111111111111111111111111111111111111111111111111111111
];
const TRIANGLE_BITMASKS: Simd<u64, W> = Simd::from_slice(&TRIANGLE_BITMASKS_ARY);

#[allow(dead_code)]
const ZEROES : Simd<usize,W> = Simd::from_slice(&[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);

const IDXS_U8_SIMD : Simd<u8,W> = Simd::from_slice(&[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63]);
const IDXS_USIZE_SIMD : Simd<usize,W> = Simd::from_slice(&[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63]);

// Precondition: slash_sep.len() <= 64 .
// Otherwise probably weird things will happen. Actually no they don't. The last part is just unsplit'd.
#[allow(dead_code)]
fn fast_split_chunk(slash_sep : &str) -> Vec<&str>
{
  let simd_buf : Simd<u8,W> = Simd::gather_or_default(&slash_sep.as_bytes(), IDXS_USIZE_SIMD);

  // find the slashes
  use std::simd::prelude::*;
  let slashes : Simd<u8,W> = Simd::splat('/' as std::ffi::c_char as u8);
  let founds_mask = simd_buf.simd_eq(slashes);

  // convert founds_mask to a bitmask
  let founds_bitmask : u64 = founds_mask.to_bitmask();

  // calculate prefix sum using bitmasking of the founds_mask & each column in the triangle
  let founds_vector : Simd<u64,W> = Simd::splat(founds_bitmask);
  let prefix_bits = TRIANGLE_BITMASKS & founds_vector;
  // This mask is somewhat useful for debugging
  // let prefix_bits = founds_mask.cast::<i64>().select(prefix_bits, ZEROES.cast::<u64>());

  // unfortunately popcnt will not work here, because prefix_bits is 4096 bits
  //
  // popcnt doesn't work, because we need it per-item
  // called popcnt in x86
  // https://doc.rust-lang.org/core/arch/x86_64/fn._mm512_mask_popcnt_epi8.html
   //
  // _mm512_popcnt_epi8 seems to be the c intrinsic
  // https://en.wikipedia.org/wiki/AVX-512#VPOPCNTDQ_and_BITALG
  // core::arch::x86_64::_mm512_mask_popcnt_epi8
  // let mm512 : core::arch::x86_64::__m512i = prefix_bits.into();
  // assert_eq!("", format!("{mm512:?}"));
  // let prefix_sum = prefix_bits.
  // Simd::<u8,8>::from_slice(&indices);
  //
  // My laptop supports AVX2 https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#Advanced_Vector_Extensions_2
  // So the best I can do on avx2 is use u128 popcnt, but that will give counts per 16 bytes.
  // There are however other fast popcnt implementations.
  // http://0x80.pl/articles/sse-popcount.html
  // https://arxiv.org/pdf/1611.07612
  //
  // TODO on avx512 can maybe split it into 8x 512 blocks?
  // TODO can maybe unroll this since it's a constant (8 or 64) number of loops
  // TODO and then how to handle max_count?
  let mut max_count = 0usize;
  let prefix_sums = prefix_bits.as_array().iter()
    .map(|v| v.count_ones() as usize)
    .inspect(|c| max_count = std::cmp::max(*c,max_count))
    .collect::<Vec<usize>>();

  // copy prefix sum counts into the first max_count indices of an output array
  let prefix_sums = Simd::from_slice(&prefix_sums);
  let mut slash_indices = vec![0u8;64];
  // TODO is there a way to avoid this cast?
  IDXS_U8_SIMD.scatter_select(slash_indices.as_mut(), founds_mask.cast::<isize>(), prefix_sums);

  /////////////////////////////////////////////
  // Now finally split the string into parts

  // results go here. max_count very useful for this.
  let mut parts : Vec<&str> = Vec::with_capacity(max_count);
  // The first index is always zero, which we handle below, so ignore it
  let valid_indices = &slash_indices[1..(max_count+1)];

  // Iterate through the identified separators,
  // with adjustments for first and last.
  // first index of a sub-slice
  let mut fix = 0usize;
  // iterate through all splits and create slices
  for &ix in valid_indices {
    let ix = ix as usize; // types @_@
    parts.push(&slash_sep[fix..ix]);
    fix = ix+1; // +1 to skip over the separator
  }
  // last (TODO partial - must be prepended to next chunk) part
  // but just push it for now
  parts.push(&slash_sep[fix..]);
  parts
}

#[allow(dead_code)]
pub fn fast_split_path(slash_sep : &str) -> Vec<&str> {
  match slash_sep.len() {
    W => fast_split_chunk(slash_sep),
    n if n < W => fast_split_chunk(slash_sep),
    _ => todo!(),
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn basic() {
    let parts = fast_split_path("hello/this/is/longish/string/but/less/than/sixty/four");
    assert_eq!(parts,vec!["hello", "this", "is", "longish", "string", "but", "less", "than", "sixty", "four"]);
  }

  #[test]
  fn leading_slash() {
    let parts = fast_split_path("/start/with");
    assert_eq!(parts,vec!["","start","with"]);
  }

  #[test]
  fn no_slashes() {
    let parts = fast_split_path("start_with_a_string_but_have_no_slashes");
    assert_eq!(parts,vec!["start_with_a_string_but_have_no_slashes"]);
  }

  #[test]
  fn trailing_slash() {
    let parts = fast_split_path("the/end/");
    assert_eq!(parts,vec!["the", "end", ""]);
  }

  #[test]
  fn when_im_sixty_four_trailing() {
    let parts = fast_split_path("This/is/the/very/long/path/that/is/trailin/sixtyfour/characters/");
    assert_eq!(parts,vec!["This","is","the","very","long","path","that","is","trailin","sixtyfour","characters",""]);
  }

  #[test]
  fn when_im_sixty_four_leading() {
    let parts = fast_split_path("/This/is/the/very/long/path/that/is/exactly/sixtyfour/characters");
    // TODO failing because of trailing / at boundarys
    assert_eq!(parts,vec!["","This","is","the","very","long","path","that","is","exactly","sixtyfour","characters"]);
  }

  #[test]
  fn when_im_sixty_four_exact() {
    let parts = fast_split_path("This/is/a/lengthy/path/that/will/be/exactly/sixtyfour/characters");
    assert_eq!(parts,vec!["This","is","a","lengthy","path","that","will","be","exactly","sixtyfour","characters"]);
  }

  #[test]
  fn longer() {
    let parts = fast_split_path("/This/is/a/very/long/path/that/is/longer/than/sixty/four/characters/");
    assert_eq!(parts,vec!["","This","is","a","very","long","path","that","is","longer","than","sixty","four","characters", ""]);
  }

  #[test]
  fn chunk_longer() {
    let parts = fast_split_chunk("/This/is/a/very/long/path/that/is/longer/than/sixty/four/characters/");
    assert_eq!(parts,vec!["","This","is","a","very","long","path","that","is","longer","than","sixty","four","characters", ""]);
  }
}
