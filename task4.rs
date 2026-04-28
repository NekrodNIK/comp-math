// FIXME reinspect code;
use std::f64::consts::{E, PI};

fn simpsons(f: &impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    (b - a) / 6. * (f(a) + 4. * f((a + b) / 2.) + f(b))
}

fn composite_simpsons(f: &impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / n as f64;
    let x = |i| a + i as f64 * h;
    let sum: f64 = (1..=n)
        .step_by(2)
        .map(|i| f(x(i - 1)) + 4. * f(x(i)) + f(x(i + 1)))
        .sum();
    h / 3. * sum
}

fn legendre(n: usize, x: f64) -> f64 {
    match n {
        0 => 1.,
        1 => x,
        _ => {
            let mut prev = legendre(0, x);
            let mut cur = legendre(1, x);

            for k in 2..=n {
                let kf = k as f64;
                let new = (2. * kf - 1.) / kf * x * cur - (kf - 1.) / kf * prev;
                prev = std::mem::replace(&mut cur, new);
            }

            cur
        }
    }
}

fn legendre_derivative(n: usize, x: f64) -> f64 {
    n as f64 / (1. - x * x) * (legendre(n - 1, x) - x * legendre(n, x))
}

fn legendre_roots(n: usize, max_iter: usize, abs_tol: f64) -> Vec<f64> {
    (0..n)
        .map(|i| PI * (4. * i as f64 - 1.) / (4. * n as f64 + 2.))
        .map(|i| i.cos())
        .map(|mut root| {
            for _ in 0..max_iter {
                let p = legendre(n, root);
                let dp = legendre_derivative(n, root);
                let delta = p / dp;
                root -= delta;

                if delta.abs() < abs_tol {
                    break;
                }
            }
            root
        })
        .collect()
}

fn legendre_w(root: f64, n: usize) -> f64 {
    let derivative = legendre_derivative(n, root);
    2. / ((1. - root * root) * derivative * derivative)
}

fn gauss_legendre(f: &impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let h = (b - a) / 2.;
    let m = (a + b) / 2.;
    let roots = legendre_roots(n, 1000, 1e-10);
    h * ((1..=n)
        .map(|i| legendre_w(roots[i - 1], n) * f(h * roots[i - 1] + m))
        .sum::<f64>())
}

fn run(f: &impl Fn(f64) -> f64, range: (f64, f64), n: usize) {
    println!("simpson's = {}", simpsons(f, range.0, range.1));
    println!(
        "composite simpson's (n={}) = {}",
        n,
        composite_simpsons(f, range.0, range.1, n)
    );

    let val = gauss_legendre(&f, range.0, range.1, n);
    println!("gauss-legandre (n={}) = {}", n, val);
}

fn calc_approx(f: impl Fn(f64) -> f64, a: f64, b: f64, exact: f64) {
    let mut prev_s = exact - composite_simpsons(&f, a, b, 2);
    for n in (4..=100).step_by(2) {
        let s = exact - composite_simpsons(&f, a, b, n);
        println!(
            "composite simpson's order of approximation (n={}) = {}",
            n,
            (prev_s / s).log2()
        );
        prev_s = s;
    }
    let mut prev_gl = exact - gauss_legendre(&f, a, b, 2);
    for n in 3..=100 {
        let gl = exact - gauss_legendre(&f, a, b, n);
        println!(
            "gauss-legandre order of approximation (n={}) = {}",
            n,
            (prev_gl / gl).log2()
        );
        prev_gl = gl;
    }
}

fn main() {
    let eps = 1e-8;
    // exact 8.0349107
    let f1 = |x: f64| (PI * x.powi(5)).sin() / (x.powi(5) * (1. - x));
    let range1 = (eps, 1. - eps);
    // exact 2.9810030
    let f2 = |t: f64| {
        let j = 1. / ((1. - t) * (1. - t));
        let x = t / (1. - t);
        E.powf(-x.sqrt() + (x / 10.).sin()) * j
    };
    let range2 = (eps, 1. - eps);

    println!("[FIRST]");
    run(&f1, range1, 6000);
    println!("[SECOND]");
    run(&f2, range2, 6000);
    calc_approx(|x| E.powf(x), 0., 1., E - 1.);
}
