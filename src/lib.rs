pub unsafe fn memchr_avx2(needle: u8, haystack: &[u8]) -> Option<usize> {
    use std::arch::x86_64::*;
    let q = _mm256_set1_epi8(needle as i8);

    let start = haystack.as_ptr();
    let end = start.offset(haystack.len() as isize);

    let mut i: *const u8 = start;
    
    while i.offset(32) < end {
        let x = _mm256_lddqu_si256(i as *const __m256i);
        let r = _mm256_cmpeq_epi8(x, q);
        let z = _mm256_movemask_epi8(r);
        if z != 0 {
            // TODO: cttz_nonzero better than trailing_zeros?
            return Some(i as usize - start as usize + z.trailing_zeros() as usize);
        }

        i = i.offset(32);
    }

    if i < end {
        if (end as usize - start as usize) < 32 {
            while i < end {
                if *i == needle {
                    return Some(i as usize - start as usize);
                }
                i = i.offset(1);
            }
        } else {
            i = end.offset(-32);
            let x = _mm256_lddqu_si256(i as *const __m256i);
            let r = _mm256_cmpeq_epi8(x, q);
            let z = _mm256_movemask_epi8(r);
            if z != 0 {
                // TODO: cttz_nonzero better than trailing_zeros?
                return Some(i as usize - start as usize + z.trailing_zeros() as usize);
            }
        }
    }

    None
}

pub unsafe fn memchr_sse(needle: u8, haystack: &[u8]) -> Option<usize> {
    use std::arch::x86_64::*;
    let q = _mm_set1_epi8(needle as i8);

    let start = haystack.as_ptr();
    let end = start.offset(haystack.len() as isize);

    let mut i: *const u8 = start;
    
    while i.offset(16) < end {
        let x = _mm_lddqu_si128(i as *const __m128i);
        let r = _mm_cmpeq_epi8(x, q);
        let z = _mm_movemask_epi8(r);
        if z != 0 {
            // TODO: cttz_nonzero better than trailing_zeros?
            return Some(i as usize - start as usize + z.trailing_zeros() as usize);
        }

        i = i.offset(16);
    }

    if i < end {
        if (end as usize - start as usize) < 16 {
            while i < end {
                if *i == needle {
                    return Some(i as usize - start as usize);
                }
                i = i.offset(1);
            }
        } else {
            i = end.offset(-16);
            let x = _mm_lddqu_si128(i as *const __m128i);
            let r = _mm_cmpeq_epi8(x, q);
            let z = _mm_movemask_epi8(r);
            if z != 0 {
                // TODO: cttz_nonzero better than trailing_zeros?
                return Some(i as usize - start as usize + z.trailing_zeros() as usize);
            }
        }
    }

    None
}
