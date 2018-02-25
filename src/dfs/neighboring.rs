pub trait Neighboring
{
    type Type;
    fn get_neighbors(&self, node: &Self::Type) -> ::std::slice::Iter<Self::Type>;
}
