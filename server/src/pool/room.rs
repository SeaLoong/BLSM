use std::cmp::Ordering;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Room {
    pub id: String,
    pub weight: i32,
}

impl PartialOrd for Room {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.weight
                .partial_cmp(&other.weight)?
                .then(other.id.partial_cmp(&self.id)?),
        )
    }
}

impl Ord for Room {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight).then(self.id.cmp(&other.id))
    }
}
