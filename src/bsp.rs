use crate::map::Map;
use crate::math::Vector2;

// 0x8000 in binary 1000000000000000
const SUBSECTORIDENTIFIER: u16 = 0x8000;

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

    pub fn visit_debug<'a,'b>(&self, position: &Vector2<i16>, callback_segs: impl FnMut(u16) + 'a, callback_node: impl FnMut(u16) + 'b) {
        self.visit_debug_aux(&position, self.root_id, callback_segs, callback_node);
    }

    fn visit_aux<'a>(
        &self,
        position: &Vector2<i16>,
        node_id: u16,
        mut callback: impl FnMut(u16) + 'a,
    ) {
        // Masking all the bits except the last one
        // to check if this is a subsector
        if node_id & SUBSECTORIDENTIFIER > 0 {
            callback(node_id & (!SUBSECTORIDENTIFIER));
            return;
        }

        // It is a node
        let node = self.map.nodes[node_id as usize];

        // Left or right side
        if self.is_on_left_size(&position, node_id) {
            self.visit_aux(&position, node.left_child_id, callback);
        } else {
            self.visit_aux(&position, node.right_child_id, callback);
        }
    }

    fn visit_debug_aux<'a,'b>(
        &self,
        position: &Vector2<i16>,
        node_id: u16,
        mut callback_segs: impl FnMut(u16) + 'a,
        mut callback_node: impl FnMut(u16) + 'b,
    ) {
        // Masking all the bits except the last one
        // to check if this is a subsector
        if node_id & SUBSECTORIDENTIFIER > 0 {
            callback_segs(node_id & (!SUBSECTORIDENTIFIER));
            return;
        } else {
            callback_node(node_id);
        }

        // It is a node
        let node = self.map.nodes[node_id as usize];

        // Left or right side
        if self.is_on_left_size(&position, node_id) {
            self.visit_debug_aux(&position, node.left_child_id, callback_segs, callback_node);
        } else {
            self.visit_debug_aux(&position, node.right_child_id, callback_segs, callback_node);
        }
    }

    fn is_on_left_size(&self, position: &Vector2<i16>, node_id: u16) -> bool {
        let node = self.map.nodes[node_id as usize];
        let delta = Vector2::<i32>::from(&(*position - node.partition));
        let change_partition = node.change_partition;
        return delta.cross(& Vector2::<i32>::from(&change_partition)) <= 0;
    }
}
