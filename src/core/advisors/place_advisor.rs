use crate::core::link_struct::PageLink;
use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectVec};
use crate::core::structs::tree::vectors::tree_vec::TreeVec;

// <V, M>
//     where
//         V: TreeVec<PageLink> + Sized,
//         M: TreeObject<PageLink> + Sized + TreeObjectVec<PageLink, V>

pub enum PlaceAdvisorStrategyName {
    BestFit,
    WorstFit,
}

struct PlaceAdvisor
{

}