pub mod combinatorics {
    use std::cmp::{max, min};
    use std::fmt::Debug;
    use std::iter::Sum;
    use std::ops::{Add, Range, Sub};

    pub struct RangeSet<Idx> where Idx: Ord + Copy {
        pub data: Vec<Range<Idx>>
    }

    impl <Idx: Ord + Copy + Debug> RangeSet<Idx> {
        pub fn new() -> RangeSet<Idx> {
            RangeSet { data: Vec::new() }
        }

        pub fn from_vec(vec: Vec<Range<Idx>>) -> RangeSet<Idx> {
            let mut set = RangeSet::new();
            for range in vec.into_iter() {
                set.add(range);
            }

            set
        }

        pub fn contains(&self, val: Idx) -> bool {
            if self.data.is_empty() { return false; }

            let idx = self.find(&(val..val));
            if idx >= self.data.len() {
                self.data[idx-1].contains(&val)
            } else if idx > 0 {
                self.data[idx].contains(&val) || self.data[idx-1].contains(&val)
            } else {
                self.data[idx].contains(&val)
            }
        }

        pub fn add(&mut self, range: Range<Idx>) {
            if self.data.is_empty() {
                self.data.push(range);
                return;
            }

            let mut idx = self.find(&range);
            self.data.insert(idx, range);
            while idx < self.data.len() - 1 && self.data[idx].end >= self.data[idx + 1].start {
                self.data[idx].start = min(self.data[idx + 1].start, self.data[idx].start);
                self.data[idx].end = max(self.data[idx + 1].end, self.data[idx].end);
                self.data.remove(idx + 1);
            }
            while idx > 0 && self.data[idx].start <= self.data[idx - 1].end {
                self.data[idx].start = min(self.data[idx - 1].start, self.data[idx].start);
                self.data[idx].end = max(self.data[idx - 1].end, self.data[idx].end);
                self.data.remove(idx - 1);
                idx -= 1;
            }
        }

        pub fn find(&self, range: &Range<Idx>) -> usize {
            if self.data.is_empty() { return 0 }

            let mut low = 0;
            let mut high = self.data.len() - 1;
            while low < high {
                let halfway = (low + high) / 2;
                if self.data[halfway].start == range.start {
                    return halfway;
                }
                if self.data[halfway].start < range.start {
                    low = halfway + 1;
                } else {
                    high = if halfway == 0 { 0 } else { halfway - 1 };
                }
            }

            if self.data[low].start < range.start { low + 1 } else { low }
        }
    }

    impl<Idx: Ord + Copy + Sum + Sub<Output=Idx>> RangeSet<Idx> {
        pub fn len(&self) -> Idx {
            self.data.iter().map(|r| r.end - r.start).sum()
        }
    }

    #[test]
    fn test_add() {
        let mut set = RangeSet { data: vec![] };
        set.add(1..3);
        assert_eq!(set.data, vec![1..3]);

        set.add(20..30);
        assert_eq!(set.data, vec![1..3, 20..30]);
        set.add(6..12);
        assert_eq!(set.data, vec![1..3, 6..12, 20..30]);
        set.add(34..40);
        assert_eq!(set.data, vec![1..3, 6..12, 20..30, 34..40]);

        set.add(12..13);
        assert_eq!(set.data, vec![1..3, 6..13, 20..30, 34..40]);

        set.add(19..21);
        assert_eq!(set.data, vec![1..3, 6..13, 19..30, 34..40]);

        set.add(2..46);
        assert_eq!(set.data, vec![1..46]);
    }

    #[test]
    fn test_find() {
        let set = RangeSet { data: vec![1..3, 3..6, 10..12]};
        assert_eq!(set.find(&(1..2)), 0);
        assert_eq!(set.find(&(2..10)), 1);
        assert_eq!(set.find(&(0..0)), 0);
        assert_eq!(set.find(&(15..20)), 3);
        assert_eq!(set.find(&(11..20)), 3);
        assert_eq!(set.find(&(3..12)), 1);
        assert_eq!(set.find(&(7..9)), 2);
    }

    #[test]
    fn test_contains() {
        let set = RangeSet { data: vec![1..3, 3..6, 10..12]};
        assert!(!set.contains(0));
        assert!( set.contains(1));
        assert!( set.contains(2));
        assert!( set.contains(3));
        assert!( set.contains(5));
        assert!(!set.contains(6));
        assert!(!set.contains(7));
        assert!( set.contains(10));
        assert!( set.contains(11));
        assert!(!set.contains(12));
    }
}