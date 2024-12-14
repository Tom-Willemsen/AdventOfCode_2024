use bitvec::prelude::*;

/// Emulates part of the interface of a HashSet<(usize, usize)>, with bounded
/// allowed positions, using a bitvec as underlying storage.
pub struct BitVecSet2D {
    pub bv: BitVec,
    pub bounds: (usize, usize),
}

impl BitVecSet2D {
    pub fn new(bounds: (usize, usize)) -> BitVecSet2D {
        BitVecSet2D {
            bv: bitvec![usize, Lsb0; 0; bounds.0 * bounds.1],
            bounds,
        }
    }

    fn idx(&self, pos: &(usize, usize)) -> usize {
        pos.0 * self.bounds.1 + pos.1
    }

    pub fn get(&self, pos: &(usize, usize)) -> Option<bool> {
        if pos.0 >= self.bounds.0 || pos.1 >= self.bounds.1 {
            None
        } else {
            self.bv.get(self.idx(pos)).map(|b| *b)
        }
    }

    pub fn contains(&self, pos: &(usize, usize)) -> bool {
        self.get(pos).is_some_and(|b| b)
    }

    pub fn insert(&mut self, pos: (usize, usize)) -> bool {
        debug_assert!(pos.0 < self.bounds.0);
        debug_assert!(pos.1 < self.bounds.1);
        let idx = self.idx(&pos);
        !self
            .bv
            .get_mut(idx)
            .expect("invalid bv index")
            .replace(true)
    }

    pub fn remove(&mut self, pos: (usize, usize)) -> bool {
        debug_assert!(pos.0 < self.bounds.0);
        debug_assert!(pos.1 < self.bounds.1);
        let idx = self.idx(&pos);
        self.bv
            .get_mut(idx)
            .expect("invalid bv index")
            .replace(false)
    }

    pub fn is_empty(&self) -> bool {
        self.bv.not_any()
    }

    pub fn len(&self) -> usize {
        self.bv.count_ones()
    }

    pub fn clear(&mut self) {
        self.bv.fill(false);
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> + use<'_> {
        self.bv
            .iter_ones()
            .map(|b| (b / self.bounds.1, b % self.bounds.1))
    }
}
