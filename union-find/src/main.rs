use std::collections::HashMap;
use std::hash::Hash;

struct UnionFind<T: Eq + Hash> {
    members: HashMap<T, usize>,
    groups: HashMap<usize, usize>,
}

impl<T: Eq + Hash> UnionFind<T> {
    fn new() -> Self {
        let members: HashMap<T, usize> = HashMap::new();
        let groups: HashMap<usize, usize> = HashMap::new();
        Self { members, groups }
    }

    pub fn root_group(&self, member: &T) -> Option<&usize> {
        let group = self.members.get(member);
        match group {
            None => None,
            Some(group) => self.find_root_group(group),
        }
    }

    fn find_root_group(&self, group: &usize) -> Option<&usize> {
        let parent = self.groups.get(group);
        match parent {
            None => return None,
            Some(parent) => {
                if parent != group {
                    return self.find_root_group(parent);
                } else {
                    Some(parent)
                }
            }
        }
    }

    pub fn is_same(&self, x: &T, y: &T) -> bool {
        let rx = self.root_group(x);
        let ry = self.root_group(y);
        return rx == ry;
    }

    pub fn unite(&mut self, x: T, y: T) {
        let rx = self.root_group(&x);
        let ry = self.root_group(&y).cloned();
        if rx.is_none() && ry.is_none() {
            let new_group = self.groups.len() + 1;
            self.members.insert(x, new_group);
            self.members.insert(y, new_group);
            self.groups.insert(new_group, new_group);
            return;
        }
        if rx.is_none() {
            let ry = ry.unwrap();
            let ry = ry.clone();
            self.members.insert(x, ry);
            return;
        } else {
            let rx = rx.unwrap();
            let rx = rx.clone();
            if let Some(ry) = ry {
                self.groups.insert(ry.clone(), rx);
            }
            self.members.insert(y, rx);
            return;
        }
    }
}

fn main() {
    {
        let mut union_find = UnionFind::new();

        union_find.unite(1, 2);
        union_find.unite(1, 3);
        union_find.unite(4, 5);
        union_find.unite(2, 6);
        union_find.unite(7, 8);
        union_find.unite(7, 1);

        println!("{:?}", union_find.groups);
        println!("{:?}", union_find.members);

        assert_eq!(union_find.groups.len(), 3);

        assert_eq!(union_find.is_same(&1, &2), true);
        assert_eq!(union_find.is_same(&1, &3), true);
        assert_eq!(union_find.is_same(&2, &3), true);
        assert_eq!(union_find.is_same(&2, &6), true);
        assert_eq!(union_find.is_same(&1, &4), false);
    }

    {
        let mut union_find = UnionFind::new();

        union_find.unite("A", "B");
        union_find.unite("A", "C");
        union_find.unite("D", "E");
        union_find.unite("B", "F");
        union_find.unite("G", "H");
        union_find.unite("H", "A");

        println!("{:?}", union_find.groups);
        println!("{:?}", union_find.members);

        assert_eq!(union_find.groups.len(), 3);

        assert_eq!(union_find.is_same(&"A", &"B"), true);
        assert_eq!(union_find.is_same(&"A", &"C"), true);
        assert_eq!(union_find.is_same(&"B", &"C"), true);
        assert_eq!(union_find.is_same(&"B", &"F"), true);
        assert_eq!(union_find.is_same(&"A", &"D"), false);
    }
}
