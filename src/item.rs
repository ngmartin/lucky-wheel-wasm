pub struct Item {
    pub color: String,
    pub label: String,
    pub start_degree: f64,
    pub end_degree: f64,
}

pub struct Items {
    pub items: Vec<Item>,
}

impl Items {
    pub fn new() -> Items {
        let config_items = vec![
            Item {
                color: String::from("#f82"),
                label: String::from("test1"),
                start_degree: 0.0,
                end_degree: 0.0,
            },
            Item {
                color: String::from("#0bf"),
                label: String::from("test2"),
                start_degree: 0.0,
                end_degree: 0.0,
            },
            Item {
                color: String::from("#fb0"),
                label: String::from("test3"),
                start_degree: 0.0,
                end_degree: 0.0,
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
                    color: el.color.clone(),
                    label: el.label.clone(),
                    start_degree,
                    end_degree,
                }
            })
            .collect();

        Items { items }
    }
}
