use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct UnionFind<'a> {
    node_to_parent: HashMap<&'a (i64, i64, i64), &'a (i64, i64, i64)>,
    node_to_size: HashMap<&'a (i64, i64, i64), i64>,
}

impl<'a> UnionFind<'a> {
    fn new() -> Self {
        UnionFind {
            node_to_parent: HashMap::new(),
            node_to_size: HashMap::new(),
        }
    }

    // Find the root of the set containing the node
    fn find(&mut self, node: &'a (i64, i64, i64)) -> &'a (i64, i64, i64) {
        // Initialize if not seen before
        if !self.node_to_parent.contains_key(&node) {
            self.node_to_parent.insert(node, node);
            self.node_to_size.insert(node, 1);
            return node;
        }

        // Path compression: make each node point directly to the root
        if self.node_to_parent[&node] != node {
            let root = self.find(self.node_to_parent[&node]);
            self.node_to_parent.insert(node, root);
        }

        // Will always return root due to path compression above
        self.node_to_parent[node]
    }

    // Union by size: attach the smaller tree to the larger one
    fn union(&mut self, a: &'a (i64, i64, i64), b: &'a (i64, i64, i64)) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return false; // Already in the same set
        }

        let size_a = self.node_to_size[root_a];
        let size_b = self.node_to_size[root_b];

        if size_a < size_b {
            self.node_to_parent.insert(root_a, root_b);
            self.node_to_size.insert(root_b, size_a + size_b);
        } else {
            self.node_to_parent.insert(root_b, root_a);
            self.node_to_size.insert(root_a, size_a + size_b);
        }

        true // Successfully merged
    }

    fn get_size(&mut self, node: &'a (i64, i64, i64)) -> i64 {
        let root = self.find(node);
        self.node_to_size[root]
    }

    fn get_component_sizes(&mut self) -> Vec<i64> {
        let mut roots: HashSet<&(i64, i64, i64)> = HashSet::new();
        let nodes: Vec<_> = self.node_to_parent.keys().copied().collect();
        for &node in &nodes {
            let root = self.find(node);
            roots.insert(root);
        }
        roots.iter().map(|node| self.get_size(node)).collect()
    }
}


fn connect_junction_boxes(input: &str, limit: usize) -> i64 {

    let junction_boxes = input
        .lines()
        .map(|line| {
            let parts = line
                .split(',')
                .map(|p| p.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (parts[0], parts[1], parts[2])
        })
        .collect::<Vec<(i64, i64, i64)>>();

    let unique_pairs_by_distance: Vec<_> = junction_boxes
        .iter()
        .combinations(2)
        .sorted_by_key(|pair| {
            let a = pair[0];
            let b = pair[1];
            let x = a.0 - b.0;
            let y = a.1 - b.1;
            let z = a.2 - b.2;
            x * x + y * y + z * z
        })
        .map(|pair| (pair[0], pair[1]))
        .take(limit) //assumes that if we try to connect boxes already in a circuit, it counts as a 'connection'
        .collect();

    let mut union_find = UnionFind::new();
    for (a, b) in unique_pairs_by_distance {
        union_find.union(a, b);
    }

    union_find
        .get_component_sizes()
        .iter()
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day08;

    #[test]
    fn can_connect_junction_boxes_for_sample_input() {
        assert_eq!(day08::connect_junction_boxes(SAMPLE_INPUT, 10), 40)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day08.txt").unwrap();
        assert_eq!(day08::connect_junction_boxes(input.as_str(), 1000), 47040)
    }

    const SAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
}