use plotly::{
    Layout, Plot, Scatter,
    common::{Label, Mode, Title},
    layout::{Axis, AxisRange},
};

type V = Vec<f64>;
fn first(_t: f64, y: &V) -> V {
    vec![998. * y[0] + 1998. * y[1], -999. * y[0] - 1999. * y[1]]
}
fn second_(_t: f64, y: &V, a: f64, b: f64, c: f64, d: f64) -> V {
    vec![a * y[0] - b * y[0] * y[1], c * y[0] * y[1] - d * y[1]]
}
fn second(t: f64, y: &V) -> V {
    second_(t, y, 10., 2., 2., 10.)
}

fn euler(f: &impl Fn(f64, &V) -> V, n: usize, step: f64, t0: f64, y0: &V) -> (f64, V) {
    let mut t = t0;
    let mut y = y0.clone();
    for _ in 0..n {
        (0..y.len()).for_each(|i| y[i] += step * f(t, &y)[i]);
        t += step;
    }
    (t, y)
}

fn simple_iterations_method(phi: impl Fn(&V) -> V, x0: &V, max_iter: usize) -> V {
    let mut x = x0.clone();
    for i in 0..max_iter {
        let next_x = phi(&x);
        if (0..x.len())
            .map(|i| (next_x[i] - x[i]).abs())
            .fold(0_f64, |a, b| a.max(b))
            < 1e-15
        {
            break;
        }
        x = next_x;
    }
    x
}
fn implicit_euler(f: &impl Fn(f64, &V) -> V, n: usize, step: f64, t0: f64, y0: &V) -> (f64, V) {
    let mut t = t0;
    let mut y = y0.clone();
    for _ in 0..n {
        let prev_y = y.clone();
        y = simple_iterations_method(
            |v: &Vec<f64>| {
                f(t, &v)
                    .into_iter()
                    .enumerate()
                    .map(|(i, x)| prev_y[i] + step * x)
                    .collect()
            },
            &prev_y,
            10000,
        );
        t += step;
    }
    (t, y)
}

