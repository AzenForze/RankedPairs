pub mod neighboring;

use std::collections::HashSet;
use std::hash::Hash;
use dfs::neighboring::Neighboring;

pub struct DFS<T> where T: Neighboring
{
    stack: Vec<T::Type>,
    visits: HashSet<T::Type>,
}

impl<T, U> DFS<T> where
    U: Eq + Hash + Clone,
    T: Neighboring<Type=U>
{
    pub fn new(start: &T::Type) -> Self
    {
        let mut v: HashSet<T::Type> = HashSet::new();
        v.insert(start.clone());
        DFS { stack: vec![start.clone()], visits: v }
    }

    pub fn next(&mut self, graph: &T) -> Option<T::Type>
    {
        let n = match self.stack.pop()
        {
            Some(n) => n,
            None => return None
        };

        for adj in graph.get_neighbors(&n)
        {
            if self.visits.insert(adj.clone())
            {
                self.stack.push(adj.clone());
            }
        }
        
        return Some(n);
    }
}
