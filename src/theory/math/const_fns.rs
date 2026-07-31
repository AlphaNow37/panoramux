const fn eval_taylor<const N: usize>(derivatives: [f32; N], x: f32) -> f32 {
    let mut i = 0;
    let mut powx = 1.; // x^i
    let mut fact = 1; // i!
    let mut res = 0.; // sum
    while i < N {
        res += derivatives[i] * powx / fact as f32;
        i += 1;
        fact *= i;
        powx *= x;
    }
    res
}

pub const fn sin(x: f32) -> f32 {
    eval_taylor([0., 1., 0., -1., 0., 1., 0., -1., 0., 1., 0., -1.], x)
}
pub const fn cos(x: f32) -> f32 {
    eval_taylor([1., 0., -1., 0., 1., 0., -1., 0., 1., 0., -1., 0.], x)
}

// pub const fn factorial(n: usize) -> usize {
//     let mut k = 0;
//     let mut fact = 1;
//     while k < n {
//         k += 1;
//         fact *= k;
//     }
//     fact
// }
//
// pub const fn const_from_fn<const N: usize, T: Zero+Copy>(f: impl Fn(usize)->T)->[T; N] {
//     let mut arr = [T::ZERO; N];
//     let mut i = 0;
//     while i < N {
//         arr[i] = f(i);
//     }
//     arr
// }

const PRECOMPUTED_FACTORIAL: usize = 20;
const FACTORIALS: [usize; PRECOMPUTED_FACTORIAL] = {
    let mut facts = [1; PRECOMPUTED_FACTORIAL];
    let mut i = 2;
    while i < PRECOMPUTED_FACTORIAL {
        facts[i] = facts[i - 1] * i;
        i += 1;
    }
    facts
};
pub const fn factorial(n: usize) -> usize {
    FACTORIALS[n]
}

const PRECOMPUTED_BINOMIALS: usize = 10;
const BINOMIALS: [[usize; PRECOMPUTED_BINOMIALS]; PRECOMPUTED_BINOMIALS] = {
    let mut bins = [[0; PRECOMPUTED_BINOMIALS]; PRECOMPUTED_BINOMIALS];
    bins[0][0] = 1;
    let mut n = 1;
    while n < PRECOMPUTED_BINOMIALS {
        bins[n][0] = 1;
        let mut k = 1;
        while k < n + 1 {
            bins[n][k] = bins[n - 1][k - 1] + bins[n - 1][k];
            k += 1;
        }
        n += 1;
    }
    bins
};

pub const fn binomial(k: usize, n: usize) -> usize {
    BINOMIALS[n][k]
}

// pub const fn binomial_rev(k: usize, n: usize) -> usize {
//     if k > n {
//         0
//     } else {
//         BINOMIALS[n][n-k]
//     }
// }

#[test]
fn test() {
    // dbg!(cos(TAU));
    // dbg!(cos(TAU / 4.));
    // dbg!(sin(TAU));
    // dbg!(sin(TAU / 4.));

    dbg!(BINOMIALS, FACTORIALS);
}
