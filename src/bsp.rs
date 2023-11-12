use crate::map::Map;
use crate::math::Vector2;

// 0x8000 in binary 1000000000000000
const SUBSECTORIDENTIFIER: u16 = 0x8000;
const BSP_MAX_DEPTH: usize = ((u16::MAX / 2).ilog2() + 1) as usize;

pub struct BSP<'wad> {
    map: Box<Map<'wad>>,
    root_id: u16,
}

impl<'wad> BSP<'wad> {
    pub fn new(map: &Box<Map<'wad>>) -> Self {
        BSP {
            map: map.clone(),
            root_id: (map.nodes.len() - 1) as u16,
        }
    }

    pub fn visit<'a>(&self, position: &Vector2<i16>, callback: impl FnMut(u16) + 'a) {
        self.visit_aux(&position, self.root_id, callback);
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

    fn visit_aux<'a>(
        &self,
        position: &Vector2<i16>,
        node_id: u16,
        mut callback: impl FnMut(u16) + 'a
    ) {
        let mut stack = vec![node_id];
        stack.reserve(BSP_MAX_DEPTH);
        
        while let Some(node_id) = stack.pop() {
            if node_id & SUBSECTORIDENTIFIER > 0 {
                callback(node_id & (!SUBSECTORIDENTIFIER));
                continue;
            }
    
            let node = self.map.nodes[node_id as usize];
    
            if self.is_on_left_size(&position, node_id) {
                stack.push(node.right_child_id);
                stack.push(node.left_child_id);
            } else {
                stack.push(node.left_child_id);
                stack.push(node.right_child_id);
            }
        }
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
