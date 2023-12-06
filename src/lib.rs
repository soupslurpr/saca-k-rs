/*
Adapted from SACA-K C++ demo source code.
Check original-code-attribution.txt.
*/

// Set only the highest bit as 1, i.e. 1000...
const EMPTY_U32: u32 = 1 << (std::mem::size_of::<u32>() * 8 - 1);
const EMPTY_I32: i32 = 1 << (std::mem::size_of::<i32>() * 8 - 1);

fn get_buckets(s: &[u8], bkt: &mut [u32], n: u32, k: u32, end: bool) {
    let mut i: u32;
    let mut sum: u32 = 0;

    // Clear all buckets.
    i = 0;
    while i < k {
        bkt[i as usize] = 0;
        i = i.wrapping_add(1);
    }

    // Compute the size of each bucket.
    i = 0;
    while i < n {
        bkt[s[i as usize] as usize] = bkt[s[i as usize] as usize].wrapping_add(1);
        i = i.wrapping_add(1);
    }

    i = 0;
    while i < k {
        sum = sum.wrapping_add(bkt[i as usize]);
        bkt[i as usize] = if end {
            sum.wrapping_sub(1)
        } else {
            sum.wrapping_sub(bkt[i as usize])
        };
        i = i.wrapping_add(1);
    }
}

fn put_suffix0(sa: &mut [u32], s: &[u8], bkt: &mut [u32], n: u32, k: u32, n1: i32) {
    let mut i: u32;
    let mut j: u32;

    // Find the end of each bucket.
    get_buckets(s, bkt, n, k, true);

    // Put the suffixes into their buckets.
    i = (n1 - 1) as u32;
    while i > 0 {
        j = sa[i as usize];
        sa[i as usize] = 0;

        sa[bkt[s[j as usize] as usize] as usize] = j;
        bkt[s[j as usize] as usize] = bkt[s[j as usize] as usize].wrapping_sub(1);

        i = i.wrapping_sub(1);
    }

    sa[0] = n.wrapping_sub(1); // Set the single sentinel suffix.
}

fn induce_sal0(sa: &mut [u32], s: &[u8], bkt: &mut [u32], n: u32, k: u32, suffix: bool) {
    let mut i: u32;
    let mut j: u32;

    // Find the head of each bucket.
    get_buckets(s, bkt, n, k, false);

    bkt[0] = bkt[0].wrapping_add(1); // Skip the virtual sentinel.

    i = 0;
    while i < n {
        if sa[i as usize] > 0 {
            j = sa[i as usize].wrapping_sub(1);
            if s[j as usize] >= s[(j + 1) as usize] {
                sa[bkt[s[j as usize] as usize] as usize] = j;
                bkt[s[j as usize] as usize] = bkt[s[j as usize] as usize].wrapping_add(1);

                if !suffix && (i > 0) {
                    sa[i as usize] = 0;
                }
            }
        }

        i = i.wrapping_add(1);
    }
}

fn induce_sas0(sa: &mut [u32], s: &[u8], bkt: &mut [u32], n: u32, k: u32, suffix: bool) {
    let mut i: u32;
    let mut j: u32;

    // Find the end of each bucket.
    get_buckets(s, bkt, n, k, true);

    i = n.wrapping_sub(1);
    while i > 0 {
        if sa[i as usize] > 0 {
            j = sa[i as usize].wrapping_sub(1);
            if (s[j as usize] <= s[j.wrapping_add(1) as usize])
                && ((bkt[s[j as usize] as usize] as usize) < (i as usize))
            {
                sa[bkt[s[j as usize] as usize] as usize] = j;
                bkt[s[j as usize] as usize] = bkt[s[j as usize] as usize].wrapping_sub(1);

                if !suffix {
                    sa[i as usize] = 0;
                }
            }
        }

        i = i.wrapping_sub(1);
    }
}

