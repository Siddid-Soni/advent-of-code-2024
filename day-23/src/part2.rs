use crate::parse_input;
use std::{collections::{HashSet, HashMap}, path::Path};

pub fn process<P>(filename: P) -> String where P: AsRef<Path>  {
    let graph = parse_input(filename);
    
    let mut sets = HashSet::<Vec<String>>::new();
    
    fn search(node: String, req: HashSet<String>, sets: &mut HashSet<Vec<String>>, graph: &HashMap<String, HashSet<String>>) {
        let mut key = req.clone().into_iter().collect::<Vec<_>>();
        key.sort_unstable();
        if sets.contains(&key) {return;}
        sets.insert(key);
        for neigbour in &graph[&node] {
            if req.contains(neigbour) { continue; }
            if !(req.is_subset(&graph[neigbour])) {continue;}
            let mut copy = req.clone();
            copy.insert(neigbour.to_owned());
            search(neigbour.to_owned(), copy, sets, graph);
        }
    }
    for x in graph.keys() {
        search(x.to_owned(), HashSet::from([x.to_owned()]), &mut sets, &graph);
    }

    let mut ans = sets.iter().max_by_key(|x| x.len()).unwrap().clone();
    ans.sort_unstable();

    ans.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        assert_eq!("co,de,ka,ta", process("src/test.txt"));
    }
}