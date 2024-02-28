use std::collections::{HashMap, HashSet};
#[allow(dead_code)]
pub struct Graph<V> {
    pub vertices: HashMap<u64, V>,
    pub adjancies: HashMap<u64, HashSet<u64>>,
}

#[allow(dead_code)]
impl<V> Graph<V> {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            adjancies: HashMap::new(),
        }
    }

    pub fn push_vertice(&mut self, id: u64, vertex: V) {
        self.vertices.insert(id, vertex);
    }

    //TODO: Optimization of batch insert of the dest
    pub fn push_edge(&mut self, src: u64, dest: u64) {
        let edges = self.adjancies.get_mut(&src);
        match edges {
            Some(val) => {
                val.insert(dest);
            }
            None => {
                let set: HashSet<u64> = HashSet::from([dest]);
                self.adjancies.insert(src, set);
            }
        }
    }
}