fn put_substr0(sa: &mut [u32], s: &[u8], bkt: &mut [u32], n: u32, k: u32) {
    let mut i: u32;
    let mut cur_t: bool;
    let mut succ_t: bool;

    // Find the end of each bucket.
    get_buckets(s, bkt, n, k, true);

    // Set each item in sa as empty.
    i = 0;
    while i < n {
        sa[i as usize] = 0;
        i = i.wrapping_add(1);
    }

    succ_t = false; // s[n.wrapping_sub(2) as usize] must be L-type.
    i = n.wrapping_sub(2);
    while i > 0 {
        cur_t = (s[(i - 1) as usize] < s[i as usize])
            || (s[(i - 1) as usize] == s[i as usize]) && succ_t;

        if !cur_t && succ_t {
            sa[bkt[s[i as usize] as usize] as usize] = i;
            bkt[s[i as usize] as usize] = bkt[s[i as usize] as usize].wrapping_sub(1);
        }
        succ_t = cur_t;

        i = i.wrapping_sub(1);
    }

    // Set the single sentinel LMS-substring.
    sa[0] = n.wrapping_sub(1);
}

fn put_suffix1(sa: &mut [i32], s: &[i32], n1: i32) {
    let mut i: i32;
    let mut j: i32;
    let mut pos: i32 = 0; // In the original code, pos seems to be left uninitialized.
    let mut cur: i32;
    let mut pre: i32 = -1;

    i = n1 - 1;
    while i > 0 {
        j = sa[i as usize];
        sa[i as usize] = EMPTY_I32;

        cur = s[j as usize];

        if cur != pre {
            pre = cur;
            pos = cur;
        }

        sa[pos as usize] = j;
        pos -= 1;

        i -= 1;
    }
}

fn induce_sal1(sa: &mut [i32], s: &[i32], n: i32, suffix: bool) {
    let mut h: i32;
    let mut i: i32;
    let mut j: i32;
    let mut step: i32 = 1;

    i = 0;
    while i < n {
        step = 1;
        j = sa[i as usize] - 1;

        if sa[i as usize] <= 0 {
            i += step;
            continue;
        }

        let c: i32 = s[j as usize];
        let c1: i32 = s[(j + 1) as usize];

        let is_l: bool = c >= c1;

        if !is_l {
            i += step;
            continue;
        }

        // s[j] is L-type.

        let mut d: i32 = sa[c as usize];
        if d >= 0 {
            // sa[c] is borrowed by the left
            // neighbor bucket.
            // shift-left the items in the
            // left neighbor bucket.
            let mut foo: i32;
            let mut bar: i32;

            foo = sa[c as usize];

            h = c - 1;
            while sa[h as usize] >= 0 || sa[h as usize] == EMPTY_I32 {
                bar = sa[h as usize];
                sa[h as usize] = foo;
                foo = bar;

                h -= 1;
            }

            sa[h as usize] = foo;

            if h < i {
                step = 0;
            }

            d = EMPTY_I32;
        }

        if d == EMPTY_I32 {
            // sa[c] is empty.
            if (c < n - 1) && (sa[(c + 1) as usize] == EMPTY_I32) {
                sa[c as usize] = -1; // Init the counter.
                sa[(c + 1) as usize] = j;
            } else {
                sa[c as usize] = j; // A size-1 bucket.
            }
        } else {
            // sa[c] is reused as a counter.
            let mut pos: i32 = c - d + 1;

            if (pos > (n - 1)) || (sa[pos as usize] != EMPTY_I32) {
                // We are running into the right
                // neighbor bucket.
                // Shift-left one step the items
                // of bucket(sa, s, j).
                h = 0;
                while h < -d {
                    sa[(c + h) as usize] = sa[(c + h + 1) as usize];
                    h += 1;
                }

                pos -= 1;

                if c < i {
                    step = 0;
                }
            } else {
                sa[c as usize] -= 1;
            }

            sa[pos as usize] = j;
        }

        let c2: i32 = s[(j + 2) as usize];
        let is_l1: bool = (j + 1 < n - 1) && (c1 > c2) || ((c1 == c2) && (c1 < i)); // Is s[sa[i]] L-type?

        if (!suffix || !is_l1) && (i > 0) {
            let i1: i32 = if step == 0 { i - 1 } else { i };
            sa[i1 as usize] = EMPTY_I32;
        }
        i += step;
    }

    // Scan to shift-left the items in each bucket
    // with its head being reused as a counter.
    i = 1;
    while i < n {
        j = sa[i as usize];

        if (j < 0) && (j != EMPTY_I32) {
            // is sa[i] a counter?
            h = 0;
            while h < -j {
                sa[(i + h) as usize] = sa[(i + h + 1) as usize];
                h += 1;
            }
            sa[(i + h) as usize] = EMPTY_I32;
        }
        i += 1
    }
}

