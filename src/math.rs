use factorial::Factorial;

use crate::util::FunctionDetails;

pub fn euler_method(
    starting_point: (f64, f64),
    end: f64,
    function_details: FunctionDetails,
    n_points: usize,
) -> Vec<[f64; 2]> {
    let (mut x, mut y) = starting_point;
    let start = x;
    let length = end - x;
    let h = length / (n_points - 1) as f64;
    let function = function_details.function;
    let next_y = |x, y| y + function[0](x, &[y]) * h;
    let mut result = vec![[x, y]];
    for i in 1..n_points {
        y = next_y(x, y);
        x += start + i as f64 * (length) / (n_points - 1) as f64;
        result.push([x, y]);
    }
    result
}

// pub fn taylor_method(
//     starting_point: (f64, f64),
//     end: f64,
//     function_details: FunctionDetails,
//     n_points: usize,
// ) -> Vec<[f64; 2]> {
//     let (mut x, mut y) = starting_point;
//     let start = x;
//     let length = (end - x).abs();
//     let h = length / (n_points - 1) as f64;
//     let function = function_details.function;
//     let first_derivative = function_details.first_derivative;
//     let next_y = |x, y, y2| y + h * (function(x, y) + 0.5 * h * y2);
//     let mut result = vec![[x, y]];
//     for i in 1..n_points {
//         let y1 = function(x, y);
//         let y2 = first_derivative(x, y, y1);
//         y = next_y(x, y, y2);
//         x = start + i as f64 * (length) / (n_points - 1) as f64;
//         result.push([x, y]);
//     }
//     result
// }

pub fn taylor_method(
    starting_point: (f64, f64),
    end: f64,
    function_details: &FunctionDetails,
    n_points: usize,
) -> Vec<[f64; 2]> {
    let function = &function_details.function;
    let (x0, y0) = starting_point;
    let mut x = x0;
    let length = (end - x).abs();
    let h = length / (n_points - 1) as f64;
    let n_derivatives = function.len() + 1;
    let mut ys: Vec<f64> = vec![y0].repeat(n_derivatives);
    let mut result = vec![[x, y0]];
    for i in 1..n_points {
        for d in 1..n_derivatives {
            ys[d] = function[d - 1](x, &ys);
        }
        let y: f64 = ys
            .iter()
            .enumerate()
            .map(|(k, v)| h.powf(k as f64) * v / k.factorial() as f64)
            .sum();
        x = x0 + i as f64 * (length) / (n_points - 1) as f64;
        result.push([x, y]);
        ys[0] = y;
    }
    return result;
}
