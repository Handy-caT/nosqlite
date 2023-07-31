use crate::core::link_struct::PageLink;
use crate::core::structs::tree::object::balanced_tree::decoratable_balanced_tree::DecoratableBalancedTree;
use crate::core::structs::tree::object::tree_object::{TreeObject, TreeObjectVec};
use crate::core::structs::tree::vectors::tree_vec::{TreeVec, TreeVecIndexes, TreeVecLevels};

struct EmptyLinkRegistry<V, M>
    where
        V: TreeVec<PageLink> + Sized,
        M: TreeObject<PageLink> + Sized + TreeObjectVec<PageLink, V>
{
    data: DecoratableBalancedTree<PageLink, V, M>
}

impl <V, M> EmptyLinkRegistry<V, M>
    where
        V: TreeVec<PageLink> + TreeVecLevels + Sized + TreeVecIndexes<PageLink>,
        M: TreeObject<PageLink> + Sized + TreeObjectVec<PageLink, V>
{
    pub fn new(data: DecoratableBalancedTree<PageLink, V, M>) -> Self {
        EmptyLinkRegistry {
            data
        }
    }

    pub fn add_link(&mut self, link: PageLink) {
        self.data.push(link);
    }

    pub fn remove_link(&mut self, link: PageLink) {
        self.data.remove_by_value(link);
    }

    pub fn pop(&mut self) -> Option<PageLink> {
        self.data.remove_by_index(0)
    }
}


#[cfg(test)]
mod tests {

}