fn induce_sas1(sa: &mut [i32], s: &[i32], n: i32, suffix: bool) {
    let mut h: i32;
    let mut i: i32;
    let mut j: i32;
    let mut step: i32 = 1;

    i = n - 1;
    while i > 0 {
        step = 1;
        j = sa[i as usize] - 1;

        if sa[i as usize] <= 0 {
            i -= step;
            continue;
        }

        let c: i32 = s[j as usize];

        let c1: i32 = s[(j + 1) as usize];

        let is_s: bool = (c < c1) || ((c == c1) && (c > i));

        if !is_s {
            i -= step;
            continue;
        }

        // s[j] is S-type.

        let d: i32 = sa[c as usize];

        if d >= 0 {
            // sa[c] is borrowed by the right
            // neighbor bucket.
            // Shift-right the items in the
            // right neighbor bucket.
            let mut foo: i32;
            let mut bar: i32;

            foo = sa[c as usize];

            h = c + 1;

            while sa[h as usize] >= 0 || sa[h as usize] == EMPTY_I32 {
                bar = sa[h as usize];
                sa[h as usize] = foo;
                foo = bar;

                h += 1;
            }

            sa[h as usize] = foo;

            if h > i {
                step = 0;
            }
        }

        if d == EMPTY_I32 {
            // sa[c] is empty.
            if sa[(c - 1) as usize] == EMPTY_I32 {
                sa[c as usize] -= 1; // Init the counter.
                sa[(c - 1) as usize] = j;
            } else {
                sa[c as usize] = j; // A size-1 bucket.
            }
        } else {
            // sa[c] is reused as a counter.
            let mut pos: i32 = c + d - 1;

            if sa[pos as usize] != EMPTY_I32 {
                // We are running into the left
                // neighbor bucket.
                // Shift-right one step the items
                // of bucket(sa, s, j).
                h = 0;
                while h < -d {
                    sa[(c - h) as usize] = sa[(c - h - 1) as usize];
                    h += 1;
                }

                pos += 1;

                if c > i {
                    step = 0;
                }
            } else {
                sa[c as usize] -= 1;
            }

            sa[pos as usize] = j;
        }

        if !suffix {
            let i1: i32 = if step == 0 { i + 1 } else { i };
            sa[i1 as usize] = EMPTY_I32;
        }

        i -= step;
    }

    // Scan to shift-right the items in each bucket
    // with its head being reused as a counter.
    if !suffix {
        i = n - 1;
        while i > 0 {
            j = sa[i as usize];

            if (j < 0) && (j != EMPTY_I32) {
                // is sa[i] a counter?
                h = 0;
                while h < -j {
                    sa[(i - h) as usize] = sa[(i - h - 1) as usize];
                    h += 1;
                }
                sa[(i - h) as usize] = EMPTY_I32;
            }

            i -= 1;
        }
    }
}

fn put_substr1(sa: &mut [i32], s: &[i32], n: i32) {
    let mut h: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;

    while i < n {
        sa[i as usize] = EMPTY_I32;
        i += 1;
    }

    let mut c: i32;
    let mut c1: i32 = s[(n - 2) as usize];
    let mut t: bool;
    let mut t1: bool = false;

    i = n - 2;
    while i > 0 {
        c = c1;
        t = t1;

        c1 = s[(i - 1) as usize];
        t1 = (c1 < c) || ((c1 == c) && t);

        #[allow(clippy::collapsible_if)]
        if t && !t1 {
            if sa[c as usize] >= 0 {
                // sa[c] is borrowed by the right
                // neighbor bucket.
                // Shift-right the items in the
                // right neighbor bucket.
                let mut foo: i32;
                let mut bar: i32;

                foo = sa[c as usize];
                h = c + 1;
                while sa[h as usize] >= 0 {
                    bar = sa[h as usize];
                    sa[h as usize] = foo;
                    foo = bar;
                    h += 1;
                }

                sa[h as usize] = foo;

                sa[c as usize] = EMPTY_I32;
            }

            let d = sa[c as usize];
            if d == EMPTY_I32 {
                // sa[c] is empty.
                if sa[(c - 1) as usize] == EMPTY_I32 {
                    sa[c as usize] = -1; // Init the counter.
                    sa[(c - 1) as usize] = i;
                } else {
                    sa[c as usize] = i; // A size-1 bucket.
                }
            } else {
                // sa[c] is reused as a counter
                let mut pos = c + d - 1;

                if sa[pos as usize] != EMPTY_I32 {
                    // We are running into the left
                    // neighbor bucket.
                    // Shift-right one step the items
                    // of bucket(sa, s, i).
                    h = 0;
                    while h < -d {
                        sa[(c - h) as usize] = sa[(c - h - 1) as usize];
                        h += 1
                    }
                    pos += 1;
                } else {
                    sa[c as usize] -= 1;
                }
                sa[pos as usize] = i;
            }
        }
        i -= 1;
    }

    // Scan to shift-right the items in each bucket
    // with its head being reused as a counter.
    i = n - 1;
    while i > 0 {
        j = sa[i as usize];

        if (j < 0) && (j != EMPTY_I32) {
            // Is sa[i] a counter?
            h = 0;
            while h < -j {
                sa[(i - h) as usize] = sa[(i - h - 1) as usize];
                h += 1;
            }
            sa[(i - h) as usize] = EMPTY_I32;
        }

        i -= 1;
    }

    // Put the single sentinel LMS-substring.
    sa[0] = n - 1;
}

