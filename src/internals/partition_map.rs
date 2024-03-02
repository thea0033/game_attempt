use std::collections::HashMap;
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct PartitionMapID(pub usize);
pub struct PartitionMap {
    pub player: Partition,
    pub map: HashMap<PartitionMapID, Partition>,
    pub next_id: usize,
    pub cache: Vec<PartitionMapID>,
}
impl PartitionMap {
    pub fn new(player: Partition) -> PartitionMap {
        let mut res = PartitionMap {
            player,
            map: HashMap::new(),
            next_id: 0,
            cache: Vec::new(),
        };
        res.reset_cache();
        res
    }
    pub fn add(&mut self, partition: Partition) -> PartitionMapID {
        let id = PartitionMapID(self.next_id);
        self.next_id += 1;
        self.map.insert(id, partition);
        id
    }
    pub fn remove(&mut self, id: PartitionMapID) {
        self.map.remove(&id);
        self.cache.push(id);
    }
    pub fn set(&mut self, id: PartitionMapID, new: Partition) {
        self.map.insert(id, new);
    }
    pub fn set_player(&mut self, map: Partition) {
        if self.player != map {
            self.player = map;
            self.reset_cache();
        }
    }
    pub fn reset_cache(&mut self) {
        self.cache.clear();
        for (id, val) in self.map.iter() {
            if val.collides(&self.player) {
                self.cache.push(*id);
            }
        }
    }
    pub fn clear(&mut self) {
        self.map.clear();
        self.cache.clear();
        self.next_id = 0;
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Partition {
    pub x: u64, // simple partition indicator. In binary, 1 means the object is inside this partition, 0 means it is not.
    pub y: u64,
}
impl Partition {
    pub fn collides(&self, other: &Partition) -> bool {
        (self.x & other.x) != 0 && (self.y & other.y) != 0
    }
}
