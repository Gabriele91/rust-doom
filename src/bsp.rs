use std::rc::Rc;
// Engine
use crate::map::{Map, NodeBox};
use crate::math::Vector2;

// 0x8000 in binary 1000000000000000
const SUBSECTORIDENTIFIER: u16 = 0x8000;
const BSP_MAX_DEPTH: usize = ((u16::MAX / 2).ilog2() + 1) as usize;

pub struct BSP<'wad> {
    map: Rc<Map<'wad>>,
    root_id: u16,
    stack: Vec<u16>
}

impl<'wad> BSP<'wad> {
    pub fn new(map: &Rc<Map<'wad>>) -> Self {
        BSP {
            map: map.clone(),
            root_id: (map.nodes.len() - 1) as u16,
            stack: {
                let mut stack = vec![];
                stack.reserve(BSP_MAX_DEPTH);
                stack
            } 
        }
    }

    pub fn floor_height(&self, position: &Vector2<i16>) -> i16 {
        // Looking for sector id
        let mut node_id = self.root_id;
        // Search for the segment
        while node_id < SUBSECTORIDENTIFIER {
            let node = self.map.nodes[node_id as usize];
            if self.is_on_left_size(&position, node_id) {
                node_id = node.left_child_id;
            } else {
                node_id = node.right_child_id;
            }
        }
        if node_id & SUBSECTORIDENTIFIER > 0 { 
            let sub_sector_id = node_id & (!SUBSECTORIDENTIFIER);
            let first_seg_id = self.map.sub_sectors[sub_sector_id as usize].first_seg_id;
            if let Some(sector) = self.map.segs[first_seg_id as usize].right_sector(&self.map) {
                return sector.floor_height;
            }
        }
        return 0;
    }

    pub fn visit<'a,'b, T>(&mut self, position: &Vector2<i16>, context: &mut T, callback: impl FnMut(u16, &mut T) -> bool + 'a, test_node: impl FnMut(&NodeBox, &mut T) -> bool + 'b) {
        self.visit_aux(&position, context, self.root_id, callback, test_node);
    }

    fn visit_aux<'a,'b, T>(
        &mut self,
        position: &Vector2<i16>,
        context: &mut T,
        node_id: u16,
        mut callback: impl FnMut(u16, &mut T) -> bool + 'a,
        mut test_node: impl FnMut(&NodeBox, &mut T) -> bool + 'b
    ) {
        self.stack.push(node_id);
        while let Some(node_id) = self.stack.pop() {
            if node_id & SUBSECTORIDENTIFIER > 0 {
                if callback(node_id & (!SUBSECTORIDENTIFIER), context) {
                    continue;
                }
                return;
            }
    
            let node = self.map.nodes[node_id as usize];
    
            if self.is_on_left_size(&position, node_id) {
                self.stack.push(node.right_child_id);
                let left_box = node.left_box;
                if test_node(&left_box, context) {
                    self.stack.push(node.left_child_id);
                }
            } else {
                self.stack.push(node.left_child_id);
                let right_box = node.right_box;
                if test_node(&right_box, context) {
                    self.stack.push(node.right_child_id);
                }
            }
        }
    }
    
    pub fn visit_debug<'a, 'b, 'c>(
        &self,
        position: &Vector2<i16>,
        leaf_node: impl FnMut(u16) + 'a,
        first_node: impl FnMut(u16) + 'b,
        second_node: impl FnMut(u16) + 'c,
    ) {
        self.visit_debug_aux(&position, self.root_id, leaf_node, first_node, second_node);
    }

    fn visit_debug_aux<'a, 'b, 'c>(
        &self,
        position: &Vector2<i16>,
        node_id: u16,
        mut leaf_node: impl FnMut(u16) + 'a,
        mut first_node: impl FnMut(u16) + 'b,
        mut second_node: impl FnMut(u16) + 'c
    ) {
        let mut stack = vec![(node_id, true)];
        stack.reserve(BSP_MAX_DEPTH);

        while let Some((node_id, is_first)) = stack.pop() {
            if node_id & SUBSECTORIDENTIFIER > 0 {
                if is_first {
                    leaf_node(node_id & (!SUBSECTORIDENTIFIER));
                }
                continue;
            } else { 
                if is_first {
                    first_node(node_id);
                } else {
                    second_node(node_id);
                }
            }

            let node = self.map.nodes[node_id as usize];
    
            if self.is_on_left_size(&position, node_id) {
                stack.push((node.left_child_id, is_first));
                stack.push((node.right_child_id, false));
            } else {
                stack.push((node.right_child_id, is_first));
                stack.push((node.left_child_id, false));
            }
        }    
    }
    
    fn is_on_left_size(&self, position: &Vector2<i16>, node_id: u16) -> bool {
        let node = self.map.nodes[node_id as usize];
        let delta = Vector2::<i32>::from(&(*position - node.partition));
        let change_partition = node.change_partition;
        return delta.cross(&Vector2::<i32>::from(&change_partition)) <= 0;
    }
}
