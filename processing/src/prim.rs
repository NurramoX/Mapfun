use crate::models::DistanceResult;
use ndarray::Array2;
use std::collections::{HashMap, HashSet};

pub fn prims_algorithm(edges: &Vec<DistanceResult>, ids: &Vec<i32>) -> Vec<(i32, i32)> {
    let n: usize = ids.len();
    let mut matrix = Array2::<f64>::zeros((n, n));
    let mut result = Vec::new();

    // Mapping id to matrix index
    let id_to_index: HashMap<i32, usize> = ids.iter().enumerate().map(|(index, &id)| (id, index)).collect();

    // Populate the matrix
    for edge in edges {
        let index1 = id_to_index[&edge.point1_id];
        let index2 = id_to_index[&edge.point2_id];
        matrix[[index1, index2]] = edge.distance;
        matrix[[index2, index1]] = edge.distance;
    }

    // Prim's algorithm
    let mut in_mst = HashSet::new();
    in_mst.insert(0); // Start with the first vertex

    while in_mst.len() < n {
        let mut min_edge = DistanceResult { point1_id: -1, point2_id: -1, distance: f64::MAX };

        for &vertex in &in_mst {
            for next_vertex in 0..n {
                if !in_mst.contains(&next_vertex) && matrix[[vertex, next_vertex]] > 0.0 {
                    let edge = DistanceResult {
                        point1_id: ids[vertex] as i32,
                        point2_id: ids[next_vertex] as i32,
                        distance: matrix[[vertex, next_vertex]]
                    };

                    if edge.distance < min_edge.distance {
                        min_edge = edge;
                    }
                }
            }
        }

        if min_edge.point1_id != -1 {
            in_mst.insert(id_to_index[&min_edge.point2_id]);
            result.push((min_edge.point1_id, min_edge.point2_id));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prims_algorithm() {
        // Create a small graph
        let edges = vec![
            DistanceResult { point1_id: 0, point2_id: 1, distance: 10.0 },
            DistanceResult { point1_id: 0, point2_id: 2, distance: 6.0 },
            DistanceResult { point1_id: 1, point2_id: 2, distance: 1.0 },
        ];
        let ids = vec![0, 1, 2];

        // Run Prim's algorithm
        let mst = prims_algorithm(&edges, &ids);

        // Expected MST result (based on the input graph)
        let expected = vec![
            (0, 2),
            (2, 1)
        ];

        // Assert that the MST matches the expected output
        assert_eq!(mst, expected);
    }

    #[test]
    fn test_prims_algorithm2() {
        // Create a small graph
        let edges = vec![
            DistanceResult { point1_id: 0, point2_id: 1, distance: 2.0 },
            DistanceResult { point1_id: 0, point2_id: 2, distance: 3.0 },
            DistanceResult { point1_id: 0, point2_id: 3, distance: 3.0 },
            DistanceResult { point1_id: 1, point2_id: 2, distance: 4.0 },
            DistanceResult { point1_id: 1, point2_id: 4, distance: 3.0 },
            DistanceResult { point1_id: 2, point2_id: 3, distance: 5.0 },
            DistanceResult { point1_id: 2, point2_id: 4, distance: 1.0 },
            DistanceResult { point1_id: 2, point2_id: 5, distance: 6.0 },
            DistanceResult { point1_id: 3, point2_id: 5, distance: 7.0 },
            DistanceResult { point1_id: 4, point2_id: 5, distance: 8.0 },
            DistanceResult { point1_id: 5, point2_id: 6, distance: 9.0 },
        ];
        let ids = vec![0, 1, 2, 3, 4, 5, 6];

        // Run Prim's algorithm
        let mst = prims_algorithm(&edges, &ids);

        // Expected MST result (based on the input graph)
        let expected = vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (2, 4),
            (2, 5),
            (5, 6),
        ];

        // Assert that the MST matches the expected output
        assert_eq!(mst, expected);
    }
}
