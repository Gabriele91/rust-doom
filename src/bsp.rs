use crate::map::Map;
use crate::math::Vector2;

// 0x8000 in binary 1000000000000000
const SUBSECTORIDENTIFIER: u16 = 0x8000;

struct BSP<'wad> {
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

    pub fn visit(&self, position: &Vector2<i16>, mut callback: impl FnMut(u16) + 'static) {
        self.visit_aux(&position, self.root_id, callback);
    }

    fn visit_aux(
        &self,
        position: &Vector2<i16>,
        node_id: u16,
        mut callback: impl FnMut(u16) + 'static,
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

    fn is_on_left_size(&self, position: &Vector2<i16>, node_id: u16) -> bool {
        let node = self.map.nodes[node_id as usize];
        let delta = *position - node.partition;
        let change_partition = node.change_partition;
        return delta.cross(&change_partition) <= 0;
    }
}
