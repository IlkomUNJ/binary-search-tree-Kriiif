use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type BstNodeLink = Rc<RefCell<BstNode>>;
pub type WeakBstNodeLink = Weak<RefCell<BstNode>>;

//this package implement BST wrapper
#[derive(Debug, Clone)]
pub struct BstNode {
    pub key: Option<i32>,
    pub parent: Option<WeakBstNodeLink>,
    pub left: Option<BstNodeLink>,
    pub right: Option<BstNodeLink>,
}

impl BstNode {
    //private interface
    fn new(key: i32) -> Self {
        BstNode {
            key: Some(key),
            left: None,
            right: None,
            parent: None,
        }
    }

    pub fn new_bst_nodelink(value: i32) -> BstNodeLink {
        let currentnode = BstNode::new(value);
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    /**
     * Get a copy of node link
     */
    pub fn get_bst_nodelink_copy(&self) -> BstNodeLink {
        Rc::new(RefCell::new(self.clone()))
    }

    fn downgrade(node: &BstNodeLink) -> WeakBstNodeLink {
        Rc::<RefCell<BstNode>>::downgrade(node)
    }

    //private interface
    fn new_with_parent(parent: &BstNodeLink, value: i32) -> BstNodeLink {
        let mut currentnode = BstNode::new(value);
        //currentnode.add_parent(Rc::<RefCell<BstNode>>::downgrade(parent));
        currentnode.parent = Some(BstNode::downgrade(parent));
        let currentlink = Rc::new(RefCell::new(currentnode));
        currentlink
    }

    //add new left child, set the parent to current_node_link
    pub fn add_left_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.left = Some(new_node);
    }

    //add new left child, set the parent to current_node_link
    pub fn add_right_child(&mut self, current_node_link: &BstNodeLink, value: i32) {
        let new_node = BstNode::new_with_parent(current_node_link, value);
        self.right = Some(new_node);
    }

    //search the current tree which node fit the value
    pub fn tree_search(&self, value: &i32) -> Option<BstNodeLink> {
        if let Some(key) = self.key {
            if key == *value {
                return Some(self.get_bst_nodelink_copy());
            }
            if *value < key && self.left.is_some() {
                return self.left.as_ref().unwrap().borrow().tree_search(value);
            } else if self.right.is_some() {
                return self.right.as_ref().unwrap().borrow().tree_search(value);
            }
        }
        //default if current node is NIL
        None
    }

