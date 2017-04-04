extern {
    fn memcpy(
        __dest : *mut ::std::os::raw::c_void,
        __src : *const ::std::os::raw::c_void,
        __n : usize
    ) -> *mut ::std::os::raw::c_void;
}

unsafe extern fn get_unaligned_le16(
    mut p : *const ::std::os::raw::c_void
) -> u16 {
    let mut ret : u16 = 0i32 as (u16);
    memcpy(
        &mut ret as (*mut u16) as (*mut ::std::os::raw::c_void),
        p,
        ::std::mem::size_of::<u16>()
    );
    ret
}

#[no_mangle]
pub unsafe extern fn lzo1x_decompress_safe(
    mut in_ : *const u8,
    mut in_len : usize,
    mut out : *mut u8,
    mut out_len : *mut usize
) -> i32 {
    let mut op : *mut u8;
    let mut ip : *const u8;
    let mut t : usize;
    let mut next : usize;
    let mut state : usize = 0i32 as (usize);
    let mut m_pos : *const u8;
    let ip_end : *const u8 = in_.offset(in_len as (isize));
    let op_end : *mut u8 = out.offset(*out_len as (isize));
    op = out;
    ip = in_;
    if in_len < 3i32 as (usize) {
        *out_len = ((op as (isize)).wrapping_sub(
                        out as (isize)
                    ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
        -4i32
    } else {
        if *ip as (i32) > 17i32 {
            t = (*{
                      let _old = ip;
                      ip = ip.offset(1 as (isize));
                      _old
                  } as (i32) - 17i32) as (usize);
            if t < 4i32 as (usize) {
                next = t;
                state = next;
                t = next;
                if !(((ip_end as (isize)).wrapping_sub(
                          ip as (isize)
                      ) / ::std::mem::size_of::<u8>(
                          ) as (isize)) as (usize) >= t.wrapping_add(3i32 as (usize))) {
                    *out_len = ((op as (isize)).wrapping_sub(
                                    out as (isize)
                                ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                    return -4i32;
                } else if !(((op_end as (isize)).wrapping_sub(
                                 op as (isize)
                             ) / ::std::mem::size_of::<u8>() as (isize)) as (usize) >= t) {
                    *out_len = ((op as (isize)).wrapping_sub(
                                    out as (isize)
                                ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                    return -6i32;
                } else {
                    'loop12: loop {
                        if t > 0i32 as (usize) {
                            *{
                                 let _old = op;
                                 op = op.offset(1 as (isize));
                                 _old
                             } = *{
                                      let _old = ip;
                                      ip = ip.offset(1 as (isize));
                                      _old
                                  };
                            t = t.wrapping_sub(1 as (usize));
                            continue 'loop12;
                        } else {
                            break 'loop12;
                        }
                    }
                }
            } else if !(((op_end as (isize)).wrapping_sub(
                             op as (isize)
                         ) / ::std::mem::size_of::<u8>() as (isize)) as (usize) >= t) {
                *out_len = ((op as (isize)).wrapping_sub(
                                out as (isize)
                            ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                return -6i32;
            } else if !(((ip_end as (isize)).wrapping_sub(
                             ip as (isize)
                         ) / ::std::mem::size_of::<u8>(
                             ) as (isize)) as (usize) >= t.wrapping_add(3i32 as (usize))) {
                *out_len = ((op as (isize)).wrapping_sub(
                                out as (isize)
                            ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                return -4i32;
            } else {
                'loop5: loop {
                    *{
                         let _old = op;
                         op = op.offset(1 as (isize));
                         _old
                     } = *{
                              let _old = ip;
                              ip = ip.offset(1 as (isize));
                              _old
                          };
                    if {
                           t = t.wrapping_sub(1 as (usize));
                           t
                       } > 0i32 as (usize) {
                        continue 'loop5;
                    } else {
                        break 'loop5;
                    }
                }
                state = 4i32 as (usize);
            }
        }
        'loop13: loop {
            t = *{
                     let _old = ip;
                     ip = ip.offset(1 as (isize));
                     _old
                 } as (usize);
            let mut do_match_next : i32 = 0i32;
            if t < 16i32 as (usize) {
                if state == 0i32 as (usize) {
                    if t == 0i32 as (usize) {
                        let mut offset : usize;
                        let mut ip_last : *const u8 = ip;
                        'loop63: loop {
                            if *ip as (i32) == 0i32 {
                                ip = ip.offset(1 as (isize));
                                if !(((ip_end as (isize)).wrapping_sub(
                                          ip as (isize)
                                      ) / ::std::mem::size_of::<u8>(
                                          ) as (isize)) as (usize) >= 1i32 as (usize)) {
                                    break 'loop13;
                                } else {
                                    continue 'loop63;
                                }
                            } else {
                                break 'loop63;
                            }
                        }
                        offset = ((ip as (isize)).wrapping_sub(
                                      ip_last as (isize)
                                  ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                        if offset > (!0i32 as (usize)).wrapping_div(
                                        255i32 as (usize)
                                    ).wrapping_sub(
                                        2i32 as (usize)
                                    ) {
                            return -1i32;
                        } else {
                            offset = (offset << 8i32).wrapping_sub(offset);
                            t = t.wrapping_add(
                                    offset.wrapping_add(15i32 as (usize)).wrapping_add(
                                        *{
                                             let _old = ip;
                                             ip = ip.offset(1 as (isize));
                                             _old
                                         } as (usize)
                                    )
                                );
                        }
                    }
                    t = t.wrapping_add(3i32 as (usize));
                    if !(((op_end as (isize)).wrapping_sub(
                              op as (isize)
                          ) / ::std::mem::size_of::<u8>() as (isize)) as (usize) >= t) {
                        *out_len = ((op as (isize)).wrapping_sub(
                                        out as (isize)
                                    ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                        return -6i32;
                    } else if !(((ip_end as (isize)).wrapping_sub(
                                     ip as (isize)
                                 ) / ::std::mem::size_of::<u8>(
                                     ) as (isize)) as (usize) >= t.wrapping_add(3i32 as (usize))) {
                        *out_len = ((op as (isize)).wrapping_sub(
                                        out as (isize)
                                    ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                        return -4i32;
                    } else {
                        'loop69: loop {
                            *{
                                 let _old = op;
                                 op = op.offset(1 as (isize));
                                 _old
                             } = *{
                                      let _old = ip;
                                      ip = ip.offset(1 as (isize));
                                      _old
                                  };
                            if {
                                   t = t.wrapping_sub(1 as (usize));
                                   t
                               } > 0i32 as (usize) {
                                continue 'loop69;
                            } else {
                                break 'loop69;
                            }
                        }
                        state = 4i32 as (usize);
                        continue 'loop13;
                    }
                } else if state != 4i32 as (usize) {
                    next = t & 3i32 as (usize);
                    m_pos = op.offset(-(1i32 as (isize))) as (*const u8);
                    m_pos = m_pos.offset(-((t >> 2i32) as (isize)));
                    m_pos = m_pos.offset(
                                -((*{
                                        let _old = ip;
                                        ip = ip.offset(1 as (isize));
                                        _old
                                    } as (i32) << 2i32) as (isize))
                            );
                    if m_pos < out as (*const u8) {
                        *out_len = ((op as (isize)).wrapping_sub(
                                        out as (isize)
                                    ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                        return -6i32;
                    } else if !(((op_end as (isize)).wrapping_sub(
                                     op as (isize)
                                 ) / ::std::mem::size_of::<u8>(
                                     ) as (isize)) as (usize) >= 2i32 as (usize)) {
                        *out_len = ((op as (isize)).wrapping_sub(
                                        out as (isize)
                                    ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                        return -6i32;
                    } else {
                        *op.offset(0i32 as (isize)) = *m_pos.offset(0i32 as (isize));
                        *op.offset(1i32 as (isize)) = *m_pos.offset(1i32 as (isize));
                        op = op.offset(2i32 as (isize));
                        do_match_next = 1i32;
                    }
                } else {
                    next = t & 3i32 as (usize);
                    m_pos = op.offset(-((1i32 + 0x800i32) as (isize))) as (*const u8);
                    m_pos = m_pos.offset(-((t >> 2i32) as (isize)));
                    m_pos = m_pos.offset(
                                -((*{
                                        let _old = ip;
                                        ip = ip.offset(1 as (isize));
                                        _old
                                    } as (i32) << 2i32) as (isize))
                            );
                    t = 3i32 as (usize);
                }
            } else if t >= 64i32 as (usize) {
                next = t & 3i32 as (usize);
                m_pos = op.offset(-(1i32 as (isize))) as (*const u8);
                m_pos = m_pos.offset(-((t >> 2i32 & 7i32 as (usize)) as (isize)));
                m_pos = m_pos.offset(
                            -((*{
                                    let _old = ip;
                                    ip = ip.offset(1 as (isize));
                                    _old
                                } as (i32) << 3i32) as (isize))
                        );
                t = (t >> 5i32).wrapping_sub(1i32 as (usize)).wrapping_add(
                        (3i32 - 1i32) as (usize)
                    );
            } else if t >= 32i32 as (usize) {
                t = (t & 31i32 as (usize)).wrapping_add((3i32 - 1i32) as (usize));
                if t == 2i32 as (usize) {
                    let mut offset : usize;
                    let mut ip_last : *const u8 = ip;
                    'loop30: loop {
                        if *ip as (i32) == 0i32 {
                            ip = ip.offset(1 as (isize));
                            if !(((ip_end as (isize)).wrapping_sub(
                                      ip as (isize)
                                  ) / ::std::mem::size_of::<u8>(
                                      ) as (isize)) as (usize) >= 1i32 as (usize)) {
                                *out_len = ((op as (isize)).wrapping_sub(
                                                out as (isize)
                                            ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                                return -4i32;
                            } else {
                                continue 'loop30;
                            }
                        } else {
                            break 'loop30;
                        }
                    }
                    offset = ((ip as (isize)).wrapping_sub(
                                  ip_last as (isize)
                              ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                    if offset > (!0i32 as (usize)).wrapping_div(
                                    255i32 as (usize)
                                ).wrapping_sub(
                                    2i32 as (usize)
                                ) {
                        return -1i32;
                    } else {
                        offset = (offset << 8i32).wrapping_sub(offset);
                        t = t.wrapping_add(
                                offset.wrapping_add(31i32 as (usize)).wrapping_add(
                                    *{
                                         let _old = ip;
                                         ip = ip.offset(1 as (isize));
                                         _old
                                     } as (usize)
                                )
                            );
                        if !(((ip_end as (isize)).wrapping_sub(
                                  ip as (isize)
                              ) / ::std::mem::size_of::<u8>(
                                  ) as (isize)) as (usize) >= 2i32 as (usize)) {
                            *out_len = ((op as (isize)).wrapping_sub(
                                            out as (isize)
                                        ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                            return -4i32;
                        }
                    }
                }
                m_pos = op.offset(-(1i32 as (isize))) as (*const u8);
                next = get_unaligned_le16(
                           ip as (*const ::std::os::raw::c_void)
                       ) as (usize);
                ip = ip.offset(2i32 as (isize));
                m_pos = m_pos.offset(-((next >> 2i32) as (isize)));
                next = next & 3i32 as (usize);
            } else {
                m_pos = op as (*const u8);
                m_pos = m_pos.offset(
                            -(((t & 8i32 as (usize)) << 11i32) as (isize))
                        );
                t = (t & 7i32 as (usize)).wrapping_add((3i32 - 1i32) as (usize));
                if t == 2i32 as (usize) {
                    let mut offset : usize;
                    let mut ip_last : *const u8 = ip;
                    'loop18: loop {
                        if *ip as (i32) == 0i32 {
                            ip = ip.offset(1 as (isize));
                            if !(((ip_end as (isize)).wrapping_sub(
                                      ip as (isize)
                                  ) / ::std::mem::size_of::<u8>(
                                      ) as (isize)) as (usize) >= 1i32 as (usize)) {
                                *out_len = ((op as (isize)).wrapping_sub(
                                                out as (isize)
                                            ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                                return -4i32;
                            } else {
                                continue 'loop18;
                            }
                        } else {
                            break 'loop18;
                        }
                    }
                    offset = ((ip as (isize)).wrapping_sub(
                                  ip_last as (isize)
                              ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                    if offset > (!0i32 as (usize)).wrapping_div(
                                    255i32 as (usize)
                                ).wrapping_sub(
                                    2i32 as (usize)
                                ) {
                        return -1i32;
                    } else {
                        offset = (offset << 8i32).wrapping_sub(offset);
                        t = t.wrapping_add(
                                offset.wrapping_add(7i32 as (usize)).wrapping_add(
                                    *{
                                         let _old = ip;
                                         ip = ip.offset(1 as (isize));
                                         _old
                                     } as (usize)
                                )
                            );
                        if !(((ip_end as (isize)).wrapping_sub(
                                  ip as (isize)
                              ) / ::std::mem::size_of::<u8>(
                                  ) as (isize)) as (usize) >= 2i32 as (usize)) {
                            *out_len = ((op as (isize)).wrapping_sub(
                                            out as (isize)
                                        ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                            return -4i32;
                        }
                    }
                }
                next = get_unaligned_le16(
                           ip as (*const ::std::os::raw::c_void)
                       ) as (usize);
                ip = ip.offset(2i32 as (isize));
                m_pos = m_pos.offset(-((next >> 2i32) as (isize)));
                next = next & 3i32 as (usize);
                if m_pos == op as (*const u8) {
                    *out_len = ((op as (isize)).wrapping_sub(
                                    out as (isize)
                                ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                    return
                        if t != 3i32 as (usize) {
                            -1i32
                        } else if ip == ip_end {
                            0i32
                        } else if ip < ip_end {
                            -8i32
                        } else {
                            -4i32
                        };
                } else {
                    m_pos = m_pos.offset(-(0x4000i32 as (isize)));
                }
            }
            if do_match_next == 0 {
                if m_pos < out as (*const u8) {
                    *out_len = ((op as (isize)).wrapping_sub(
                                    out as (isize)
                                ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                    return -6i32;
                } else {
                    let mut oe : *mut u8 = op.offset(t as (isize));
                    if !(((op_end as (isize)).wrapping_sub(
                              op as (isize)
                          ) / ::std::mem::size_of::<u8>() as (isize)) as (usize) >= t) {
                        *out_len = ((op as (isize)).wrapping_sub(
                                        out as (isize)
                                    ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                        return -6i32;
                    } else {
                        *op.offset(0i32 as (isize)) = *m_pos.offset(0i32 as (isize));
                        *op.offset(1i32 as (isize)) = *m_pos.offset(1i32 as (isize));
                        op = op.offset(2i32 as (isize));
                        m_pos = m_pos.offset(2i32 as (isize));
                        'loop49: loop {
                            *{
                                 let _old = op;
                                 op = op.offset(1 as (isize));
                                 _old
                             } = *{
                                      let _old = m_pos;
                                      m_pos = m_pos.offset(1 as (isize));
                                      _old
                                  };
                            if op < oe {
                                continue 'loop49;
                            } else {
                                break 'loop49;
                            }
                        }
                    }
                }
            }
            state = next;
            t = next;
            if !(((ip_end as (isize)).wrapping_sub(
                      ip as (isize)
                  ) / ::std::mem::size_of::<u8>(
                      ) as (isize)) as (usize) >= t.wrapping_add(3i32 as (usize))) {
                *out_len = ((op as (isize)).wrapping_sub(
                                out as (isize)
                            ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                return -4i32;
            } else if !(((op_end as (isize)).wrapping_sub(
                             op as (isize)
                         ) / ::std::mem::size_of::<u8>() as (isize)) as (usize) >= t) {
                *out_len = ((op as (isize)).wrapping_sub(
                                out as (isize)
                            ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
                return -6i32;
            } else {
                'loop53: loop {
                    if t > 0i32 as (usize) {
                        *{
                             let _old = op;
                             op = op.offset(1 as (isize));
                             _old
                         } = *{
                                  let _old = ip;
                                  ip = ip.offset(1 as (isize));
                                  _old
                              };
                        t = t.wrapping_sub(1 as (usize));
                        continue 'loop53;
                    } else {
                        continue 'loop13;
                    }
                }
            }
        }
        *out_len = ((op as (isize)).wrapping_sub(
                        out as (isize)
                    ) / ::std::mem::size_of::<u8>() as (isize)) as (usize);
        -4i32
    }
}
