use core::f64;

use plotly::{Plot, Scatter};

fn div_diff(p: &[(f64, f64)]) -> f64 {
    if p.len() == 1 {
        p[0].1
    } else {
        (div_diff(&p[1..]) - div_diff(&p[..p.len() - 1])) / (p[p.len() - 1].0 - p[0].0)
    }
}

fn newton(points: &[(f64, f64)]) -> Option<impl Fn(f64) -> f64> {
    if points.len() < 2 {
        return None;
    }

    let y0 = points[0].1;
    let v: Vec<_> = (1..points.len())
        .map(|i| (points[i - 1].0, div_diff(&points[..=i])))
        .collect();

    Some(move |x| {
        let mut res = y0;
        let mut c = 1.;
        for &(px, d) in v.iter() {
            c *= x - px;
            res += d * c;
        }
        return res;
    })
}

fn xi(i: usize, n: usize) -> f64 {
    (2. * i as f64) / n as f64 - 1.
}

fn f(x: f64) -> f64 {
    1. / (1. + 25. * x * x)
}

fn cheb_nodes(n: usize) -> impl Iterator<Item = f64> {
    (0..=n).map(move |k| ((2. * k as f64 + 1.) * f64::consts::PI / (2. * (n + 1) as f64)).cos())
}

fn draw_plot<P1, P2>(p1: P1, p2: P2, name: &str)
where
    P1: Fn(f64) -> f64,
    P2: Fn(f64) -> f64,
{
    let mut plot = Plot::new();
    let xs: Vec<f64> = (-500..=500).map(|i| (i as f64) / 500.).collect();
    let ys: Vec<f64> = xs.iter().map(|&x| (p1(x) - f(x)).abs()).collect();
    let ys_cheb: Vec<f64> = xs.iter().map(|&x| (p2(x) - f(x)).abs()).collect();
    plot.add_trace(Scatter::new(xs.clone(), ys).name("|P_n(x) - f(x)|"));
    plot.add_trace(Scatter::new(xs.clone(), ys_cheb).name("cheb"));
    plot.write_html(format!("task3-{}.html", name));
}

fn draw_plot2<P1, P2>(p1: P1, p2: P2)
where
    P1: Fn(usize) -> f64,
    P2: Fn(usize) -> f64,
{
    let mut plot = Plot::new();
    let xs: Vec<_> = (3..=10).collect();
    let ys: Vec<_> = xs.iter().map(|&x| p1(x)).collect();
    let ys_cheb: Vec<_> = xs.iter().map(|&x| p2(x)).collect();
    plot.add_trace(Scatter::new(xs.clone(), ys).name("|P_n(x) - f(x)|"));
    plot.add_trace(Scatter::new(xs.clone(), ys_cheb).name("cheb"));
    plot.write_image(
        format!("task3.png"),
        plotly::ImageFormat::PNG,
        1600,
        1200,
        1.,
    )
    .unwrap();
}

fn main() {
    let mut max_error1 = Vec::new();
    let mut max_error2 = Vec::new();

    for n in 3..=10 {
        let points: Vec<_> = (0..=n).map(|i| (xi(i, n), f(xi(i, n)))).collect();
        let cheb: Vec<_> = cheb_nodes(n).map(|x| (x, f(x))).collect();
        let p1 = newton(&points).unwrap();
        let p2 = newton(&cheb).unwrap();
        let max1 = (-500..=500)
            .map(|i| (i as f64) / 500.)
            .map(|x| (p1(x) - f(x)).abs())
            .fold(f64::NEG_INFINITY, |a, b| a.max(b));
        let max2 = (-500..=500)
            .map(|i| (i as f64) / 500.)
            .map(|x| (p2(x) - f(x)).abs())
            .fold(f64::NEG_INFINITY, |a, b| a.max(b));
        draw_plot(p1, p2, &format!("n={}", n));
        max_error1.push(max1);
        max_error2.push(max2);
    }

    draw_plot2(|n| max_error1[n - 3], |n| max_error2[n - 3]);
}
