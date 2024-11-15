use std::fmt::Debug;

fn main() {
    let mut v = Vec::new();
    v.push(3);
    v.push(1);
    v.push(6);
    v.push(4);
    v.push(7);
    v.push(0);
    let mut tree=TreeNode::from_vec(v);
    tree.pre_order_visit();
}
//ES1
struct TreeNode<T: PartialOrd + Clone + Debug> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> where T: PartialOrd + Clone + Debug {
    pub fn new(value: T) -> Self {
        Self { value, left: None, right: None }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut root = TreeNode::new(vec[0].clone());
        for i in 1..vec.len(){
            Self::insert(&mut root,vec[i].clone())
        }
        root
    }

    pub fn insert(&mut self, value: T) {
        if value > self.value{
            match self.right{
                None => self.right = Some(Box::new(TreeNode::new(value))),
                Some(ref mut right) => right.insert(value)
            }
        }else{
            match self.left{
                None => self.left = Some(Box::new(TreeNode::new(value))),
                Some(ref mut left) => left.insert(value)
            }
        }
    }

    pub fn pre_order_visit(&self){
        println!("{:?}",self.value);
        if let Some(ref left) = self.left{
            left.pre_order_visit()
        }
        if let Some(ref right) = self.right{
            right.pre_order_visit()
        }
    }

}
