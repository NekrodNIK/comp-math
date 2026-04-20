use core::f64;
use image::{Rgb, RgbImage};
use num::Complex;

const ABS_TOL: f64 = 1e-15;

fn bisection_method<F>(f: F, mut a: f64, mut b: f64, max_iter: usize)
where
    F: Fn(f64) -> f64,
{
    if f(a) * f(b) > 0. {
        panic!("sufficient condition isn't satisfied");
    }

    let mut c = 0.;
    for i in 0..max_iter {
        c = (a + b) / 2.;

        if (b - a).abs() < ABS_TOL {
            println!("root: {}, iterations: {}", c, i);
            return;
        }

        if f(a) * f(c) < 0. {
            b = c;
        } else {
            a = c;
        }
    }
    println!("root: {}, iterations (max): {}", c, max_iter);
}

fn simple_iterations_method<Phi>(phi: Phi, mut x: f64, max_iter: usize)
where
    Phi: Fn(f64) -> f64,
{
    for i in 0..max_iter {
        let next_x = phi(x);
        if (next_x - x).abs() < ABS_TOL {
            println!("root: {}, iterations: {}", next_x, i);
            return;
        }
        x = next_x;
    }
    println!("root: {}, iterations (max): {}", x, max_iter);
}

fn newton_method<F, D>(f: F, der_f: D, x0: f64, max_iter: usize)
where
    F: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    let mut xn = x0;
    for i in 0..max_iter {
        let next_xn = xn - f(xn) / der_f(xn);
        if (next_xn - xn).abs() < ABS_TOL {
            println!("root: {}, iterations: {}", next_xn, i);
            return;
        }
        xn = next_xn;
    }
    println!("root: {}, iterations (max): {}", xn, max_iter);
}

fn complex_newton_method<F, D>(
    f: F,
    der_f: D,
    z0: Complex<f64>,
    max_iter: usize,
    print_: bool,
) -> Complex<f64>
where
    F: Fn(Complex<f64>) -> Complex<f64>,
    D: Fn(Complex<f64>) -> Complex<f64>,
{
    let mut zn = z0;
    for i in 0..max_iter {
        let next_zn = zn - f(zn) / der_f(zn);
        if (next_zn - zn).norm() < ABS_TOL {
            if print_ {
                println!("root: {}, iterations: {}", next_zn, i);
            }
            return next_zn;
        }
        zn = next_zn;
    }
    if print_ {
        println!("root: {}, iterations (max): {}", zn, max_iter);
    }
    return zn;
}

fn secant_method<F>(f: F, z0: f64, x1: f64, max_iter: usize)
where
    F: Fn(f64) -> f64,
{
    let mut prev_xn = z0;
    let mut xn = x1;
    for i in 0..max_iter {
        let λ = (xn - prev_xn) / (f(xn) - f(prev_xn));
        let next_xn = xn - λ * f(xn);
        if (next_xn - xn).abs() < ABS_TOL {
            println!("root: {}, iterations: {}", next_xn, i);
            return;
        }
        prev_xn = xn;
        xn = next_xn;
    }
    println!("root: {}, iterations (max): {}", xn, max_iter);
}

fn run_first(max_iter: usize) {
    let range = (7.7, 7.8);
    let f = |x: f64| x.tan() - x;
    let der_f = |x: f64| x.tan().powi(2);
    let phi = |x: f64| x.atan() + 2. * f64::consts::PI;

    println!("f(x) = tan(x) - x, [a, b] = [{}, {}]", range.0, range.1);
    println!("[BISECTION]");
    bisection_method(f, range.0, range.1, max_iter);
    println!("[SIMPLE ITERATIONS]");
    simple_iterations_method(phi, range.0, max_iter);
    println!("[NEWTON]");
    newton_method(f, der_f, range.0, max_iter);
    println!("[SECANT]");
    secant_method(f, range.0, range.1, max_iter);
}

fn run_second(z0: Complex<f64>, max_iter: usize) {
    let f = |z: Complex<f64>| z.powi(3) - 1.;
    let der_f = |z: Complex<f64>| 3. * z.powi(2);
    println!("f(z) = z^3 - 1, z0 = {}", z0);
    complex_newton_method(f, der_f, z0, max_iter, true);
}

fn bassin_draw() {
    let f = |z: Complex<f64>| z.powi(3) - 1.;
    let der_f = |z: Complex<f64>| 3. * z.powi(2);

    let xmin = -4.;
    let xmax = 4.;
    let ymin = -4.;
    let ymax = 4.;
    let width = 2048;
    let height = 2048;

    let mut img = RgbImage::new(width, height);

    let sqrt3 = 3.0_f64.sqrt();
    let roots = [
        Complex::new(1.0, 0.0),
        Complex::new(-0.5, sqrt3 / 2.0),
        Complex::new(-0.5, -sqrt3 / 2.0),
    ];

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let z = Complex {
            re: xmin + x as f64 * (xmax - xmin) / (width as f64),
            im: ymin + y as f64 * (ymax - ymin) / (height as f64),
        };
        let result = complex_newton_method(f, der_f, z, 1000, false);
        let mut root_ind = 0;
        for (i, root) in roots.iter().enumerate() {
            if (root - result).norm() < ABS_TOL {
                root_ind = i;
                break;
            }
        }

        let color = match root_ind {
            0 => Rgb([255, 0, 0]),
            1 => Rgb([0, 255, 0]),
            2 => Rgb([0, 0, 255]),
            _ => Rgb([0, 0, 0]),
        };
        *pixel = color;
    }

    img.save("task2.png").unwrap();
}

fn main() {
    run_first(100);
    println!("");
    run_second(Complex::new(1000., 0.5), 100);
    run_second(Complex::new(-1000., 10.), 100);
    run_second(Complex::new(-1000., -10.), 100);
    bassin_draw();
}
