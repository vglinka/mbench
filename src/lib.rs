// Copyright (c) 2022 Vadim Glinka
//
// See the COPYRIGHT file at the top-level directory of this distribution
// and at https://github.com/vglinka/mbench/blob/main/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export] #[rustfmt::skip] macro_rules!
microbench {
    ($n:expr, $code:block) => {
        let now = std::time::Instant::now();
        
        for _ in 0..$n { $code };

        let elapsed = now.elapsed();
        println!("Elapsed / {} = {:.2?}", $n, elapsed / $n);
    };
    ($($code:tt)*) => { microbench!(10, { $($code)* } ); };
}

#[macro_export] #[rustfmt::skip] macro_rules!
fixedbench { ([$range:expr], $n:expr, $code:block) => {
        let now = std::time::Instant::now();
        
        for _ in 0..$n { $code };

        let elapsed = now.elapsed() / $n;
        println!("Elapsed / {} = {:.2?}", $n, elapsed);
        let elapsed = elapsed.as_millis();
        if !$range.contains(&elapsed) {
            panic!("expected [{:?}]ms, finished in {elapsed:?}ms.", $range);
        };
    };
    ([$range:expr], $($code:tt)*) => { fixedbench!([$range], 1, $($code)* ); };
} 

#[macro_export] #[rustfmt::skip] macro_rules!
microbench1 { ($($code:tt)*) => { microbench!(1, { $($code)* } ); }; }

#[macro_export] #[rustfmt::skip] macro_rules!
microbench5 { ($($code:tt)*) => { microbench!(5, { $($code)* } ); }; }

#[macro_export] #[rustfmt::skip] macro_rules!
microbench10 { ($($code:tt)*) => { microbench!(10, { $($code)* } ); }; }

#[macro_export] #[rustfmt::skip] macro_rules!
microbench50 { ($($code:tt)*) => { microbench!(50, { $($code)* } ); }; }

#[macro_export] #[rustfmt::skip] macro_rules!
microbench100 { ($($code:tt)*) => { microbench!(100, { $($code)* } ); }; }

#[macro_export] #[rustfmt::skip] macro_rules!
microbench500 { ($($code:tt)*) => { microbench!(500, { $($code)* } ); }; }

#[macro_export] #[rustfmt::skip] macro_rules!
microbench1000 { ($($code:tt)*) => { microbench!(1000, { $($code)* } ); }; }
