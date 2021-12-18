#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![feature(generators, generator_trait)]
#![feature(test)]
#![feature(drain_filter)]
#![feature(const_option)]
#![feature(type_alias_impl_trait)]

extern crate test;

use itertools::Itertools;

use std::cell::RefCell;
use std::error;
use std::rc::Rc;

mod utils;

////////////////////////////////////////////////////////////////////////////////////
/// The main function prints out the results for part1 and part2
/// AOC
fn main() -> Result<(), Box<dyn error::Error>> {
    utils::with_measure("Part 1", || solve_part1("test.txt"));
    utils::with_measure("Part 2", || solve_part2("test.txt"));
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////

/*
[[6,[5,[4,[3,2]]]],1] becomes [[6,[5,[7,0]]],3].

        /\
       /\ 1
      6 /\
       5 /\
        4 /\
         3  2
=>
        /\
       /\ 2+1
      6 /\
       5 /\
        7  0

[[[[[9,8],1],2],3],4] becomes [[[[0,9],2],3],4]
      /\
     /\ 4
    /\ 3
   /\ 2
  /\ 1
 9  8
 =>
      /\
     /\ 4
    /\ 3
   /\ 2
  0 1+8

 explode left
1. go up one level
1a. if on top STOP
1b. if coming from left GOTO 1
- coming from right
2. if left is number add to this one -> END
3. if left is no number but pair, go down into left
4. if right is number add to this one -> END
5. if right is no number but pair, go down into right
6. GOTO 4

explode right
1. go up one level
1a. if on top STOP
1b. if coming from right GOTO 1
- coming from left
2. if right is number add to this one -> END
3. if right is no number but pair, go down into right
4. if left is number add to this one -> END
5. if left is no number but pair, go down into left
6. GOTO 4

To split a regular number, replace it with a pair;
the left element of the pair should be the regular number divided by two and rounded down,
while the right element of the pair should be the regular number divided by two and rounded up.
For example, 10 becomes [5,5], 11 becomes [5,6], 12 becomes [6,6], and so on.

*/

type NumberType = i32;
type CoorType = (NumberType, NumberType);
type PathType = Vec<(CoorType, NumberType, NumberType)>;

//#[derive(Debug, PartialEq, Eq, FromPrimitive)]
struct TreeNode {
    parent: Option<Rc<RefCell<TreeNode>>>,
    value: Option<NumberType>,
    children: Vec<Rc<RefCell<TreeNode>>>,
}

impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TreeNode").field("value", &self.value).field("children", &self.children).finish()
    }
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = self.value {
            write!(f, "{}", value)
        } else {
            f.write_str("[");
            for (index, child) in self.children.iter().enumerate() {
                write!(f, "{}", child.as_ref().borrow());
                if index < self.children.len() -1 {
                    f.write_str(",");
                }
            }
            f.write_str("]")
        }
    }
}


impl TreeNode {
    fn create_value_node(value: NumberType) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            parent: None,
            value: Some(value),
            children: Vec::new(),
        }))
    }

    fn create_children_node(left_node: Rc<RefCell<TreeNode>>, right_node: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
        let parent = Rc::new(RefCell::new(TreeNode {
            parent: None,
            value: None,
            children: vec![
                Rc::clone(&left_node),
                Rc::clone(&right_node),
            ],
        }));

        left_node.borrow_mut().parent = Some(Rc::clone(&parent));
        right_node.borrow_mut().parent = Some(Rc::clone(&parent));

        parent
    }

    fn create_regular_pair(left: NumberType, right: NumberType) -> Rc<RefCell<TreeNode>> {
        let left_children_node = TreeNode::create_value_node(left);
        let right_children_node = TreeNode::create_value_node(right);

        TreeNode::create_children_node(left_children_node, right_children_node)
    }
}

/*
fn create_splitted(tree_node: Rc<RefCell<TreeNode>>, node_value: &NodeValue) -> TreeNode {
    match node_value {
        NodeValue::Leaf(value) => TreeNode::create_regular_pair(
            Some(tree_node.clone()),
            (*value as f64 / 2.0).floor() as NumberType,
            (*value as f64 / 2.0).ceil() as NumberType,
        ),
        NodeValue::SubTree(_) => unreachable!(),
    }
}

impl NodeValue {
    fn explode_left(&self) {
        /*
           1. go up one level
           1a. if on top STOP
           1b. if coming from left GOTO 1
           - coming from right
           2. if left is number add to this one -> END
           3. if left is no number but pair, go down into left
           4. if right is number add to this one -> END
           5. if right is no number but pair, go down into right
        */

        match self {
            NodeValue::Leaf(_) => todo!(),
            NodeValue::SubTree(_, _) => todo!(),
        }
    }

    fn left(&self) -> Option<&NodeValue> {
        match self {
            NodeValue::Leaf(_) => None,
            NodeValue::SubTree(left, _) => Some(left),
        }
    }

    fn right(&self) -> Option<&NodeValue> {
        match self {
            NodeValue::Leaf(_) => None,
            NodeValue::SubTree(_, right) => Some(right),
        }
    }

    fn left_mut(&mut self) -> Option<&mut Box<NodeValue>> {
        match self {
            NodeValue::Leaf(_) => None,
            NodeValue::SubTree(left, _) => Some(left),
        }
    }

    fn right_mut(&mut self) -> Option<&mut Box<NodeValue>> {
        match self {
            NodeValue::Leaf(_) => None,
            NodeValue::SubTree(_, right) => Some(right),
        }
    }
}
*/

fn parse(content : &str) -> TreeNode {
    if content[0] == '[' {
        let index = 1;
        
    }
}

fn solve_part1(content: &str) -> Result<NumberType, String> {
    let x = TreeNode::create_regular_pair(2,3);
    println!("x: {}", x.as_ref().borrow());
    let x = TreeNode::create_children_node(TreeNode::create_value_node(1), x);
    println!("x: {}", x.as_ref().borrow());

    for c in "[1,[2,3]]".chars()

    Ok(42)
}

fn solve_part2(_: &str) -> Result<usize, String> {
    Ok(42)

}


////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test0() -> Result<(), Box<dyn error::Error>> {
        Ok(())
    }

    #[test]
    fn test1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("test.txt").unwrap(), 10585);
        Ok(())
    }

    #[test]
    fn verify1() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part1("input.txt").unwrap(), 10585);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("test.txt").unwrap(), 5247);
        Ok(())
    }

    #[test]
    fn verify2() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(solve_part2("input.txt").unwrap(), 5247);
        Ok(())
    }

    #[bench]
    fn benchmark_part1(b: &mut Bencher) {
        b.iter(|| solve_part1("input.txt"));
    }

    #[bench]
    fn benchmark_part2(b: &mut Bencher) {
        b.iter(|| solve_part2("input.txt"));
    }
}
