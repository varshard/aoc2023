use std::cmp::Ordering;
use std::ops::Range;

pub struct Node {
	dst_range: Range<u64>,
	src_range: Range<u64>,
	left: Option<Box<Node>>,
	right: Option<Box<Node>>,
}

impl Node {
	pub fn new(dst_range: Range<u64>, src_range: Range<u64>) -> Self {
		Node {
			dst_range,
			src_range,
			left: None,
			right: None,
		}
	}
	pub fn insert(&mut self, node: Node) {
		// just like usual less than, but cmp will technically reduce the chance of using the comparison backward or forgot to handle an equal
		match self.src_range.start.cmp(&node.src_range.start) {
			Ordering::Greater => {
				match self.left {
					Some(ref mut left) => {
						left.insert(node);
					}
					None => {
						self.left = Some(Box::new(node));
					}
				}
			}
			Ordering::Less => {
				match self.right {
					Some(ref mut right) => {
						right.insert(node);
					}
					None => {
						self.right = Some(Box::new(node));
					}
				}
			}
			_ => {}
		}
	}

	pub fn find(&self, value: &u64) -> u64 {
		if self.src_range.contains(value) {
			let diff = value - self.src_range.start;
			return self.dst_range.start + diff;
		}
		return match self.src_range.start.cmp(&value) {
			Ordering::Greater => {
				match self.left {
					Some(ref left) => {
						left.find(value)
					}
					None => {
						*value
					}
				}
			}
			Ordering::Less => {
				match self.right {
					Some(ref right) => {
						right.find(value)
					}
					None => {
						*value
					}
				}
			}
			Ordering::Equal => {
				self.dst_range.start
			}
		};
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_should_work() {
		let mut tree = Node::new(50..52, 98..100);
		tree.insert(Node::new(52..100, 50..98));

		assert_eq!(81, tree.find(&79));
		assert_eq!(48, tree.find(&48));
	}
}