fn rk4(f: &impl Fn(f64, &V) -> V, n: usize, step: f64, t0: f64, y0: &V) -> (f64, V) {
    let mut t = t0;
    let mut y = y0.clone();
    let len = y.len();

    for _ in 0..n {
        let k1 = f(t, &y);

        let mut y2 = vec![0.0; len];
        for i in 0..len {
            y2[i] = y[i] + step / 2.0 * k1[i];
        }
        let k2 = f(t + step / 2.0, &y2);

        let mut y3 = vec![0.0; len];
        for i in 0..len {
            y3[i] = y[i] + step / 2.0 * k2[i];
        }
        let k3 = f(t + step / 2.0, &y3);

        let mut y4 = vec![0.0; len];
        for i in 0..len {
            y4[i] = y[i] + step * k3[i];
        }
        let k4 = f(t + step, &y4);

        for i in 0..len {
            y[i] += step / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
        t += step;
    }
    (t, y)
}

fn rk4_with_trajectory(
    f: &impl Fn(f64, &V) -> V,
    n: usize,
    step: f64,
    t0: f64,
    y0: &V,
) -> (V, Vec<V>) {
    let len = y0.len();
    let mut ts = vec![t0];
    let mut ys = vec![y0.clone()];

    for _ in 0..n {
        let mut t = *ts.last().unwrap();
        let mut y = ys.last().unwrap().clone();

        let k1 = f(t, &y);

        let mut y2 = vec![0.0; len];
        for i in 0..len {
            y2[i] = y[i] + step / 2.0 * k1[i];
        }
        let k2 = f(t + step / 2.0, &y2);

        let mut y3 = vec![0.0; len];
        for i in 0..len {
            y3[i] = y[i] + step / 2.0 * k2[i];
        }
        let k3 = f(t + step / 2.0, &y3);

        let mut y4 = vec![0.0; len];
        for i in 0..len {
            y4[i] = y[i] + step * k3[i];
        }
        let k4 = f(t + step, &y4);

        for i in 0..len {
            y[i] += step / 6.0 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
        t += step;
        ts.push(t);
        ys.push(y);
    }
    (ts, ys)
}

fn calc_error(comp: &V, exact: &V) -> f64 {
    (0..exact.len())
        .map(|i| (comp[i] - exact[i]).abs())
        .fold(0_f64, |a, b| a.max(b))
}

fn calc_order_of_approximation() {
    // let steps = [
    //     0.0005,
    //     0.00025,
    //     0.000125,
    //     0.0000625,
    //     0.00003125,
    //     0.000015625,
    //     0.0000078125,
    //     0.00000390625,
    // ];
    // let y0 = vec![1., 2.];
    // let t_end = 1.;

    // let system = &first;
    // let analytical = |t: f64| {
    //     let c1 = (1000.0 * y0[0] + 1998.0 * y0[1]) / 999.0;
    //     let c2 = (y0[0] + 2.0 * y0[1]) / 999.0;
    //     vec![
    //         c1 * (-t).exp() + 2.0 * c2 * (-1000.0 * t).exp(),
    //         c1 * (-t).exp() - c2 * (-1000.0 * t).exp(),
    //     ]
    // };

    let steps = [0.1, 0.05, 0.025, 0.0125, 0.00625, 0.003125];
    let start_step = 0.2;
    let y0 = vec![1., 1.];
    let t_start = 0.;
    let t_end = 1.;
    let system = |_: f64, y: &Vec<f64>| vec![-1. * y[0], -2. * y[1]];
    let analytical = |t: f64| vec![y0[0] * (-t).exp(), y0[1] * (-2.0 * t).exp()];

    println!("\nEULER, start_step={}", start_step);
    let (t, y) = euler(
        &system,
        (t_end / start_step) as usize,
        start_step,
        t_start,
        &y0,
    );
    let mut prev_err = calc_error(&y, &analytical(t));
    for step in steps {
        let (t, y) = euler(&system, (t_end / step) as usize, step, t_start, &y0);
        let err = calc_error(&y, &analytical(t));
        let order = (prev_err / err).log2();
        println!("order (step={}): {}", step, order);
        prev_err = err;
    }

    println!("\nIMPLICIT EULER, start_step={}", start_step);
    let (t, y) = implicit_euler(
        &system,
        (t_end / start_step) as usize,
        start_step,
        t_start,
        &y0,
    );
    prev_err = calc_error(&y, &analytical(t));
    for step in steps {
        let (t, y) = implicit_euler(&system, (t_end / step) as usize, step, t_start, &y0);
        let err = calc_error(&y, &analytical(t));
        let order = (prev_err / err).log2();
        println!("order (step={}): {}", step, order);
        prev_err = err;
    }

    println!("\nRK4, start_step={}", start_step);
    let (t, y) = rk4(
        &system,
        (t_end / start_step) as usize,
        start_step,
        t_start,
        &y0,
    );
    prev_err = calc_error(&y, &analytical(t));
    for step in steps {
        let (t, y) = rk4(&system, (t_end / step) as usize, step, t_start, &y0);
        let err = calc_error(&y, &analytical(t));
        let order = (prev_err / err).log2();
        println!("order (step={}): {}", step, order);
        prev_err = err;
    }
}

fn solution_of_first() {
    let system = &first;
    let step = 0.00005;

    println!("\n\nFIRST");
    let ys = [-1., 1., 2.];
    let ts = [-5., 0., 2., 5.];

    for y0_0 in ys {
        for y0_1 in ys {
            let y0 = vec![y0_0, y0_1];

            let analytical = |t: f64| {
                let c1 = (1000.0 * y0[0] + 1998.0 * y0[1]) / 999.0;
                let c2 = (y0[0] + 2.0 * y0[1]) / 999.0;
                vec![
                    c1 * (-t).exp() + 2.0 * c2 * (-1000.0 * t).exp(),
                    c1 * (-t).exp() - c2 * (-1000.0 * t).exp(),
                ]
            };
            for t_start in ts {
                for t_end in ts {
                    if t_start >= t_end {
                        continue;
                    }
                    let n = ((t_end - t_start) / step) as usize;

                    let (_, y_euler) = euler(system, n, step, t_start, &y0);
                    let (_, y_implicit) = implicit_euler(system, n, step, t_start, &y0);

                    println!(
                        "y0=[{},{}], t in [{}, {}]: \neuler={:?}, \nimplicit={:?}, \nanalytical={:?}\n",
                        y0_0,
                        y0_1,
                        t_start,
                        t_end,
                        y_euler,
                        y_implicit,
                        analytical(t_end)
                    );
                }
            }
        }
    }
}

fn solution_of_second() {
    let system = &second;
    let step = 0.00005;
    let n = (1. / step) as usize;

    let y0 = vec![1., 2.];
    let (_, y) = rk4(system, n, step, 0., &y0);
    println!("SECOND: {:?}", y);
}

fn plot() {
    let step = 0.01;
    let t_max = 30.;
    let n = (t_max / step) as usize;

    let colors = vec!["red", "blue", "green", "purple", "orange", "brown"];
    let mut plot = Plot::new();

    for (i, (x0, y0)) in (0..=20).map(move |a| (a as f64, a as f64)).enumerate() {
        let (_, ys) = rk4_with_trajectory(&second, n, step, 0.0, &vec![x0, y0]);
        let x: Vec<f64> = ys.iter().map(|v| v[0]).collect();
        let y: Vec<f64> = ys.iter().map(|v| v[1]).collect();

        let fst = &ys[0];
        let fixed = ys
            .iter()
            .all(|v| (v[0] - fst[0]).abs() < 1e-10 && (v[1] - fst[1]).abs() < 1e-10);
        let trace = if !fixed {
            plotly::traces::Scatter::new(x, y)
                .name(format!("start = ({}, {})", x0, y0))
                .mode(Mode::Lines)
                .line(
                    plotly::common::Line::new()
                        .color(colors[i % colors.len()])
                        .width(2.),
                )
        } else {
            plotly::traces::Scatter::new(x, y)
                .name(format!("start = ({}, {})", x0, y0))
                .mode(Mode::Markers)
                .marker(plotly::common::Marker::new().color(colors[i % colors.len()]))
        };
        plot.add_trace(trace);
    }

    let layout = Layout::new()
        .x_axis(Axis::new())
        .y_axis(Axis::new())
        .width(800)
        .height(800);

    plot.set_layout(layout);
    plot.write_html(format!("task5.html"));
}

fn main() {
    calc_order_of_approximation();
    solution_of_first();
    solution_of_second();
    plot();
}