fn get_length_of_lms(s: &[u8], n: u32, level: i32, x: u32) -> u32 {
    if x == n - 1 {
        return 1;
    };

    let mut dist: u32 = 0;
    let mut i: u32 = 1;

    #[allow(clippy::nonminimal_bool)]
    while !((if level == 0 {
        // bytemuck::cast_slice::<u8, i32>(s)[(x + i) as usize]
        s[x.wrapping_add(i) as usize] as i32
    } else {
        // [u8] to [i32]
        let i32_slice: &[i32] = bytemuck::cast_slice(s);

        *i32_slice.get(x.wrapping_add(i) as usize).unwrap()
    }) < (if level == 0 {
        s[x.wrapping_add(i).wrapping_sub(1) as usize] as i32
    } else {
        // [u8] to [i32]
        let i32_slice: &[i32] = bytemuck::cast_slice(s);

        *i32_slice
            .get(x.wrapping_add(i).wrapping_sub(1) as usize)
            .unwrap()
    })) {
        i = i.wrapping_add(1);
    }

    while !(x.wrapping_add(i) > n.wrapping_sub(1)
        || (if level == 0 {
            s[x.wrapping_add(i) as usize] as i32
        } else {
            // [u8] to [i32]
            let i32_slice: &[i32] = bytemuck::cast_slice(s);

            *i32_slice.get(x.wrapping_add(i) as usize).unwrap()
        }) > (if level == 0 {
            s[x.wrapping_add(i).wrapping_sub(1) as usize] as i32
        } else {
            // [u8] to [i32]
            let i32_slice: &[i32] = bytemuck::cast_slice(s);

            *i32_slice
                .get(x.wrapping_add(i).wrapping_sub(1) as usize)
                .unwrap()
        }))
    {
        if x.wrapping_add(i) == n.wrapping_sub(1)
            || (if level == 0 {
                s[x.wrapping_add(i) as usize] as i32
            } else {
                // [u8] to [i32]
                let i32_slice: &[i32] = bytemuck::cast_slice(s);

                *i32_slice.get(x.wrapping_add(i) as usize).unwrap()
            }) < (if level == 0 {
                s[x.wrapping_add(i).wrapping_sub(1) as usize] as i32
            } else {
                // [u8] to [i32]
                let i32_slice: &[i32] = bytemuck::cast_slice(s);

                *i32_slice
                    .get(x.wrapping_add(i).wrapping_sub(1) as usize)
                    .unwrap()
            })
        {
            dist = i;
        }
        i = i.wrapping_add(1);
    }
    dist.wrapping_add(1)
}

