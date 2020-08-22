use std::{cmp::{max, min}, hash::Hash};
use std::collections::{hash_map::{Entry, HashMap}, VecDeque};

pub struct Trie<C: Hash + Eq> {
    links: Vec<HashMap<C, usize>>,
}

impl<C: Hash + Eq> Default for Trie<C> {
    fn default() -> Self {
        Self { links: vec![HashMap::new()] }
    }
}

impl<C: Hash + Eq> Trie<C> {
    pub fn insert(&mut self, word: impl IntoIterator<Item=C>) -> usize {
        let mut node = 0;
        word.into_iter().for_each(|ch| {
            let len = self.links.len();
            node = match self.links[node].entry(ch) {
                Entry::Occupied(ent) => *ent.get(),
                Entry::Vacant(ent) => {
                    ent.insert(len);
                    self.links.push(HashMap::new());
                    len
                }
            }
        });
        node
    }

    pub fn get(&self, word: impl IntoIterator<Item=C>) -> Option<usize> {
        let mut node = 0;
        word.into_iter().for_each(|ch| {
            node = *self.links[node].get(&ch).unwrap();
        });
        Some(node)
    }

}

pub struct Match<'a, C: Eq> {
    pub pat: &'a [C],
    pub fail: Vec<usize>,
}

pub struct MultiMatch<C: Hash + Eq> {
    pub trie: Trie<C>,
    pub pat_id: Vec<Option<usize>>,
    pub fail: Vec<usize>,
    pub fast: Vec<usize>,
}
