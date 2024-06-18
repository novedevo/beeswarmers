use std::ops::Range;

/// input slice of points must be sorted, radius must not be zero
/// if either of these preconditions is invalidated, this function may panic or return nonsensical results.
pub fn beeswarm<const DIM: usize>(sorted_points: &[f64], radius: f64) -> Vec<[f64; DIM]> {
    let ranges = sorted_points
        .iter()
        .map(|&point| {
            intersection_range(sorted_points, radius, point)
                .expect("there should be at least one point within its radius-- itself")
        })
        .collect::<Vec<_>>();
    let mut retval = vec![[0.0; DIM]; sorted_points.len()];
    for (i, &point) in sorted_points.iter().enumerate() {
        let mut next_point = [0.0; DIM];
        next_point[0] = point;
        let intersecting_points =
            intersections(&retval, radius, (i, next_point), ranges[i].clone());

        retval[i] = if intersecting_points.is_empty() {
            next_point
        } else {
            jiggle(
                &retval,
                intersecting_points,
                radius,
                ranges[i].clone(),
                next_point,
            )
        }
    }
    retval
}

fn jiggle<const DIM: usize>(
    placed_points: &[[f64; DIM]],
    mut intersecting_points: Vec<[f64; DIM]>,
    radius: f64,
    range: Range<usize>,
    new_point: [f64; DIM],
) -> [f64; DIM] {
    let index = placed_points.len();
    let mut candidate = None;
    let mut iteration = 0.0;
    let jigglestep = radius / 2.0;
    while candidate.is_none() {
        iteration += 1.0;
        candidate = (1..DIM)
            .flat_map(|i| {
                let mut candidate1 = new_point;
                let mut candidate2 = new_point;
                candidate1[i] += iteration * jigglestep;
                candidate2[i] -= iteration * jigglestep;
                [candidate1, candidate2]
            })
            .find(|candidate| {
                intersections(placed_points, radius, (index, *candidate), range.clone()).is_empty()
            });
    }
    candidate.unwrap()
}

//fn get_positions_along_radius_higher_dimensional

fn intersections<const DIM: usize>(
    placed_points: &[[f64; DIM]],
    radius: f64,
    new_point: (usize, [f64; DIM]),
    range: Range<usize>,
) -> Vec<[f64; DIM]> {
    placed_points
        .into_iter()
        .enumerate()
        .filter(|(i, placed_point)| {
            !(!range.contains(i)
                || *i == new_point.0
                || euclidean_distance(**placed_point, new_point.1) > radius)
        })
        .map(|(_, point)| *point)
        .collect()
}

fn euclidean_distance<const DIM: usize>(point1: [f64; DIM], point2: [f64; DIM]) -> f64 {
    (0..DIM)
        .map(|i| (point1[i] - point2[i]).powi(2))
        .sum::<f64>()
        .sqrt()
}

fn intersection_range(
    sorted_points: &[f64],
    radius: f64,
    current_point: f64,
) -> Option<Range<usize>> {
    let indices = sorted_points
        .iter()
        .enumerate()
        .filter(|(_, &point)| (point - current_point).abs() < radius * 2.0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    Some(*indices.first()?..*indices.last()?)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
