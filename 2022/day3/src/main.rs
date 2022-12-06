use std::fs::read;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Item(u8);

impl From<u8> for Item {
    fn from(byte: u8) -> Self {
        if byte >= b'a' && byte <= b'z' {
            return Item(byte - b'a' + 1);
        }

        if byte >= b'A' && byte <= b'Z' {
            return Item(byte - b'A' + 27);
        }

        panic!("Unknown item");
    }
}

impl Into<u8> for &Item {
    fn into(self) -> u8 {
        if self.0 >= 1 && self.0 <= 26 {
            return self.0 + b'a' - 1;
        }
        if self.0 >= 27 && self.0 <= 52 {
            return self.0 + b'A' - 27;
        }

        return b'?';
    }
}

impl Item {
    fn value(&self) -> u8 {
        self.0
    }
}

impl std::fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: u8 = self.into();

        f.debug_tuple("Item")
            .field(&(c as char))
            .field(&self.0)
            .finish()
    }
}

struct ItemSet(u64);

impl ItemSet {
    fn new() -> Self {
        ItemSet(0)
    }

    fn insert(&mut self, value: impl Into<Item>) {
        let item: Item = value.into();
        self.0 |= 1 << item.0;
    }

    fn contains(&self, value: impl Into<Item>) -> bool {
        let item: Item = value.into();
        (self.0 & 1 << item.0) != 0
    }

    fn remove(&mut self, value: impl Into<Item>) {
        let item: Item = value.into();
        self.0 &= !(1 << item.0);
    }

    fn is_empty(&self) -> bool {
        self.0 == 0
    }

    fn peek(&self) -> Item {
        Item(self.0.trailing_zeros() as u8)
    }
}

impl IntoIterator for ItemSet {
    type Item = Item;
    type IntoIter = ItemSetIter;

    fn into_iter(self) -> Self::IntoIter {
        ItemSetIter(self)
    }
}

struct ItemSetIter(ItemSet);

impl Iterator for ItemSetIter {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let ItemSetIter(item_set) = self;

        if item_set.is_empty() {
            return None;
        }

        let item = item_set.peek();
        item_set.remove(item);
        Some(item)
    }
}

fn items<'a>(bytes: &'a [u8]) -> impl Iterator<Item = Item> + 'a {
    bytes.iter().copied().map(Into::<Item>::into)
}

fn main() {
    let result = read("./input.txt")
        .unwrap()
        .split(|byte| *byte == b'\n')
        .filter(|rucksacks| rucksacks.len() != 0)
        .map(|rucksacks| {
            let (left, right) = rucksacks.split_at(rucksacks.len() / 2);
            debug_assert_eq!(left.len(), right.len());

            let mut left_set = ItemSet::new();

            for &item in left {
                left_set.insert(item);
            }

            let mut duplicates = ItemSet::new();

            items(right)
                .filter(|item| left_set.contains(*item))
                .for_each(|item| duplicates.insert(item));

            duplicates
                .into_iter()
                .map(|item| item.value() as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_values() {
        assert_eq!(Item::from(b'a').value(), 1);
        assert_eq!(Item::from(b'z').value(), 26);
        assert_eq!(Item::from(b'A').value(), 27);
        assert_eq!(Item::from(b'Z').value(), 52);
    }

    #[test]
    fn item_set() {
        let mut set = ItemSet::new();

        assert_eq!(set.is_empty(), true);

        assert_eq!(set.contains(b'b'), false);

        set.insert(b'b');

        assert_eq!(set.is_empty(), false);
        assert_eq!(set.peek(), Item::from(b'b'));

        assert_eq!(set.contains(b'a'), false);
        assert_eq!(set.contains(b'b'), true);
        assert_eq!(set.contains(b'c'), false);

        set.insert(b'C');

        assert_eq!(set.contains(b'B'), false);
        assert_eq!(set.contains(b'C'), true);
        assert_eq!(set.contains(b'D'), false);

        set.remove(b'C');

        assert_eq!(set.contains(b'b'), true);
        assert_eq!(set.contains(b'C'), false);
    }
}
