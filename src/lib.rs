use std::collections::HashMap;
#[derive(Debug, PartialEq)]
pub struct Solution {}

/* key takeaways
   - A tree is a special undirected graph. It satisfies two properties
     - It is connected
     - It has no cycle.
   - the proper design of graph data structure
     - how it stores edges
     - how you mutate HashMap in Rust
   - have the concept of which node you are visiting
     from to detect the false cycles
   - ignore false cycle due to it's a undirected graph
     and how the graph is constructed
     - graph[0] = [1,...]
       graph[1] = [0,...]
     - false cycle
       you are visiting 0's neighbors 1, whose neighbors
       will also have 0 in it.
*/

impl Solution {
    pub fn valid_tree(list: &Vec<(usize, usize)>, n: &usize) -> bool {
        let mut visited: Vec<usize> = vec![];
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

        /*
          - we just randomly pick the first entry in the list
            as the starting node, but it really doesn't matter
            which node you start the visiting from
        */
        let (root, _) = list[0];

        /* construct the graph
           - use get_mut so you can mutate the vec
             (value) by inserting new element to
             the vec
           - what we are really doing is to store
             the neighbors of a given node in
             the value (a vec) of a dict entry
        */
        for (src, dst) in list.into_iter() {
            /* each edge will create two entries in the dict
               - remember this is a undirected graph we are
                 visiting
               - 0 is connected to 1 and vice versa
                 graph[0] = [1,...]
                 graph[1] = [0,...]
            */
            graph.entry(*src).or_insert(vec![]);
            graph.get_mut(src).unwrap().push(*dst);
            graph.entry(*dst).or_insert(vec![]);
            graph.get_mut(dst).unwrap().push(*src);
        }

        println!("graph: {:?}", graph);

        let no_cycle = Self::visit(&root, None, &mut visited, &graph);

        println!("visited: {:?}", &mut visited);
        visited.len() == *n && no_cycle
    }

    fn visit(
        node: &usize,
        from: Option<&usize>,
        visited: &mut Vec<usize>,
        graph: &HashMap<usize, Vec<usize>>,
    ) -> bool {
        //visit the node
        visited.push(*node);
        //visit its neighbors
        let neighbors = &graph[node];
        for i in 0..neighbors.len() {
            let element = &neighbors[i];
            if visited.contains(element) {
                if let Some(from) = from {
                    /* false cycle detected */
                    if *element == *from {
                        continue;
                    }
                    return false;
                }
            }
            let no_cycle = Self::visit(&element, Some(node), visited, graph);
            if !no_cycle {
                return false;
            }
        }

        /* no cycles detected */
        true
    }

    pub fn test_fixture_1() -> Vec<(usize, usize)> {
        vec![(0, 1), (0, 2), (0, 3), (1, 4)]
    }
    pub fn test_fixture_2() -> Vec<(usize, usize)> {
        vec![(0, 1), (1, 2), (2, 3), (1, 3), (1, 4)]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let result = Solution::valid_tree(&Solution::test_fixture_1(), &5);
        assert_eq!(result, true);
    }

    #[test]
    fn sample_2() {
        let result = Solution::valid_tree(&Solution::test_fixture_2(), &5);
        assert_eq!(result, false);
    }
}
