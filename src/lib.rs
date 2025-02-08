use std::ops::Range;

use itertools::Itertools;

//O(DIM * n^2)
/// input slice of points must be sorted, radius must not be zero
/// if either of these preconditions is invalidated, this function may panic or return nonsensical results.
/// returns a list of bees (multidimensional points plottable on your graph)
pub fn beeswarm_greedy<const DIM: usize>(sorted_points: &[f64], radius: f64) -> Vec<[f64; DIM]> {
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
        let intersecting_bees = intersections(&swarm, radius, (i, next_bee), ranges[i].clone());

        swarm[i] = if intersecting_bees.is_empty() {
            next_bee
        } else {
            jiggle_greedy(
                &swarm,
                // intersecting_bees,
                radius,
                ranges[i].clone(),
                next_bee,
            )
        }
    }
    swarm
}

fn jiggle_greedy<const DIM: usize>(
    placed_bees: &[[f64; DIM]],
    // mut intersecting_bees: Vec<[f64; DIM]>, //why is this unused?
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

//O(DIM * steps * n^2)
// pub fn beeswarm_exhaustive<const DIM: usize>(
//     sorted_points: &[f64],
//     radius: f64,
//     steps: usize,
// ) -> Vec<[f64; DIM]> {
//     let greedy_solution = beeswarm_greedy(sorted_points, radius);
//     let max = max_distance_from_centre(&greedy_solution);
//     let possibilities = (0..steps)
//         .map(|step| step as f64 / steps as f64 - 0.5)
//         .map(|step| step * max * 2.0);
// }

//O(DIM * n^2)
fn valid_swarm<const DIM: usize>(bees: &[[f64; DIM]], radius: f64) -> bool {
    bees.iter()
        .cartesian_product(bees)
        .map(|(bee1, bee2)| euclidean_distance(bee1, bee2))
        .any(|distance| distance < radius)
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
                || euclidean_distance(placed_bee, &new_bee.1) > radius)
        })
        .map(|(_, bee)| *bee)
        .collect()
}

//idea: dynamic programming solution that finds a true optimal solution instead of the greedy algorithm

pub fn distances_from_centre<const DIM: usize>(bees: &[[f64; DIM]]) -> (f64, f64) {
    (
        max_distance_from_centre(bees),
        rms_distance_from_centre(bees),
    )
}

fn max_distance_from_centre<const DIM: usize>(bees: &[[f64; DIM]]) -> f64 {
    bees.iter()
        .map(euclidean_distance_from_centre)
        .max_by(|d1, d2| d1.partial_cmp(d2).unwrap())
        .unwrap_or(f64::NAN)
}

fn rms_distance_from_centre<const DIM: usize>(bees: &[[f64; DIM]]) -> f64 {
    let sum_of_squares = bees
        .iter()
        .map(euclidean_distance_from_centre) //distances
        .map(|distance| distance.powi(2))
        .sum::<f64>();
    let mean_square = sum_of_squares / bees.len() as f64;
    mean_square.sqrt()
}

fn euclidean_distance_from_centre<const DIM: usize>(bee: &[f64; DIM]) -> f64 {
    let mut unjiggled_bee = [0.0; DIM];
    unjiggled_bee[0] = bee[0];
    euclidean_distance(bee, &unjiggled_bee)
}

fn euclidean_distance<const DIM: usize>(bee1: &[f64; DIM], bee2: &[f64; DIM]) -> f64 {
    (0..DIM)
        .map(|i| (bee1[i] - bee2[i]).powi(2))
        .sum::<f64>()
        .sqrt()
}

///range of indices into the slice whose circles of a given radius would overlap the provided point
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

#[cfg(test)]
mod tests {
    use core::f64;

    use rand::prelude::*;
    use rand_distr::Uniform;

    use super::*;
    const POINT_RADIUS_BEES: f64 = 1.0 / 20.0;

    fn generate_random_sorted() -> Vec<f64> {
        let rng = rand::rng();
        let normal = rand_distr::Normal::new(0.0, 0.4).unwrap();
        let mut points = normal
            .sample_iter(rng.clone())
            .take(100)
            .collect::<Vec<f64>>();

        points.sort_by(|a, b| a.partial_cmp(b).unwrap());
        points
    }

    fn get_biggest_difference(n: usize) -> (Vec<f64>, Vec<f64>, f64, f64) {
        let mut diff_max_distance = f64::MIN;
        let mut diff_rms_distance = f64::MIN;
        let mut max_outlier = vec![];
        let mut rms_outlier = vec![];
        for _ in 0..n {
            let points = generate_random_sorted();
            let mirrored_points = mirror_points(&points);

            let bees: Vec<[f64; 2]> = beeswarm_greedy(points.as_slice(), POINT_RADIUS_BEES);
            let seeb: Vec<[f64; 2]> = beeswarm_greedy(&mirrored_points, POINT_RADIUS_BEES);

            let bees = distances_from_centre(&bees);
            let seeb = distances_from_centre(&seeb);
            let max_distance = (bees.0 - seeb.0).abs();
            let rms_distance = (bees.1 - seeb.1).abs();

            if max_distance > diff_max_distance {
                diff_max_distance = max_distance;
                max_outlier = points.clone();
            }
            if rms_distance > diff_rms_distance {
                diff_rms_distance = rms_distance;
                rms_outlier = points;
            }
        }

        (
            max_outlier,
            rms_outlier,
            diff_max_distance,
            diff_rms_distance,
        )
    }

    fn mirror_points(points: &[f64]) -> Vec<f64> {
        points.iter().rev().map(|p| -p).collect()
    }

    #[test]
    fn meow() {
        assert!(false);
    }
}