    /**seek minimum by recurs
     * in BST minimum always on the left
     */
    pub fn minimum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(left_node) = &self.left {
                return left_node.borrow().minimum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    pub fn maximum(&self) -> BstNodeLink {
        if self.key.is_some() {
            if let Some(right_node) = &self.right {
                return right_node.borrow().maximum();
            }
        }
        self.get_bst_nodelink_copy()
    }

    /**
     * Return the root of a node, return self if not exist
     */
    pub fn get_root(node: &BstNodeLink) -> BstNodeLink {
        let parent = BstNode::upgrade_weak_to_strong(node.borrow().parent.clone());
        if parent.is_none() {
            return node.clone();
        }
        return BstNode::get_root(&parent.unwrap());
    }

    /**
     * NOTE: Buggy from pull request
     * Find node successor according to the book
     * Should return None, if x_node is the highest key in the tree
     */
    pub fn tree_successor(x_node: &BstNodeLink) -> Option<BstNodeLink> {
        // directly check if the node has a right child, otherwise go to the next block
        if let Some(right_node) = &x_node.borrow().right {
            return Some(right_node.borrow().minimum());
        } 
        
        // empty right child case
        else { 
            let mut x_node = x_node;
            let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
            let mut temp: BstNodeLink;

            while let Some(ref exist) = y_node {
                if let Some(ref left_child) = exist.borrow().left {
                    if BstNode::is_node_match(left_child, x_node) {
                        return Some(exist.clone());
                    }
                }

                temp = y_node.unwrap();
                x_node = &temp;
                y_node = BstNode::upgrade_weak_to_strong(temp.borrow().parent.clone());
            }

            None    
        }
    }

    pub fn transplant(root: &mut BstNodeLink, u: &BstNodeLink, v: &Option<BstNodeLink>) {
        let u_parent_weak = u.borrow().parent.clone();
    
        match u_parent_weak {
            // Case: u is root
            None => {
                if let Some(v_node) = v {
                    // Replace root's content with v's content
                    let v_data = v_node.borrow().clone();
                    root.borrow_mut().key = v_data.key;
                    root.borrow_mut().left = v_data.left.clone();
                    root.borrow_mut().right = v_data.right.clone();
    
                    // Update children's parent pointers
                    if let Some(left) = &root.borrow().left {
                        left.borrow_mut().parent = Some(Rc::downgrade(root));
                    }
                    if let Some(right) = &root.borrow().right {
                        right.borrow_mut().parent = Some(Rc::downgrade(root));
                    }
                } else {
                    // If v is None, we're clearing the root
                    root.borrow_mut().key = None;
                    root.borrow_mut().left = None;
                    root.borrow_mut().right = None;
                }
            }
    
            // Case: u is not root
            Some(u_par_weak) => {
                let u_par = BstNode::upgrade_weak_to_strong(Some(u_par_weak)).unwrap();
                let mut parent_borrow = u_par.borrow_mut();
                if let Some(left) = &parent_borrow.left {
                    if BstNode::is_node_match(left, u) {
                        parent_borrow.left = v.clone();
                    } else {
                        parent_borrow.right = v.clone();
                    }
                } else {
                    parent_borrow.right = v.clone(); // If no left child, must be right
                }
                if let Some(v_node) = v {
                    v_node.borrow_mut().parent = Some(Rc::downgrade(&u_par));
                }
            }
        }
    }    

    pub fn tree_delete(root: &mut BstNodeLink, z: &BstNodeLink) {
        if z.borrow().left.is_none() {
            let right = z.borrow().right.clone();
            BstNode::transplant(root, z, &right);
        } else if z.borrow().right.is_none() {
            let left = z.borrow().left.clone();
            BstNode::transplant(root, z, &left);
        } else {
            let right = z.borrow().right.clone().unwrap();
            let y = right.borrow().minimum();
    
            let y_parent = y.borrow().parent.clone().unwrap().upgrade().unwrap();
            if !Rc::ptr_eq(&y_parent, z) {
                let y_right = y.borrow().right.clone();
                BstNode::transplant(root, &y, &y_right);
                y.borrow_mut().right = z.borrow().right.clone();
                if let Some(ref right_node) = y.borrow().right {
                    right_node.borrow_mut().parent = Some(Rc::downgrade(&y));
                }
            }
    
            BstNode::transplant(root, z, &Some(y.clone()));
            y.borrow_mut().left = z.borrow().left.clone();
            if let Some(ref left_node) = y.borrow().left {
                left_node.borrow_mut().parent = Some(Rc::downgrade(&y));
            };
        }
    }    
       
    pub fn tree_insert(root: &BstNodeLink, new_node: &BstNodeLink) {
        let mut y: Option<BstNodeLink> = None;
        let mut x = Some(root.clone());
    
        while let Some(current) = x {
            y = Some(current.clone());
            let current_key = current.borrow().key.unwrap();
            let new_key = new_node.borrow().key.unwrap();
    
            x = if new_key < current_key {
                current.borrow().left.clone()
            } else {
                current.borrow().right.clone()
            };
        }
    
        new_node.borrow_mut().parent = y.as_ref().map(|node| Rc::downgrade(node));
    
        if let Some(parent) = y {
            let new_key = new_node.borrow().key.unwrap();
            let mut parent_borrow = parent.borrow_mut();
    
            if new_key < parent_borrow.key.unwrap() {
                parent_borrow.left = Some(new_node.clone());
            } else {
                parent_borrow.right = Some(new_node.clone());
            }
        }
    }           
    /**
     * Alternate simpler version of tree_successor that made use of is_nil checking
     */
    #[allow(dead_code)]
    pub fn tree_successor_simpler(x_node: &BstNodeLink) -> Option<BstNodeLink>{
        //create a shadow of x_node so it can mutate
        let mut x_node = x_node;
        let right_node = &x_node.borrow().right.clone();
        if BstNode::is_nil(right_node)!=true{
            return Some(right_node.clone().unwrap().borrow().minimum());
        }

        let mut y_node = BstNode::upgrade_weak_to_strong(x_node.borrow().parent.clone());
        let y_node_right = &y_node.clone().unwrap().borrow().right.clone();
        let mut y_node2: Rc<RefCell<BstNode>>;
        while BstNode::is_nil(&y_node) && BstNode::is_node_match_option(Some(x_node.clone()), y_node_right.clone()) {
            y_node2 = y_node.clone().unwrap();
            x_node = &y_node2;
            let y_parent = y_node.clone().unwrap().borrow().parent.clone().unwrap();
            y_node = BstNode::upgrade_weak_to_strong(Some(y_parent));
        }

        //in case our sucessor traversal yield root, means self is the highest key
        if BstNode::is_node_match_option(y_node.clone(), Some(BstNode::get_root(&x_node))) {
            return None;
        }

        //default return self / x_node
        return Some(y_node.clone().unwrap())
    }

    /**
     * private function return true if node doesn't has parent nor children nor key
     */
    fn is_nil(node: &Option<BstNodeLink>) -> bool {
        match node {
            None => true,
            Some(x) => {
                if x.borrow().parent.is_none()
                    || x.borrow().left.is_none()
                    || x.borrow().right.is_none()
                {
                    return true;
                }
                return false;
            }
        }
    }

    //helper function to compare both nodelink
    fn is_node_match_option(node1: Option<BstNodeLink>, node2: Option<BstNodeLink>) -> bool {
        if node1.is_none() && node2.is_none() {
            return true;
        }
        if let Some(node1v) = node1 {
            return node2.is_some_and(|x: BstNodeLink| x.borrow().key == node1v.borrow().key);
        }
        return false;
    }

    fn is_node_match(anode: &BstNodeLink, bnode: &BstNodeLink) -> bool {
        if anode.borrow().key == bnode.borrow().key {
            return true;
        }
        return false;
    }

    /**
     * As the name implied, used to upgrade parent node to strong nodelink
     */
    fn upgrade_weak_to_strong(node: Option<WeakBstNodeLink>) -> Option<BstNodeLink> {
        match node {
            None => None,
            Some(x) => Some(x.upgrade().unwrap()),
        }
    }
}
