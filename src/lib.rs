

mod lzo1x_compress;
mod lzo1x_decompress_safe;
extern crate libc;
use std::mem;
fn lzo1x_worst_compress(x: usize) -> usize {
    ((x) + ((x) / 16) + 64 + 3)
}

const LZO1X_1_MEM_COMPRESS: usize = (8192 * 16);
const LZO1X_MEM_COMPRESS: usize = LZO1X_1_MEM_COMPRESS;

#[repr(i32)]
enum LZOError {
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

struct LZOContext {
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
    fn new() -> LZOContext {
        LZOContext { wrkmem: unsafe { libc::malloc(LZO1X_MEM_COMPRESS) } }
    }

    unsafe fn compress(&mut self,
                       in_: *const u8,
                       in_len: usize,
                       out: *mut u8,
                       out_len: *mut usize)
                       -> LZOError {
        mem::transmute::<i32, LZOError>(lzo1x_compress::lzo1x_1_compress(in_,
                                                                         in_len,
                                                                         out,
                                                                         out_len,
                                                                         &mut *self.wrkmem as
                                                                         *mut _ as
                                                                         *mut _))
    }
    unsafe fn decompress(in_: *const u8,
                         in_len: usize,
                         out: *mut u8,
                         out_len: *mut usize)
                         -> LZOError {
        mem::transmute::<i32, LZOError>(lzo1x_decompress_safe::lzo1x_decompress_safe(in_,
                                                                                     in_len,
                                                                                     out,
                                                                                     out_len))
    }
}

#[test]
fn it_works() {
    unsafe {
        let data = [0u8, 2, 3, 4, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    2, 3, 4, 2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3,
                    4, 2, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4, 2,
                    2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 4];
        let mut dst_len: usize = lzo1x_worst_compress(mem::size_of_val(&data));
        let dst = libc::malloc(dst_len);
        let mut ctx = LZOContext::new();
        let err = ctx.compress(&data as *const u8,
                               mem::size_of_val(&data),
                               dst as *mut u8,
                               &mut dst_len);
        println!("{}", dst_len);

        let dec_dst = libc::malloc(mem::size_of_val(&data));
        let mut result_len = mem::size_of_val(&data);
        let err = LZOContext::decompress(dst as *const u8,
                                         dst_len,
                                         dec_dst as *mut u8,
                                         &mut result_len);
        println!("{}", result_len);
    }

}
