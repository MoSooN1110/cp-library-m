// source snippet: key=lib_golden_sep_search  prefix=lib_golden_sep_search

    let n: usize = read();
    let mut res = 0 as usize;

    let mut vec = vec![UINF; 10101];
    // dbg!(n);
    if n <= 5 {
        for i in 1..=n {
            res = max(res, ask(i, &mut vec, n));
        }
    } else {
        let mut cl: usize = 0;
        let mut cr: usize = 1597;
        let mut dl: usize = 610;
        let mut dr: usize = 987;
        let mut el = ask(dl, &mut vec, n);
        let mut er = ask(dr, &mut vec, n);
        let mut fib = vec![
            1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597,
        ];
        res = max(res, max(el, er));
        if el < er {
            cl = dl;
            dl = dr;
            dr = UINF;
            el = er;
            er = UINF;
        } else {
            cr = dr;
            dr = dl;
            dl = UINF;
            er = el;
            el = UINF;
        }

        for i in (0..13).rev() {
            if dl == UINF {
                dl = cl + fib[i];
                el = ask(dl, &mut vec, n);
            } else if dr == UINF {
                dr = cr - fib[i];
                er = ask(dr, &mut vec, n);
            }
            res = max(res, max(el, er));
            // assert_ne!(res, UINF);
            if el < er {
                cl = dl;
                dl = dr;
                dr = UINF;
                el = er;
                er = UINF;
            } else {
                cr = dr;
                dr = dl;
                dl = UINF;
                er = el;
                el = UINF;
            }
        }
        for i in cl + 1..=cr - 1 {
            res = max(res, ask(i, &mut vec, n));
        }
    }
