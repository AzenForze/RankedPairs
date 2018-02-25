use std::collections::HashMap;
use std::fmt::{self, Formatter, Display};

use dfs::DFS;
use dfs::neighboring::Neighboring;
use std::hash::Hash;

pub struct Graph<T>(HashMap<T, Vec<T>>);

impl<T> Graph<T> where
T: Eq + Hash + Clone
{
    pub fn new() -> Self
    {
        return Graph(HashMap::new());
    }

    /**
    Creates a path from 'from' to 'to'.
    */
    pub fn add_edge(&mut self, from: T, to: T)
    {
        let Graph(ref mut map) = *self;
        
        map.entry(from).or_insert(Vec::new()).push(to);
    }

    /**
    Finds the node that all nodes lead to.
    */
    pub fn find_sink(&self) -> Result<T, EmptyGraphError>
    {
        let Graph(ref graph) = *self;

        let mut any = match graph.iter().next()
        {
            Some((any, _)) => any,
            None => return Err(EmptyGraphError::new("Graph was empty".to_owned()))
        };
        
        while graph.contains_key(any)
        {
            let ref neighbors = graph[any];
            
            any = match neighbors.iter().next()
            {
                Some(any) => any,
                None => break
            };

        }
        
        return Ok(any.clone());
    }

    /**
    Checks if it's possible to get to 'to' from 'from'.
    Uses a depth-first-search.
    */
    pub fn is_path(&self, from: &T, to: &T) -> bool
    {
        let mut dfs = DFS::new(from);
        
        while let Some(next) = dfs.next(self)
        {
            if next == *to { return true }
        }

        return false;
    }
}

impl<T> Neighboring for Graph<T> where
T: Eq + Hash
{
    type Type = T;

    fn get_neighbors(&self, node: &T) -> ::std::slice::Iter<T>
    {
        let Graph(ref map) = *self;

        return match map.get(node)
        {
            Some(neighbors) => neighbors.iter(),
            None => [].iter()
        };
    }
}

impl<T> Display for Graph<T> where
T: Hash + Eq + Display
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let Graph(ref map) = *self;

        if map.len() <= 0
        {
            return write!(f, ""); 
        }

        let mut out = String::new();

        for (node, neighs) in map.iter()
        {
            out.push_str(&format!("{}: ", node));

            for n in neighs
            {
                out.push_str(&format!("{},  ", n));
            }

            let s = out.len();
            out.truncate(s-3);
            out.push_str("\n");
        }

        let s = out.len();
        out.truncate(s-1);
        
        write!(f, "{}", out)
    }
}

#[derive(Debug)]
pub struct EmptyGraphError {
    description: String
}

impl EmptyGraphError
{
    fn new(description: String) -> Self
    {
        EmptyGraphError { description: description }
    }
}

use std::error::Error;

impl Display for EmptyGraphError
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for EmptyGraphError
{
    fn description(&self) -> &str
    {
        return &self.description;
    }
}
