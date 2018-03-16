use std::collections::HashMap;
use std::collections::hash_map;
use std::hash::{Hash, Hasher};
use std::borrow::Borrow;

#[derive(PartialEq, Eq, Hash)]
pub struct Pair<R,C>(R,C);

#[derive(PartialEq, Eq, Hash)]
struct BPair<'a,'b, R:'a+?Sized, C:'b+?Sized>(&'a R, &'b C);

trait KeyPair<R,C> where
    R: ?Sized,
    C: ?Sized
{
    fn row(&self) -> &R;
    fn column(&self) -> &C;
}


impl<R, C, Q, U> KeyPair<Q, U> for Pair<R, C> where
    R: Borrow<Q>,
    C: Borrow<U>,
    Q: ?Sized,
    U: ?Sized
{
    fn row(&self) -> &Q { self.0.borrow() }
    fn column(&self) -> &U { self.1.borrow() }
}


impl<'a, 'b, R:?Sized, C:?Sized> KeyPair<R,C> for BPair<'a, 'b, R, C> {
    fn row(&self) -> &R { self.0 }
    fn column(&self) -> &C { self.1 }
}

impl<'a, R:'a, C:'a, Q, U> Borrow< KeyPair<Q,U> +'a > for Pair<R,C> where
    R: Borrow<Q>,
    C: Borrow<U>,
    Q: ?Sized,
    U: ?Sized
{
    fn borrow(&self) -> &(KeyPair<Q,U>+'a)
    {
        self
    }
}

impl<'a, R:Eq+?Sized,C:Eq+?Sized> PartialEq for KeyPair<R,C> + 'a
{
    fn eq(&self, other: &KeyPair<R,C>) -> bool
    {
        self.row() == other.row() && self.column() == other.column()
    }
}

impl<'a, R:Eq+?Sized,C:Eq+?Sized> Eq for KeyPair<R,C> + 'a {}

impl<'a, R:Hash+?Sized, C:Hash+?Sized> Hash for KeyPair<R,C> + 'a
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

impl<R:Eq+Hash, C:Eq+Hash, V> Table<R, C, V>
{
    pub fn new() -> Self
    {
        Table { map: HashMap::new() }
    }
    
    pub fn get_mut<Q, U>(&mut self, row: &Q, column: &U) -> Option<&mut V> where
        R: Borrow<Q>,
        C: Borrow<U>,
        Q: Eq+Hash+?Sized,
        U: Eq+Hash+?Sized
    {
        self.map.get_mut(&BPair(row, column) as &KeyPair<Q,U>)
    }
    
    pub fn insert(&mut self, row: R, column: C, value: V) -> Option<V>
    {
        self.map.insert(Pair(row, column), value)
    }
    
    pub fn values(&self) -> Values<R, C, V>
    {
        Values{ adapt: self.map.values() }
    }

    pub fn contains<Q, U>(&self, row: &Q, column: &U) -> bool where
        R: Borrow<Q>,
        C: Borrow<U>,
        Q: Eq+Hash+?Sized,
        U: Eq+Hash+?Sized
    {
        self.map.contains_key(&BPair(row, column) as &KeyPair<Q, U>)
    }

    pub fn entry<'a, 'b, Q, U>(&'a mut self, row: &'b Q, column: &'b U) -> TableEntry<'a, 'b, R, C, Q, U, V>
        where
        R: Borrow<Q>,
        C: Borrow<U>,
        Q: Eq+Hash+?Sized,
        U: Eq+Hash+?Sized
    {
        if self.contains(row, column)
        {
            TableEntry::Occupied( OccupiedEntry { map: self, row: row, column: column  } )
        }
        else {
            TableEntry::Vacant( VacantEntry { map: self, row: row, column: column } )
        }
    }
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



/// An entry that takes references as keys which can be cloned if they need to be inserted.
pub enum TableEntry<'a, 'b, R:'a, C:'a, Q:'b, U:'b, V:'a>
    where
    R:Eq+Hash,
    C: Eq+Hash,
    Q: ?Sized,
    U: ?Sized
{
    Vacant(VacantEntry<'a, 'b, R, C, Q, U, V>),
    Occupied(OccupiedEntry<'a, 'b, R, C, Q, U, V>)
}



impl<'a, 'b, R, C, Q:?Sized, U:?Sized, V> 
TableEntry<'a, 'b, R, C, Q, U, V>
    where
    R:Eq+Hash,
    C: Eq+Hash
{
    /// Returns the value, or insers and returns the provided value.
    pub fn or_insert(self, val: V) -> &'a mut V
        where
        R: Borrow<Q>,
        C: Borrow<U>,
        Q: ToOwned<Owned=R>+Eq+Hash,
        U: ToOwned<Owned=C>+Eq+Hash
    {
        match self
        {
            TableEntry::Vacant(entry) => { entry.insert(val) },
            TableEntry::Occupied(entry) => { entry.get() }
        }
    }
}



pub struct VacantEntry<'a, 'b, R:'a, C:'a, Q:'b, U:'b, V:'a>
    where
    R: Eq+Hash,
    C: Eq+Hash,
    Q: ?Sized,
    U: ?Sized
{
    map: &'a mut Table<R, C, V>,
    row: &'b Q,
    column: &'b U
}



impl<'a, 'b, R, C, Q, U, V> 
VacantEntry<'a, 'b, R, C, Q, U, V>
    where
    R: Eq+Hash+Borrow<Q>,
    C: Eq+Hash+Borrow<U>,
    Q: Eq+Hash+ToOwned<Owned=R>+?Sized,
    U: Eq+Hash+ToOwned<Owned=C>+?Sized
{
    pub fn insert(self, val: V) -> &'a mut V
    {
        self.map.insert(self.row.to_owned(), self.column.to_owned(), val);
        self.map.get_mut(self.row, self.column).unwrap()
    }
}



pub struct OccupiedEntry<'a, 'b, R:'a, C:'a, Q:'b, U:'b, V:'a>
    where
    R: Eq+Hash,
    C: Eq+Hash,
    Q: ?Sized,
    U: ?Sized
{
    map: &'a mut Table<R, C, V>,
    row: &'b Q,
    column: &'b U
}



impl<'a, 'b, R, C, Q, U, V> 
OccupiedEntry<'a, 'b, R, C, Q, U, V>
    where
    R: Eq+Hash+Borrow<Q>,
    C: Eq+Hash+Borrow<U>,
    Q: Eq+Hash+?Sized,
    U: Eq+Hash+?Sized
{
    pub fn get(self) -> &'a mut V
    {
        self.map.get_mut(self.row, self.column).unwrap()
    }
}