use itertools::{self, Itertools};
use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    find_cliques(3, parsed.1, parsed.0)
}

pub fn part2(input: &[String]) -> String {
    let mut parsed = parse_2(input);
    let cliques = find_max_clique(
        &mut HashSet::new(),
        &mut parsed.0,
        &mut HashSet::new(),
        &parsed.1,
    );
    let mut sorted = cliques
        .iter()
        .map(|clique| (clique, clique.len()))
        .sorted_by(|a, b| b.1.cmp(&a.1));
    convert_clique_to_password(sorted.next().unwrap().0, parsed.2)
}

fn convert_clique_to_password(clique: &HashSet<usize>, node_id_to_name: Vec<&str>) -> String {
    let mut nodes: Vec<String> = clique
        .iter()
        .map(|id| node_id_to_name[*id].to_string())
        .collect();
    nodes.sort();
    nodes.iter().join(",")
}

pub fn find_max_clique(
    r: &HashSet<usize>,
    p: &mut HashSet<usize>,
    x: &mut HashSet<usize>,
    connections: &HashMap<usize, HashSet<usize>>,
) -> Vec<HashSet<usize>> {
    if p.is_empty() && x.is_empty() {
        return vec![r.clone()];
    }
    let mut cliques = vec![];
    for v in p.clone() {
        let mut r = r.clone();
        r.insert(v.clone());

        let neighbours = connections.get(&v).unwrap();

        let mut new_p = p.intersection(neighbours).cloned().collect();
        let mut new_x = x.intersection(neighbours).cloned().collect();
        let mut new_cliques = find_max_clique(&r, &mut new_p, &mut new_x, connections);
        cliques.append(&mut new_cliques);
        p.remove(&v);
        x.insert(v);
    }
    cliques
}

fn find_cliques(k: usize, connections: Vec<Vec<bool>>, nodes: Vec<&str>) -> usize {
    let mut cliques_count = 0;
    for elem in (0..nodes.len()).combinations(k) {
        if !elem.iter().map(|i| nodes[*i]).any(|x| x.starts_with('t')) {
            continue;
        }
        let mut is_clique = true;
        for edge in elem.iter().combinations(2) {
            let start_node = *edge[0];
            let end_node = *edge[1];
            if connections[start_node][end_node] == false {
                is_clique = false;
                break;
            }
        }
        if is_clique {
            cliques_count += 1
        }
    }
    cliques_count
}

fn parse(strings: &[String]) -> (Vec<&str>, Vec<Vec<bool>>) {
    let mut nodes = HashMap::new();
    for line in strings {
        let mut splitter_line = line.split('-');
        let start_node_str = splitter_line.next().unwrap();
        let end_node_str = splitter_line.next().unwrap();
        let nodes_len = nodes.len();
        nodes.entry(start_node_str).or_insert(nodes_len);
        let nodes_len = nodes.len();
        nodes.entry(end_node_str).or_insert(nodes_len);
    }
    let mut connections: Vec<Vec<bool>> = (0..nodes.len())
        .map(|_| (0..nodes.len()).map(|_| false).collect::<Vec<bool>>())
        .collect();
    for line in strings {
        let mut splitter_line = line.split('-');
        let start_node_str = splitter_line.next().unwrap();
        let end_node_str = splitter_line.next().unwrap();
        let start_node_id = *nodes.get(start_node_str).unwrap();
        let end_node_id = *nodes.get(end_node_str).unwrap();
        connections[start_node_id][end_node_id] = true;
        connections[end_node_id][start_node_id] = true;
    }
    let mut hash_vec: Vec<(&&str, &usize)> = nodes.iter().collect();
    hash_vec.sort_by(|a, b| a.1.cmp(b.1));
    let nodes: Vec<&str> = hash_vec.iter().map(|x| *x.0).collect();
    (nodes, connections)
}

fn parse_2(strings: &[String]) -> (HashSet<usize>, HashMap<usize, HashSet<usize>>, Vec<&str>) {
    let mut nodes = HashMap::new();
    for line in strings {
        let mut splitter_line = line.split('-');
        let start_node_str = splitter_line.next().unwrap();
        let end_node_str = splitter_line.next().unwrap();
        let nodes_len = nodes.len();
        nodes.entry(start_node_str).or_insert(nodes_len);
        let nodes_len = nodes.len();
        nodes.entry(end_node_str).or_insert(nodes_len);
    }
    let mut connections: HashMap<usize, HashSet<usize>> = HashMap::new();
    for line in strings {
        let mut splitter_line = line.split('-');
        let start_node_str = splitter_line.next().unwrap();
        let end_node_str = splitter_line.next().unwrap();
        let start_node_id = *nodes.get(start_node_str).unwrap();
        let end_node_id = *nodes.get(end_node_str).unwrap();
        connections
            .entry(start_node_id)
            .and_modify(|x| {
                x.insert(end_node_id);
            })
            .or_insert({
                let mut new_set = HashSet::new();
                new_set.insert(end_node_id);
                new_set
            });
        connections
            .entry(end_node_id)
            .and_modify(|x| {
                x.insert(start_node_id);
            })
            .or_insert({
                let mut new_set = HashSet::new();
                new_set.insert(start_node_id);
                new_set
            });
    }
    let nodes_ids = connections.keys().map(|x| *x).collect();
    let mut hash_vec: Vec<(&&str, &usize)> = nodes.iter().collect();
    hash_vec.sort_by(|a, b| a.1.cmp(b.1));
    let nodes_map: Vec<&str> = hash_vec.iter().map(|x| *x.0).collect();
    (nodes_ids, connections, nodes_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(7, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!("co,de,ka,ta".to_string(), part2(&get_input()));
    }
}
