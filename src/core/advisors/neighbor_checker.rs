use crate::core::link_struct::PageLink;
use crate::core::structs::balanced_tree::BalancedTree;
use crate::core::structs::tree_vectors::optimized_tree_vector::OptimizedTreeVec;


pub struct NeighborChecker {
    tree: BalancedTree<PageLink, OptimizedTreeVec<PageLink>>
}


impl NeighborChecker {
    pub fn new() -> NeighborChecker {
        let data = OptimizedTreeVec::new();
        NeighborChecker {
            tree: BalancedTree::new_with_compare(data,PageLink::compare_by_index)
        }
    }

    pub fn add_link(&mut self, link: PageLink) {
        self.tree.add(link);
    }

    pub fn check_for_neighbors(&mut self, link: &PageLink)
        -> Result<(Option<PageLink>, Option<PageLink>), String> {
        let mut res = (None, None);

        let mut left_neighbor  = self.tree.find_less_equal(link.clone());
        let mut right_neighbor = self.tree.find_more_equal(link.clone());

        if left_neighbor.is_some() && left_neighbor.unwrap().get_raw_end() + 1 == link.get_raw_index() {
            res.0 = left_neighbor;
        } else if left_neighbor.is_some() && left_neighbor.unwrap().get_raw_end() + 1 > link.get_raw_index() {
            let msg = format!("Link {} overlaps with link {}", link, left_neighbor.unwrap());
            return Err(msg);
        }

        if right_neighbor.is_some() && right_neighbor.unwrap().get_raw_index() == link.get_raw_end() + 1 {
            res.1 = right_neighbor;
        } else if right_neighbor.is_some() && right_neighbor.unwrap().get_raw_index() < link.get_raw_end() + 1 {
            let msg = format!("Link {} overlaps with link {}", link, right_neighbor.unwrap());
            return Err(msg);
        }

        Ok(res)
    }

    pub fn remove_link(&mut self, link: &PageLink) {
        self.tree.remove(link.clone());
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbor_checker_new() {
        let checker = NeighborChecker::new();
        assert_eq!(checker.tree.size(), 0);
    }

    #[test]
    fn test_neighbor_checker_add() {
        let mut checker = NeighborChecker::new();

        let link1 = PageLink::new(0, 0, 32);
        let link2 = PageLink::new(0,54,32);
        let link3 = PageLink::new(0,32,16);

        checker.add_link(link1);
        checker.add_link(link2);
        checker.add_link(link3);

        assert_eq!(checker.tree.get_by_index(0).value, link1);
        assert_eq!(checker.tree.get_by_index(1).value, link2);
        assert_eq!(checker.tree.get_root().indexes.index, 2);
    }

    #[test]
    fn test_neighbor_checker_check_for_neighbors() {
        let mut checker = NeighborChecker::new();

        let link1 = PageLink::new(0, 0, 32);
        let link2 = PageLink::new(0,54,32);
        let link3 = PageLink::new(0,32,16);

        checker.add_link(link1);
        checker.add_link(link2);
        checker.add_link(link3);

        let possible_new_link = PageLink::new(0,48,6);

        let neighbors = checker.check_for_neighbors(&possible_new_link);

        assert!(neighbors.is_ok());
        match neighbors {
            Ok((left, right)) => {
                assert_eq!(left.unwrap(), link3);
                assert_eq!(right.unwrap(), link2);
            },
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_neighbor_checker_check_for_neighbors_overlapping() {
        let mut checker = NeighborChecker::new();

        let link1 = PageLink::new(0, 0, 32);
        let link2 = PageLink::new(0,54,32);
        let link3 = PageLink::new(0,32,16);

        checker.add_link(link1);
        checker.add_link(link2);
        checker.add_link(link3);

        let possible_new_link = PageLink::new(0,48,32);

        let neighbors = checker.check_for_neighbors(&possible_new_link);

        assert!(neighbors.is_err());
    }

    #[test]
    fn test_neighbor_checker_remove() {
        let mut checker = NeighborChecker::new();

        let link1 = PageLink::new(0, 0, 32);
        let link2 = PageLink::new(0,54,32);
        let link3 = PageLink::new(0,32,16);

        checker.add_link(link1);
        checker.add_link(link2);
        checker.add_link(link3);

        checker.remove_link(&link2);

        assert_eq!(checker.tree.get_by_index(0).value, link1);
        assert_eq!(checker.tree.get_by_index(2).value, link3);
        assert_eq!(checker.tree.get_by_index(1).value, PageLink::default());
        assert_eq!(checker.tree.get_root().indexes.index, 2);
    }

}
