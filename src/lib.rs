use std::ops::Range;

/// input slice of points must be sorted, radius must not be zero
/// if either of these preconditions is invalidated, this function may panic or return nonsensical results.
/// returns a list of bees (multidimensional points plottable on your graph)
pub fn beeswarm<const DIM: usize>(sorted_points: &[f64], radius: f64) -> Vec<[f64; DIM]> {
    let ranges = sorted_points
        .iter()
        .map(|&point| {
            intersection_range(sorted_points, radius, point)
                .expect("there should be at least one point within its radius-- itself")
        })
        .collect::<Vec<_>>();
    let mut swarm = vec![[0.0; DIM]; sorted_points.len()];
    for (i, &point) in sorted_points.iter().enumerate() {
        let mut next_bee = [0.0; DIM];
        next_bee[0] = point;
        let intersecting_bees =
            intersections(&swarm, radius, (i, next_bee), ranges[i].clone());

        swarm[i] = if intersecting_bees.is_empty() {
            next_bee
        } else {
            jiggle(
                &swarm,
                intersecting_bees,
                radius,
                ranges[i].clone(),
                next_bee,
            )
        }
    }
    swarm
}

fn jiggle<const DIM: usize>(
    placed_bees: &[[f64; DIM]],
    mut intersecting_bees: Vec<[f64; DIM]>,
    radius: f64,
    range: Range<usize>,
    new_bee: [f64; DIM],
) -> [f64; DIM] {
    let index = placed_bees.len();
    let mut candidate = None;
    let mut iteration = 0.0;
    let jigglestep = radius / 2.0;
    while candidate.is_none() {
        iteration += 1.0;
        candidate = (1..DIM)
            .flat_map(|i| {
                let mut candidate1 = new_bee;
                let mut candidate2 = new_bee;
                candidate1[i] += iteration * jigglestep;
                candidate2[i] -= iteration * jigglestep;
                [candidate1, candidate2]
            })
            .find(|candidate| {
                intersections(placed_bees, radius, (index, *candidate), range.clone()).is_empty()
            });
    }
    candidate.unwrap()
}

//fn get_positions_along_radius_higher_dimensional

fn intersections<const DIM: usize>(
    placed_bees: &[[f64; DIM]],
    radius: f64,
    new_bee: (usize, [f64; DIM]),
    range: Range<usize>,
) -> Vec<[f64; DIM]> {
    placed_bees
        .into_iter()
        .enumerate()
        .filter(|(i, placed_bee)| {
            !(!range.contains(i)
                || *i == new_bee.0
                || euclidean_distance(**placed_bee, new_bee.1) > radius)
        })
        .map(|(_, bee)| *bee)
        .collect()
}

fn euclidean_distance<const DIM: usize>(bee1: [f64; DIM], bee2: [f64; DIM]) -> f64 {
    (0..DIM)
        .map(|i| (bee1[i] - bee2[i]).powi(2))
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
