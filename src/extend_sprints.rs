use goji::Sprint;

pub type Sprints = Vec<Sprint>;

pub trait SprintsPadding {
    fn id_padding(&self) -> usize;
}

impl SprintsPadding for Sprints {
    fn id_padding(&self) -> usize {
        self.iter()
            .map(|i| i.id.to_string().len())
            .max()
            .unwrap_or(0)
    }
}
