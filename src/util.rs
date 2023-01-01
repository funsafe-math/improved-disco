use eframe::epaint::Color32;

pub fn generate_points<F>(start: f64, end: f64, function: F, count_points: usize) -> Vec<[f64; 2]>
where
    F: Fn(f64) -> f64,
{
    let range = end - start;
    (0..count_points)
        .map(|i| {
            let x = start + i as f64 * range / (count_points - 1) as f64;
            [x, function(x)]
        })
        .collect()
}

pub fn to_numeric(s: &String) -> String {
    s.chars()
        .map(|c| match c {
            ',' => '.',
            _ => c,
        })
        .filter(|c| c.is_numeric() || c.eq(&'.') || c.eq(&'-'))
        .collect()
}

pub struct FunctionDetails {
    pub function: Vec<fn(f64, &[f64]) -> f64>,
    pub solution: fn(f64, (f64, f64)) -> f64,
    pub name: &'static str,
}

pub fn calculate_error_simple(a: &Vec<[f64; 2]>, b: &Vec<[f64; 2]>) -> f64 {
    assert!(a.len() == b.len());
    a.iter()
        .zip(b.iter())
        .map(|([_x1, y1], [_x2, y2])| (y1 - y2).powi(2))
        .sum()
}

pub fn calculate_error(a: &Vec<[f64; 2]>, b: &Vec<[f64; 2]>) -> f64 {
    let (longer, shorter) = match a.len() > b.len() {
        true => (a, b),
        false => (b, a),
    };
    if shorter.len() == 0 {
        return f64::INFINITY;
    }
    let m = longer.len() - 1;
    let n = shorter.len() - 1;
    let interpolate = |longer_index: usize| {
        let scale_factor = n as f64 / m as f64;
        let lower_index = longer_index * n / m;
        let rem = longer_index * n % m;

        let upper_index = lower_index + rem.clamp(0, 1);
        let factor = rem as f64 * scale_factor / n as f64;
        shorter[lower_index][1] * factor + shorter[upper_index][1] * (1.0 - factor)
    };
    let sum_squared: f64 = longer
        .iter()
        .enumerate()
        .map(|(i, v)| interpolate(i) - v[1])
        .map(|v| v * v)
        .sum();
    sum_squared.sqrt()
}

pub struct DefaultSettings {
    pub exact_plot_color: Color32,
    pub approximate_plot_color: Color32,
    pub interval_length: f64,
}

pub static DEFAULT_SETTINGS: DefaultSettings = DefaultSettings {
    exact_plot_color: Color32::GREEN,
    approximate_plot_color: Color32::RED,
    interval_length: 2.0,
};
