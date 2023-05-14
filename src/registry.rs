pub type ID = usize;

#[derive(Default, PartialEq, Debug)]
pub struct Registry<T> {
    map: Vec<(ID, Option<T>)>,
    size: usize,
    id: ID,
}

impl<T> Registry<T> {
    // TODO remove mut
    pub fn get(&mut self, t: ID) -> Option<&T> {
        self._search(t).and_then(|x| x.as_ref())
    }

    pub fn get_mut(&mut self, t: ID) -> Option<&mut T> {
        self._search(t).and_then(|x| x.as_mut())
    }

    fn _search(&mut self, t: ID) -> Option<&mut Option<T>> {
        let i = self.map.partition_point(|x| x.0 < t);
        let ok = i != self.map.len() && self.map[i].0 == t;
        if ok { Some(&mut self.map[i].1) } else { None }
    }

    pub fn append(&mut self, val: T) -> ID {
        self.map.push( (self.id, Some(val)));
        let token = self.id;
        self.size += 1;
        self.id += 1;
        token
    }

    pub fn erase(&mut self, t: ID) { 
        let x = self._search(t);
        if x.is_none() { return; }
        *x.unwrap() = None;
        self.size -= 1; 
        if self.size < self.map.len() / 2 {
            self.map.retain(|x| x.1.is_some());
        }
    }
}