fn name_substr(
    sa: &mut [u32],
    s: &[u8],
    s1: &mut [u32],
    n: u32,
    m: u32,
    n1: u32,
    level: i32,
) -> u32 {
    let mut i: u32;
    let mut j: u32;
    let mut cur_t: u32;
    let mut succ_t: u32;

    // Init the name array buffer
    i = n1;
    while i < n {
        sa[i as usize] = EMPTY_U32;
        i = i.wrapping_add(1);
    }

    // Scan to compute the interim s1
    let mut name: u32 = 0; // Was left uninitialized in the original code
    let mut name_ctr: u32 = 0;
    let mut pre_pos: u32 = 0; // Was left uninitialized in the original code
    let mut pre_len: u32 = 0;
    i = 0;
    while i < n1 {
        let mut diff: bool = false;
        let pos: u32 = sa[i as usize];

        let len: u32 = get_length_of_lms(s, n, level, pos);

        if len != pre_len {
            diff = true;
        } else {
            let mut d: u32 = 0;
            while d < len {
                if pos.wrapping_add(d) == n.wrapping_sub(1)
                    || pre_pos.wrapping_add(d) == n.wrapping_sub(1)
                    || (if level == 0 {
                        s[pos.wrapping_add(d) as usize] as i32
                    } else {
                        // [u8] to [i32]
                        let i32_slice: &[i32] = bytemuck::cast_slice(s);

                        *i32_slice.get(pos.wrapping_add(d) as usize).unwrap()
                    }) != (if level == 0 {
                        s[pre_pos.wrapping_add(d) as usize] as i32
                    } else {
                        // [u8] to [i32]
                        let i32_slice: &[i32] = bytemuck::cast_slice(s);

                        *i32_slice.get(pre_pos.wrapping_add(d) as usize).unwrap()
                    })
                {
                    diff = true;
                    break;
                }
                d = d.wrapping_add(1);
            }
        }
        if diff {
            name = i;
            name_ctr = name_ctr.wrapping_add(1);
            sa[name as usize] = 1; // A new name.
            pre_pos = pos;
            pre_len = len;
        } else {
            sa[name as usize] = sa[name as usize].wrapping_add(1); // Count this name.
        }
        sa[n1.wrapping_add(pos.wrapping_div(2)) as usize] = name;
        i = i.wrapping_add(1)
    }

    // Compact the interim s1 sparsely stored
    // in SA[n1, n-1] into SA[m-n1, m-1].
    i = n.wrapping_sub(1);
    j = m.wrapping_sub(1);
    while i >= n1 {
        if sa[i as usize] != EMPTY_U32 {
            sa[j as usize] = sa[i as usize];
            j = j.wrapping_sub(1);
        }

        i = i.wrapping_sub(1);
    }

    // Rename each S-type character of the
    // interim s1 as the end of its bucket
    // to produce the final s1.
    succ_t = 1;
    i = n1.wrapping_sub(1);
    while i > 0 {
        let ch = s1[i as usize];
        let ch1 = s1[i.wrapping_sub(1) as usize];

        cur_t = if ch1 < ch || ((ch1 == ch) && (succ_t == 1)) {
            1
        } else {
            0
        };

        if cur_t == 1 {
            s1[i.wrapping_sub(1) as usize] = s1[i.wrapping_sub(1) as usize]
                .wrapping_add(sa[s1[i.wrapping_sub(1) as usize] as usize].wrapping_sub(1));
        }
        succ_t = cur_t;
        i = i.wrapping_sub(1);
    }

    name_ctr
}

fn get_sa_lms(sa: &mut [u32], s: &mut [u8], s1: &mut [u32], n: u32, n1: u32, level: i32) {
    let mut i: u32;
    let mut j: u32;
    let mut cur_t: u32;
    let mut succ_t: u32;

    j = n1.wrapping_sub(1);

    s1[j as usize] = n.wrapping_sub(1);
    j = j.wrapping_sub(1);

    succ_t = 0; // s[n.wrapping_sub(2) must be L-type

    i = n.wrapping_sub(2);
    while i > 0 {
        cur_t = if ((if level == 0 {
            s[i.wrapping_sub(1) as usize] as i32
        } else {
            // [u8] to [i32]
            let i32_slice: &[i32] = bytemuck::cast_slice(s);

            *i32_slice.get(i.wrapping_sub(1) as usize).unwrap()
        }) < (if level == 0 {
            s[i as usize] as i32
        } else {
            // [u8] to [i32]
            let i32_slice: &[i32] = bytemuck::cast_slice(s);

            *i32_slice.get(i as usize).unwrap()
        })) || ((if level == 0 {
            s[i.wrapping_sub(1) as usize] as i32
        } else {
            // [u8] to [i32]
            let i32_slice: &[i32] = bytemuck::cast_slice(s);

            *i32_slice.get(i.wrapping_sub(1) as usize).unwrap()
        }) == (if level == 0 {
            s[i as usize] as i32
        } else {
            // [u8] to [i32]
            let i32_slice: &[i32] = bytemuck::cast_slice(s);

            *i32_slice.get(i as usize).unwrap()
        }) && (succ_t == 1))
        {
            1
        } else {
            0
        };

        if (cur_t == 0) && (succ_t == 1) {
            s1[j as usize] = i;
            j = j.wrapping_sub(1);
        }

        succ_t = cur_t;

        i = i.wrapping_sub(1);
    }

    i = 0;
    while i < n1 {
        sa[i as usize] = s1[sa[i as usize] as usize];
        i = i.wrapping_add(1);
    }

    // Init sa[n1..n.wrapping_sub(1)]
    i = n1;
    while i < n {
        sa[i as usize] = if level != 0 { EMPTY_U32 } else { 0 };
        i = i.wrapping_add(1);
    }
}

