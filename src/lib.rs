mod lzo1x_compress;
mod lzo1x_decompress_safe;
extern crate libc;
use std::mem;
use std::slice;

/// for a given size computes the worst case size that the compresed result can be
pub fn worst_compress(x: usize) -> usize {
    ((x) + ((x) / 16) + 64 + 3)
}

const LZO1X_1_MEM_COMPRESS: usize = (8192 * 16);
const LZO1X_MEM_COMPRESS: usize = LZO1X_1_MEM_COMPRESS;

#[repr(i32)]
pub enum LZOError {
    OK = 0,
    ERROR = -1,
    OUT_OF_MEMORY = -2,
    NOT_COMPRESSIBLE = -3,
    INPUT_OVERRUN = -4,
    OUTPUT_OVERRUN = -5,
    LOOKBEHIND_OVERRUN = -6,
    EOF_NOT_FOUND = -7,
    INPUT_NOT_CONSUMED = -8,
    NOT_YET_IMPLEMENTED = -9,
    INVALID_ARGUMENT = -10,
}

pub struct LZOContext {
    wrkmem: *mut libc::c_void,
}

impl Drop for LZOContext {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.wrkmem);
        }
    }
}

impl LZOContext {
    pub fn new() -> LZOContext {
        LZOContext { wrkmem: unsafe { libc::malloc(LZO1X_MEM_COMPRESS) } }
    }

    /// returns a slice containing the compressed data
    pub fn compress<'a>(&mut self, in_: &[u8], out: &'a mut [u8]) -> (&'a mut [u8], LZOError) {
        unsafe {
            let mut out_len = out.len();
            let err = lzo1x_compress::lzo1x_1_compress(in_.as_ptr(),
                                                       in_.len(),
                                                       out.as_mut_ptr(),
                                                       &mut out_len,
                                                       &mut *self.wrkmem as *mut _ as *mut _);
            (slice::from_raw_parts_mut(out.as_mut_ptr(), out_len),
             mem::transmute::<i32, LZOError>(err))
        }
    }

    /// returns a slice containing the decompressed data
    pub fn decompress<'a>(in_: &[u8], out: &'a mut [u8]) -> (&'a mut [u8], LZOError) {
        unsafe {
            let mut out_len = out.len();
            let err = lzo1x_decompress_safe::lzo1x_decompress_safe(in_.as_ptr(),
                                                                   in_.len(),
                                                                   out.as_mut_ptr(),
                                                                   &mut out_len);
            (slice::from_raw_parts_mut(out.as_mut_ptr(), out_len),
             mem::transmute::<i32, LZOError>(err))
        }
    }
}

#[test]
fn it_works() {
    unsafe {
        let data = [0u8, 2, 3, 4, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    2, 3, 4, 2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3,
                    4, 2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 2,
                    2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4];
        let mut dst_len: usize = worst_compress(mem::size_of_val(&data));
        let dst = libc::malloc(dst_len);
        let mut ctx = LZOContext::new();
        let mut dst = slice::from_raw_parts_mut(dst as *mut u8, dst_len);
        let (dst, err) = ctx.compress(&data, &mut dst);
        println!("{}", dst.len());

        let dec_dst = libc::malloc(mem::size_of_val(&data));
        let mut result_len = mem::size_of_val(&data);
        let mut dec_dst = slice::from_raw_parts_mut(dec_dst as *mut u8, result_len);
        let (result, err) = LZOContext::decompress(&dst, &mut dec_dst);
        println!("{}", result.len());
    }

}
