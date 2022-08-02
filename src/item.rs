pub struct Item {
    pub id: u32,
    pub color: String,
    pub label: String,
    pub start_degree: f64,
    pub end_degree: f64,
    pub weight: i32,
}

pub struct Items {
    pub items: Vec<Item>,
}

impl Items {
    pub fn new() -> Items {
        let config_items = vec![
            Item {
                id: 0,
                color: String::from("#f82"),
                label: String::from("test1"),
                start_degree: 0.0,
                end_degree: 0.0,
                weight: 1,
            },
            Item {
                id: 1,
                color: String::from("#0bf"),
                label: String::from("test2"),
                start_degree: 0.0,
                end_degree: 0.0,
                weight: 1,
            },
            Item {
                id: 2,
                color: String::from("#fb0"),
                label: String::from("test3"),
                start_degree: 0.0,
                end_degree: 0.0,
                weight: 1,
            },
        ];

        let delta = 360.0 / config_items.len() as f64;
        let mut index = 0;

        let items = config_items
            .iter()
            .map(|el| {
                let start_degree = delta * index as f64;
                let end_degree = start_degree + delta;
                index += 1;

                Item {
                    id: el.id,
                    color: el.color.clone(),
                    label: el.label.clone(),
                    start_degree,
                    end_degree,
                    weight: el.weight,
                }
            })
            .collect();

        Items { items }
    }

    pub fn get_stoped_offset_degree(&self, id: u32) -> Result<f64, ()> {
        let item = self.items.iter().find(|&el| el.id == id);

        if let Some(item) = item {
            let avg_degree = (item.start_degree + item.end_degree) / 2.0;
            return Ok(360.0 - avg_degree);
        }

        Err(())
    }
}
