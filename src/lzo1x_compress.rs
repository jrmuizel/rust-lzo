extern {
    fn memcpy(
        __dest : *mut ::std::os::raw::c_void,
        __src : *const ::std::os::raw::c_void,
        __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn memset(
        __s : *mut ::std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut ::std::os::raw::c_void;
}

unsafe extern fn get_unaligned_le32(
    mut p : *const ::std::os::raw::c_void
) -> u32 {
    let mut ret : u32 = 0i32 as (u32);
    memcpy(
        &mut ret as (*mut u32) as (*mut ::std::os::raw::c_void),
        p,
        ::std::mem::size_of::<u32>()
    );
    ret
}

unsafe extern fn put_unaligned(
    mut v : u32, mut p : *mut ::std::os::raw::c_void
) {
    memcpy(
        p,
        &mut v as (*mut u32) as (*const ::std::os::raw::c_void),
        ::std::mem::size_of::<u32>()
    );
}

unsafe extern fn get_unaligned(
    mut p : *const ::std::os::raw::c_void
) -> u32 {
    let mut ret : u32 = 0i32 as (u32);
    memcpy(
        &mut ret as (*mut u32) as (*mut ::std::os::raw::c_void),
        p,
        ::std::mem::size_of::<u32>()
    );
    ret
}

unsafe extern fn lzo1x_1_do_compress(
    mut in_ : *const u8,
    mut in_len : usize,
    mut out : *mut u8,
    mut out_len : *mut usize,
    mut ti : usize,
    mut wrkmem : *mut ::std::os::raw::c_void
) -> usize {
    let mut ip : *const u8;
    let mut op : *mut u8;
    let in_end : *const u8 = in_.offset(in_len as (isize));
    let ip_end
        : *const u8
        = in_.offset(in_len as (isize)).offset(-(20i32 as (isize)));
    let mut ii : *const u8;
    let dict : *mut u16 = wrkmem as (*mut u16);
    op = out;
    ip = in_;
    ii = ip;
    ip = ip.offset(
             if ti < 4i32 as (usize) {
                 (4i32 as (usize)).wrapping_sub(ti)
             } else {
                 0i32 as (usize)
             } as (isize)
         );
    let mut m_pos : *const u8;
    let mut t : usize;
    let mut m_len : usize;
    let mut m_off : usize;
    let mut dv : u32;
    'loop2: loop {
        ip = ip.offset(
                 1i32 as (isize) + ((ip as (isize)).wrapping_sub(
                                        ii as (isize)
                                    ) / ::std::mem::size_of::<u8>() as (isize) >> 5i32)
             );
        'loop3: loop {
            if ip >= ip_end {
                break 'loop2;
            } else {
                dv = get_unaligned_le32(ip as (*const ::std::os::raw::c_void));
                t = (dv.wrapping_mul(
                         0x1824429di32 as (u32)
                     ) >> 32i32 - 13i32 & (1u32 << 13i32).wrapping_sub(
                                              1i32 as (u32)
                                          )) as (usize);
                m_pos = in_.offset(*dict.offset(t as (isize)) as (isize));
                *dict.offset(t as (isize)) = ((ip as (isize)).wrapping_sub(
                                                  in_ as (isize)
                                              ) / ::std::mem::size_of::<u8>() as (isize)) as (u16);
                if dv != get_unaligned_le32(
                             m_pos as (*const ::std::os::raw::c_void)
                         ) {
                    continue 'loop2;
                } else {
                    ii = ii.offset(-(ti as (isize)));
                    ti = 0i32 as (usize);
                    t = ((ip as (isize)).wrapping_sub(
                             ii as (isize)
                         ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                    if t != 0i32 as (usize) {
                        if t <= 3i32 as (usize) {
                            {
                                let _rhs = t;
                                let _lhs = &mut *op.offset(-2i32 as (isize));
                                *_lhs = (*_lhs as (usize) | _rhs) as (u8);
                            }
                            put_unaligned(
                                get_unaligned(
                                    ii as (*const u32) as (*const ::std::os::raw::c_void)
                                ),
                                op as (*mut u32) as (*mut ::std::os::raw::c_void)
                            );
                            op = op.offset(t as (isize));
                        } else if t <= 16i32 as (usize) {
                            *{
                                 let _old = op;
                                 op = op.offset(1 as (isize));
                                 _old
                             } = t.wrapping_sub(3i32 as (usize)) as (u8);
                            put_unaligned(
                                get_unaligned(
                                    ii as (*const u32) as (*const ::std::os::raw::c_void)
                                ),
                                op as (*mut u32) as (*mut ::std::os::raw::c_void)
                            );
                            put_unaligned(
                                get_unaligned(
                                    ii.offset(
                                        4i32 as (isize)
                                    ) as (*const u32) as (*const ::std::os::raw::c_void)
                                ),
                                op.offset(
                                    4i32 as (isize)
                                ) as (*mut u32) as (*mut ::std::os::raw::c_void)
                            );
                            put_unaligned(
                                get_unaligned(
                                    ii.offset(
                                        8i32 as (isize)
                                    ) as (*const u32) as (*const ::std::os::raw::c_void)
                                ),
                                op.offset(
                                    8i32 as (isize)
                                ) as (*mut u32) as (*mut ::std::os::raw::c_void)
                            );
                            put_unaligned(
                                get_unaligned(
                                    ii.offset(8i32 as (isize)).offset(
                                        4i32 as (isize)
                                    ) as (*const u32) as (*const ::std::os::raw::c_void)
                                ),
                                op.offset(8i32 as (isize)).offset(
                                    4i32 as (isize)
                                ) as (*mut u32) as (*mut ::std::os::raw::c_void)
                            );
                            op = op.offset(t as (isize));
                        } else {
                            if t <= 18i32 as (usize) {
                                *{
                                     let _old = op;
                                     op = op.offset(1 as (isize));
                                     _old
                                 } = t.wrapping_sub(3i32 as (usize)) as (u8);
                            } else {
                                let mut tt : usize = t.wrapping_sub(18i32 as (usize));
                                *{
                                     let _old = op;
                                     op = op.offset(1 as (isize));
                                     _old
                                 } = 0i32 as (u8);
                                'loop10: loop {
                                    if tt > 255i32 as (usize) {
                                        tt = tt.wrapping_sub(255i32 as (usize));
                                        *{
                                             let _old = op;
                                             op = op.offset(1 as (isize));
                                             _old
                                         } = 0i32 as (u8);
                                        continue 'loop10;
                                    } else {
                                        break 'loop10;
                                    }
                                }
                                *{
                                     let _old = op;
                                     op = op.offset(1 as (isize));
                                     _old
                                 } = tt as (u8);
                            }
                            'loop14: loop {
                                put_unaligned(
                                    get_unaligned(
                                        ii as (*const u32) as (*const ::std::os::raw::c_void)
                                    ),
                                    op as (*mut u32) as (*mut ::std::os::raw::c_void)
                                );
                                put_unaligned(
                                    get_unaligned(
                                        ii.offset(
                                            4i32 as (isize)
                                        ) as (*const u32) as (*const ::std::os::raw::c_void)
                                    ),
                                    op.offset(
                                        4i32 as (isize)
                                    ) as (*mut u32) as (*mut ::std::os::raw::c_void)
                                );
                                put_unaligned(
                                    get_unaligned(
                                        ii.offset(
                                            8i32 as (isize)
                                        ) as (*const u32) as (*const ::std::os::raw::c_void)
                                    ),
                                    op.offset(
                                        8i32 as (isize)
                                    ) as (*mut u32) as (*mut ::std::os::raw::c_void)
                                );
                                put_unaligned(
                                    get_unaligned(
                                        ii.offset(8i32 as (isize)).offset(
                                            4i32 as (isize)
                                        ) as (*const u32) as (*const ::std::os::raw::c_void)
                                    ),
                                    op.offset(8i32 as (isize)).offset(
                                        4i32 as (isize)
                                    ) as (*mut u32) as (*mut ::std::os::raw::c_void)
                                );
                                op = op.offset(16i32 as (isize));
                                ii = ii.offset(16i32 as (isize));
                                t = t.wrapping_sub(16i32 as (usize));
                                if t >= 16i32 as (usize) {
                                    continue 'loop14;
                                } else {
                                    break 'loop14;
                                }
                            }
                            if t > 0i32 as (usize) {
                                'loop17: loop {
                                    *{
                                         let _old = op;
                                         op = op.offset(1 as (isize));
                                         _old
                                     } = *{
                                              let _old = ii;
                                              ii = ii.offset(1 as (isize));
                                              _old
                                          };
                                    if {
                                           t = t.wrapping_sub(1 as (usize));
                                           t
                                       } > 0i32 as (usize) {
                                        continue 'loop17;
                                    } else {
                                        break 'loop17;
                                    }
                                }
                            }
                        }
                    }
                    m_len = 4i32 as (usize);
                    if *ip.offset(m_len as (isize)) as (i32) == *m_pos.offset(
                                                                     m_len as (isize)
                                                                 ) as (i32) {
                        'loop22: loop {
                            m_len = m_len.wrapping_add(1i32 as (usize));
                            if *ip.offset(m_len as (isize)) as (i32) != *m_pos.offset(
                                                                             m_len as (isize)
                                                                         ) as (i32) {
                                break 'loop22;
                            } else {
                                m_len = m_len.wrapping_add(1i32 as (usize));
                                if *ip.offset(m_len as (isize)) as (i32) != *m_pos.offset(
                                                                                 m_len as (isize)
                                                                             ) as (i32) {
                                    break 'loop22;
                                } else {
                                    m_len = m_len.wrapping_add(1i32 as (usize));
                                    if *ip.offset(m_len as (isize)) as (i32) != *m_pos.offset(
                                                                                     m_len as (isize)
                                                                                 ) as (i32) {
                                        break 'loop22;
                                    } else {
                                        m_len = m_len.wrapping_add(1i32 as (usize));
                                        if *ip.offset(m_len as (isize)) as (i32) != *m_pos.offset(
                                                                                         m_len as (isize)
                                                                                     ) as (i32) {
                                            break 'loop22;
                                        } else {
                                            m_len = m_len.wrapping_add(1i32 as (usize));
                                            if *ip.offset(
                                                    m_len as (isize)
                                                ) as (i32) != *m_pos.offset(
                                                                   m_len as (isize)
                                                               ) as (i32) {
                                                break 'loop22;
                                            } else {
                                                m_len = m_len.wrapping_add(1i32 as (usize));
                                                if *ip.offset(
                                                        m_len as (isize)
                                                    ) as (i32) != *m_pos.offset(
                                                                       m_len as (isize)
                                                                   ) as (i32) {
                                                    break 'loop22;
                                                } else {
                                                    m_len = m_len.wrapping_add(1i32 as (usize));
                                                    if *ip.offset(
                                                            m_len as (isize)
                                                        ) as (i32) != *m_pos.offset(
                                                                           m_len as (isize)
                                                                       ) as (i32) {
                                                        break 'loop22;
                                                    } else {
                                                        m_len = m_len.wrapping_add(1i32 as (usize));
                                                        if ip.offset(m_len as (isize)) >= ip_end {
                                                            break 'loop22;
                                                        } else if *ip.offset(
                                                                       m_len as (isize)
                                                                   ) as (i32) == *m_pos.offset(
                                                                                      m_len as (isize)
                                                                                  ) as (i32) {
                                                            continue 'loop22;
                                                        } else {
                                                            break 'loop22;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    m_off = ((ip as (isize)).wrapping_sub(
                                 m_pos as (isize)
                             ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                    ip = ip.offset(m_len as (isize));
                    ii = ip;
                    if m_len <= 8i32 as (usize) && (m_off <= 0x800i32 as (usize)) {
                        m_off = m_off.wrapping_sub(1i32 as (usize));
                        *{
                             let _old = op;
                             op = op.offset(1 as (isize));
                             _old
                         } = (m_len.wrapping_sub(
                                  1i32 as (usize)
                              ) << 5i32 | (m_off & 7i32 as (usize)) << 2i32) as (u8);
                        *{
                             let _old = op;
                             op = op.offset(1 as (isize));
                             _old
                         } = (m_off >> 3i32) as (u8);
                        continue 'loop3;
                    } else if m_off <= 0x4000i32 as (usize) {
                        m_off = m_off.wrapping_sub(1i32 as (usize));
                        if m_len <= 33i32 as (usize) {
                            *{
                                 let _old = op;
                                 op = op.offset(1 as (isize));
                                 _old
                             } = (32i32 as (usize) | m_len.wrapping_sub(
                                                         2i32 as (usize)
                                                     )) as (u8);
                        } else {
                            m_len = m_len.wrapping_sub(33i32 as (usize));
                            *{
                                 let _old = op;
                                 op = op.offset(1 as (isize));
                                 _old
                             } = (32i32 | 0i32) as (u8);
                            'loop42: loop {
                                if m_len > 255i32 as (usize) {
                                    m_len = m_len.wrapping_sub(255i32 as (usize));
                                    *{
                                         let _old = op;
                                         op = op.offset(1 as (isize));
                                         _old
                                     } = 0i32 as (u8);
                                    continue 'loop42;
                                } else {
                                    break 'loop42;
                                }
                            }
                            *{
                                 let _old = op;
                                 op = op.offset(1 as (isize));
                                 _old
                             } = m_len as (u8);
                        }
                        *{
                             let _old = op;
                             op = op.offset(1 as (isize));
                             _old
                         } = (m_off << 2i32) as (u8);
                        *{
                             let _old = op;
                             op = op.offset(1 as (isize));
                             _old
                         } = (m_off >> 6i32) as (u8);
                        continue 'loop3;
                    } else {
                        m_off = m_off.wrapping_sub(0x4000i32 as (usize));
                        if m_len <= 9i32 as (usize) {
                            *{
                                 let _old = op;
                                 op = op.offset(1 as (isize));
                                 _old
                             } = (16i32 as (usize) | m_off >> 11i32 & 8i32 as (usize) | m_len.wrapping_sub(
                                                                                            2i32 as (usize)
                                                                                        )) as (u8);
                        } else {
                            m_len = m_len.wrapping_sub(9i32 as (usize));
                            *{
                                 let _old = op;
                                 op = op.offset(1 as (isize));
                                 _old
                             } = (16i32 as (usize) | m_off >> 11i32 & 8i32 as (usize)) as (u8);
                            'loop35: loop {
                                if m_len > 255i32 as (usize) {
                                    m_len = m_len.wrapping_sub(255i32 as (usize));
                                    *{
                                         let _old = op;
                                         op = op.offset(1 as (isize));
                                         _old
                                     } = 0i32 as (u8);
                                    continue 'loop35;
                                } else {
                                    break 'loop35;
                                }
                            }
                            *{
                                 let _old = op;
                                 op = op.offset(1 as (isize));
                                 _old
                             } = m_len as (u8);
                        }
                        *{
                             let _old = op;
                             op = op.offset(1 as (isize));
                             _old
                         } = (m_off << 2i32) as (u8);
                        *{
                             let _old = op;
                             op = op.offset(1 as (isize));
                             _old
                         } = (m_off >> 6i32) as (u8);
                        continue 'loop3;
                    }
                }
            }
        }
    }
    *out_len = ((op as (isize)).wrapping_sub(
                    out as (isize)
                ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
    ((in_end as (isize)).wrapping_sub(
         ii.offset(-(ti as (isize))) as (isize)
     ) / ::std::mem::size_of::<u8>() as (isize)) as (usize)
}

#[no_mangle]
pub unsafe extern fn lzo1x_1_compress(
    mut in_ : *const u8,
    mut in_len : usize,
    mut out : *mut u8,
    mut out_len : *mut usize,
    mut wrkmem : *mut ::std::os::raw::c_void
) -> i32 {
    let mut ip : *const u8 = in_;
    let mut op : *mut u8 = out;
    let mut l : usize = in_len;
    let mut t : usize = 0i32 as (usize);
    'loop1: loop {
        if l > 20i32 as (usize) {
            let mut ll
                : usize
                = if l <= (0xbfffi32 + 1i32) as (usize) {
                      l
                  } else {
                      (0xbfffi32 + 1i32) as (usize)
                  };
            let mut ll_end : usize = (ip as (usize)).wrapping_add(ll);
            if ll_end.wrapping_add(t.wrapping_add(ll) >> 5i32) <= ll_end {
                break 'loop1;
            } else {
                memset(
                    wrkmem,
                    0i32,
                    ((1u32 << 13i32) as (usize)).wrapping_mul(
                        ::std::mem::size_of::<u16>()
                    )
                );
                t = lzo1x_1_do_compress(ip,ll,op,out_len,t,wrkmem);
                ip = ip.offset(ll as (isize));
                op = op.offset(*out_len as (isize));
                l = l.wrapping_sub(ll);
                continue 'loop1;
            }
        } else {
            break 'loop1;
        }
    }
    t = t.wrapping_add(l);
    if t > 0i32 as (usize) {
        let mut ii
            : *const u8
            = in_.offset(in_len as (isize)).offset(-(t as (isize)));
        if op == out && (t <= 238i32 as (usize)) {
            *{
                 let _old = op;
                 op = op.offset(1 as (isize));
                 _old
             } = (17i32 as (usize)).wrapping_add(t) as (u8);
        } else if t <= 3i32 as (usize) {
            let _rhs = t;
            let _lhs = &mut *op.offset(-2i32 as (isize));
            *_lhs = (*_lhs as (usize) | _rhs) as (u8);
        } else if t <= 18i32 as (usize) {
            *{
                 let _old = op;
                 op = op.offset(1 as (isize));
                 _old
             } = t.wrapping_sub(3i32 as (usize)) as (u8);
        } else {
            let mut tt : usize = t.wrapping_sub(18i32 as (usize));
            *{
                 let _old = op;
                 op = op.offset(1 as (isize));
                 _old
             } = 0i32 as (u8);
            'loop9: loop {
                if tt > 255i32 as (usize) {
                    tt = tt.wrapping_sub(255i32 as (usize));
                    *{
                         let _old = op;
                         op = op.offset(1 as (isize));
                         _old
                     } = 0i32 as (u8);
                    continue 'loop9;
                } else {
                    break 'loop9;
                }
            }
            *{
                 let _old = op;
                 op = op.offset(1 as (isize));
                 _old
             } = tt as (u8);
        }
        if t >= 16i32 as (usize) {
            'loop16: loop {
                put_unaligned(
                    get_unaligned(
                        ii as (*const u32) as (*const ::std::os::raw::c_void)
                    ),
                    op as (*mut u32) as (*mut ::std::os::raw::c_void)
                );
                put_unaligned(
                    get_unaligned(
                        ii.offset(
                            4i32 as (isize)
                        ) as (*const u32) as (*const ::std::os::raw::c_void)
                    ),
                    op.offset(
                        4i32 as (isize)
                    ) as (*mut u32) as (*mut ::std::os::raw::c_void)
                );
                put_unaligned(
                    get_unaligned(
                        ii.offset(
                            8i32 as (isize)
                        ) as (*const u32) as (*const ::std::os::raw::c_void)
                    ),
                    op.offset(
                        8i32 as (isize)
                    ) as (*mut u32) as (*mut ::std::os::raw::c_void)
                );
                put_unaligned(
                    get_unaligned(
                        ii.offset(8i32 as (isize)).offset(
                            4i32 as (isize)
                        ) as (*const u32) as (*const ::std::os::raw::c_void)
                    ),
                    op.offset(8i32 as (isize)).offset(
                        4i32 as (isize)
                    ) as (*mut u32) as (*mut ::std::os::raw::c_void)
                );
                op = op.offset(16i32 as (isize));
                ii = ii.offset(16i32 as (isize));
                t = t.wrapping_sub(16i32 as (usize));
                if t >= 16i32 as (usize) {
                    continue 'loop16;
                } else {
                    break 'loop16;
                }
            }
        }
        if t > 0i32 as (usize) {
            'loop19: loop {
                *{
                     let _old = op;
                     op = op.offset(1 as (isize));
                     _old
                 } = *{
                          let _old = ii;
                          ii = ii.offset(1 as (isize));
                          _old
                      };
                if {
                       t = t.wrapping_sub(1 as (usize));
                       t
                   } > 0i32 as (usize) {
                    continue 'loop19;
                } else {
                    break 'loop19;
                }
            }
        }
    }
    *{
         let _old = op;
         op = op.offset(1 as (isize));
         _old
     } = (16i32 | 1i32) as (u8);
    *{
         let _old = op;
         op = op.offset(1 as (isize));
         _old
     } = 0i32 as (u8);
    *{
         let _old = op;
         op = op.offset(1 as (isize));
         _old
     } = 0i32 as (u8);
    *out_len = ((op as (isize)).wrapping_sub(
                    out as (isize)
                ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
    0i32
}
