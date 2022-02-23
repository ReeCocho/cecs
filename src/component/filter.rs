use crate::archetype::Archetype;

use super::Component;

/// Describes a particular way to access a subset of entities based on what components they have.
pub trait ComponentFilter {
    /// Creates an archetype which has every component within the filter.
    fn archetype() -> Archetype;

    /// Creates an archetype which contains only components that are read.
    fn read_archetype() -> Archetype;

    /// Creates an archetype which contains only components that are written.
    fn write_archetype() -> Archetype;
}

/// Represents a request for access on a particular component (read or write).
pub trait ComponentAccess {
    /// The component type being accessed.
    type Component: Component + 'static;

    /// Indicates this access type needs mutable access.
    const MUTABLE: bool;
}

pub struct Read<T: Component> {
    _phantom: std::marker::PhantomData<T>,
}

pub struct Write<T: Component> {
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, C: Component + 'static> ComponentAccess for Read<C> {
    type Component = C;
    const MUTABLE: bool = false;
}

impl<'a, C: Component + 'static> ComponentAccess for Write<C> {
    type Component = C;
    const MUTABLE: bool = true;
}

macro_rules! component_filter_impl {
    ( $n:expr, $( $name:ident )+ ) => {
        impl<$($name: ComponentAccess,)*> ComponentFilter for ($($name,)*) {
            #[inline]
            fn archetype() -> Archetype {
                let mut archetype = Archetype::default();
                $(
                    archetype.add_component::<$name::Component>();
                )*
                archetype
            }

            #[inline]
            fn read_archetype() -> Archetype {
                let mut archetype = Archetype::default();
                $(
                    if !$name::MUTABLE {
                        archetype.add_component::<$name::Component>();
                    }
                )*
                archetype
            }

            #[inline]
            fn write_archetype() -> Archetype {
                let mut archetype = Archetype::default();
                $(
                    if $name::MUTABLE {
                        archetype.add_component::<$name::Component>();
                    }
                )*
                archetype
            }
        }
    }
}

component_filter_impl! { 1, A }
component_filter_impl! { 2, A B }
component_filter_impl! { 3, A B C }
component_filter_impl! { 4, A B C D }
component_filter_impl! { 5, A B C D E }
component_filter_impl! { 6, A B C D E F }
component_filter_impl! { 7, A B C D E F G }
component_filter_impl! { 8, A B C D E F G H }
component_filter_impl! { 9, A B C D E F G H I }
component_filter_impl! { 10, A B C D E F G H I J }
component_filter_impl! { 11, A B C D E F G H I J K }
component_filter_impl! { 12, A B C D E F G H I J K L }

#[cfg(test)]
mod tests {
    use crate::{
        archetype::Archetype,
        component::{
            filter::{ComponentFilter, Read, Write},
            Component,
        },
    };

    struct A {}
    struct B {}
    struct C {}

    impl Component for A {}
    impl Component for B {}
    impl Component for C {}

    #[test]
    fn component_filter_archetypes() {
        type Filter = (Read<A>, Read<B>, Write<C>);

        fn compare(a: Archetype, b: Archetype) -> bool {
            if a.len() != b.len() {
                return false;
            }

            for (a, b) in a.iter().zip(b.iter()) {
                if *a != *b {
                    return false;
                }
            }

            true
        }

        let mut archetype = Archetype::default();
        archetype.add_component::<A>();
        archetype.add_component::<B>();
        archetype.add_component::<C>();

        let mut reads = Archetype::default();
        reads.add_component::<A>();
        reads.add_component::<B>();

        let mut writes = Archetype::default();
        writes.add_component::<C>();

        assert!(compare(archetype, Filter::archetype()));
        assert!(compare(reads, Filter::read_archetype()));
        assert!(compare(writes, Filter::write_archetype()));
    }
}
