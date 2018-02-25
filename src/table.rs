use std::collections::HashMap;
use std::collections::hash_map;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

#[derive(PartialEq, Eq, Hash)]
pub struct Pair<R,C>(R,C);

#[derive(PartialEq, Eq, Hash)]
struct BPair<'a,'b,R:'a,C:'b>(&'a R,&'b C);

trait KeyPair<R,C> {
    fn row(&self) -> &R;
    fn column(&self) -> &C;
}

impl<R, C> KeyPair<R,C> for Pair<R,C> {
    fn row(&self) -> &R { &self.0 }
    fn column(&self) -> &C { &self.1 }
}
impl<'a, 'b, R, C> KeyPair<R,C> for BPair<'a, 'b, R, C> {
    fn row(&self) -> &R { self.0 }
    fn column(&self) -> &C { self.1 }
}


impl<'a, R:'a, C:'a> Borrow< KeyPair<R,C> +'a > for Pair<R,C>
{
    fn borrow(&self) -> &(KeyPair<R,C>+'a)
    {
        self
    }
}

impl<'a, R:Eq,C:Eq> PartialEq for KeyPair<R,C> + 'a
{
    fn eq(&self, other: &KeyPair<R,C>) -> bool
    {
        self.row() == other.row() && self.column() == other.column()
    }
}

impl<'a, R:Eq,C:Eq> Eq for KeyPair<R,C> + 'a {}

impl<'a, R:Hash, C:Hash> Hash for KeyPair<R,C> + 'a
{
    fn hash<H>(&self, state: &mut H) where H: Hasher
    {
        self.row().hash(state);
        self.column().hash(state);
    }
}

pub struct Table<R:Eq+Hash, C:Eq+Hash, V>
{
    map: HashMap<Pair<R, C>, V>
}


pub struct Values<'a, R:'a, C:'a, V:'a>
{
    adapt: hash_map::Values<'a, Pair<R, C>, V>
}

impl<'a, R:'a, C:'a, V:'a> Iterator for Values<'a, R, C, V>
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item>
    {
        self.adapt.next()
    }
}

impl<R:Eq+Hash, C:Eq+Hash, V> Table<R, C, V>
{
    pub fn new() -> Self
    {
        Table { map: HashMap::new() }
    }
    
    pub fn get_mut(&mut self, row: &R, column: &C) -> Option<&mut V>
    {
        self.map.get_mut(&BPair(row, column) as &KeyPair<R,C>)
    }
    
    pub fn insert(&mut self, row: R, column: C, value: V) -> Option<V>
    {
        self.map.insert(Pair(row, column), value)
    }
    
    pub fn values(&self) -> Values<R, C, V>
    {
        Values{ adapt: self.map.values() }
    }
}