pub fn saca_k(s: &mut [u8], sa: &mut [u32], n: u32, k: u32, m: u32, level: i32) {
    let mut i;
    let mut bkt = vec![];

    // Stage 1: Reduce the problem by at least 1/2.
    if level == 0 {
        let size = std::mem::size_of::<i32>().wrapping_mul(k as usize);
        bkt = vec![0; size];

        put_substr0(sa, s, &mut bkt, n, k);
        induce_sal0(sa, s, &mut bkt, n, k, false);
        induce_sas0(sa, s, &mut bkt, n, k, false);
    } else {
        put_substr1(
            bytemuck::cast_slice_mut::<u32, i32>(sa),
            bytemuck::cast_slice::<u8, i32>(s),
            bytemuck::cast::<u32, i32>(n),
        );
        induce_sal1(
            bytemuck::cast_slice_mut::<u32, i32>(sa),
            bytemuck::cast_slice::<u8, i32>(s),
            bytemuck::cast::<u32, i32>(n),
            false,
        );
        induce_sas1(
            bytemuck::cast_slice_mut::<u32, i32>(sa),
            bytemuck::cast_slice::<u8, i32>(s),
            bytemuck::cast::<u32, i32>(n),
            false,
        );
    }

    // Now, all the LMS-substrings are sorted and
    // stored sparsely in sa.

    // Compact all the sorted substrings into
    // the first n1 items of sa.
    // 2.wrapping_mul(n1) must be not larger than n.
    let mut n1: u32 = 0;
    i = 0;
    while i < n {
        if ((level == 0) && sa[i as usize] > 0)
            || ((level != 0) && (bytemuck::cast_slice::<u32, i32>(sa)[i as usize] > 0))
        {
            sa[n1 as usize] = sa[i as usize];
            n1 = n1.wrapping_add(1);
        }

        i = i.wrapping_add(1);
    }

    let mut sa1: Vec<u32> = sa.to_vec();
    let mut s1 = sa[(m - n1) as usize..].to_vec(); // Unsure if this is a correct translation.
    let name_ctr = name_substr(sa, s, &mut s1, n, m, n1, level);

    // Stage 2: Solve the reduced problem.

    // Recurse if names are not yet unique.
    if name_ctr < n1 {
        saca_k(
            bytemuck::cast_slice_mut::<u32, u8>(&mut s1),
            &mut sa1,
            n1,
            0,
            m.wrapping_sub(n1),
            level + 1,
        );
    } else {
        // get the suffix array of s1 directly.
        i = 0;
        while i < n1 {
            sa1[s1[i as usize] as usize] = i;
            i = i.wrapping_add(1);
        }
    }

    // Stage 3: Induce sa(s) from sa(s1).

    get_sa_lms(sa, s, &mut s1, n, n1, level);

    if level == 0 {
        put_suffix0(sa, s, &mut bkt, n, k, bytemuck::cast::<u32, i32>(n1));
        induce_sal0(sa, s, &mut bkt, n, k, true);
        induce_sas0(sa, s, &mut bkt, n, k, true);
    } else {
        put_suffix1(
            bytemuck::cast_slice_mut::<u32, i32>(sa),
            bytemuck::cast_slice_mut::<u8, i32>(s),
            bytemuck::cast::<u32, i32>(n1),
        );
        induce_sal1(
            bytemuck::cast_slice_mut::<u32, i32>(sa),
            bytemuck::cast_slice_mut::<u8, i32>(s),
            bytemuck::cast::<u32, i32>(n),
            true,
        );
        induce_sas1(
            bytemuck::cast_slice_mut::<u32, i32>(sa),
            bytemuck::cast_slice_mut::<u8, i32>(s),
            bytemuck::cast::<u32, i32>(n),
            true,
        );
    }
}
