use std::{collections::{HashMap, HashSet, BinaryHeap}, hash::{Hash, Hasher}, cmp};
use serde::{Deserialize, Serialize};

use crate::solution::Solution;

pub type OrderId = usize;
pub type WaveId = usize;
pub type BatchId = usize;
pub type ArticleId = usize;
pub type Warehouse = usize;
pub type Aisle = usize;
pub type Position = usize;
pub type Size = usize;
pub type Volume = usize;
pub type Cost = usize;

pub type ArticleLocations = HashMap<ArticleId, ArticleLocation>;
pub type ArticleVolumes = HashMap<ArticleId, Volume>;
pub type Orders = HashMap<OrderId, Order>;

#[derive(Clone, Deserialize, Debug)]
pub struct ArticleLocation {
    #[serde(alias = "Warehouse")]
    pub warehouse: Warehouse,

    #[serde(alias = "Aisle")]
    pub aisle: Aisle, 

    #[serde(alias = "Position")]
    pub position: Position, 

    #[serde(alias = "ArticleId")]
    pub article_id: ArticleId, 
}

#[derive(Clone, Deserialize, Debug)]
pub struct Order {
    #[serde(alias = "OrderId")]
    pub id: OrderId,

    #[serde(alias = "ArticleIds")]
    pub article_ids: Vec<ArticleId>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Article {
    #[serde(alias = "ArticleId")]
    pub id: ArticleId,

    #[serde(alias = "Volume")]
    pub volume: Volume,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct Wave {
    #[serde(rename = "WaveId")]
    pub id: WaveId,

    #[serde(rename = "BatchIds")]
    pub batch_ids: Vec<BatchId>,

    #[serde(rename = "OrderIds")]
    pub order_ids: Vec<OrderId>,

    #[serde(rename = "WaveSize")]
    pub size: Size,
}

impl Hash for Wave {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        for id in self.batch_ids.iter() {
            id.hash(state);
        }
    }
}

impl PartialEq for Wave {
    fn eq(&self, other: &Wave) -> bool {
        self.id == other.id
    }
}

impl Wave {
    pub fn new(id: WaveId) -> Self {
        Wave { 
            id: id, 
            batch_ids: Vec::new(), 
            order_ids: Vec::new(), 
            size: 0
        }
    }

    pub fn push_order(&mut self, order: &Order) {
        self.order_ids.push(order.id);
        self.size = self.size + order.article_ids.len();
    }
}

impl From<(&Solution, &Batch)> for Wave {
    fn from((solution, batch): (&Solution, &Batch)) -> Wave {
        Wave { 
            id: solution.next_wave_id(),
            batch_ids: vec![batch.id], 
            order_ids: batch.get_order_ids(), 
            size: 1
        }
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, Hash)]
pub struct Item {
    #[serde(rename = "OrderId")]
    pub order_id: OrderId,

    #[serde(rename = "ArticleId")]
    pub article_id: ArticleId,
}

#[derive(Clone, Serialize, Debug)]
pub struct Batch {
    #[serde(rename = "BatchId")]
    pub id: BatchId,

    #[serde(rename = "Items")]
    pub items: Vec<Item>,

    #[serde(rename = "BatchVolume")]
    pub volume: Volume,

    #[serde(skip_serializing)]
    pub warehouse_aisles: HashMap<Warehouse, HashSet<Aisle>>,
}

impl PartialEq for Batch {
    fn eq(&self, other: &Batch) -> bool {
        self.id == other.id
    }
}

impl Hash for Batch {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.items.hash(state);
    }
}

impl Batch {
    pub fn new(id: BatchId) -> Self {
        Batch { 
            id: id, 
            items: vec![], 
            volume: 0, 
            warehouse_aisles: HashMap::new(),
        }
    }

    /// Push an order to the batch
    pub fn push(&mut self, 
        order_id: OrderId, 
        article_id: ArticleId,
        volume: Volume
    ) {
        self.items.push(Item { order_id, article_id });
        self.volume = self.volume + volume;
    }

    pub fn get_order_ids(&self) -> Vec<OrderId> {
        self.items.iter().map(|item| item.order_id).collect()
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Input {
    #[serde(alias = "ArticleLocations")]
    pub article_locations: Vec<ArticleLocation>,

    #[serde(alias = "Articles")]
    pub articles: Vec<Article>,

    #[serde(alias = "Orders")]
    pub orders: Vec<Order>,
}

#[derive(Clone, Debug)]
pub struct Model {
    pub article_locations: ArticleLocations,

    pub article_volumes: ArticleVolumes,

    pub orders: Vec<Order>,

    /// List of warehouse => \[order\] mappings
    /// 
    /// The outer vector index indictates the sorting priority.
    /// The mappings indicate that for a given warehouse(-id)
    /// there exists orders, that contain mostly articles from that warehouse.
    pub warehouse_orders: Vec<HashMap<Warehouse, Vec<OrderId>>>
}

impl From<Input> for Model {
    fn from(input: Input) -> Self {
        let article_locations: ArticleLocations = input.article_locations
                .iter()
                .map(|al| (al.article_id, al.clone()))
                .collect();

        let wo: HashMap<Warehouse, Vec<OrderId>> = input.article_locations
            .iter()
            .map(|article| {
                (article.warehouse, vec![])
            })
            .collect();

        let mut warehouse_orders: Vec<HashMap<Warehouse, Vec<OrderId>>> = vec![
            wo.clone(),
            wo.clone(),
            wo,
        ];

        let order_warehouses: Vec<Vec<(i16, Warehouse)>> = input.orders
            .iter()
            .map(|order| {
                let mut map: HashMap<Warehouse, i16> = HashMap::new();

                for article_id in order.article_ids.iter() {
                    let warehouse = article_locations.get(article_id).unwrap().warehouse;
                    if let Some(s) = map.get_mut(&warehouse) {
                        *s = *s + 1;
                    } else {
                        map.insert(warehouse, 1);
                    }
                }

                let heap = BinaryHeap::from(
                    map
                        .into_iter()
                        .map(|(warehouse, n)| (n * -1, warehouse))
                        .collect::<Vec<(i16, Warehouse)>>()
                );
                let range_limit = cmp::min(3, heap.clone().into_vec().len());

                heap.into_sorted_vec()[0..range_limit]
                    .into_iter()
                    .map(|(n, w)| (n * -1, *w))
                    .collect::<Vec<(i16, Warehouse)>>()

            })
            .collect();
        
        
        for (order_id, warehouses) in order_warehouses.iter().enumerate() {
            for (priority, (_, warehouse)) in warehouses.iter().enumerate() {
                warehouse_orders
                    .get_mut(priority)
                    .unwrap()
                    .get_mut(warehouse)
                    .unwrap()
                    .push(order_id);
            }
        }

        Model { 
            article_locations,
            article_volumes: input.articles
                .into_iter()
                .map(|article| (article.id, article.volume))
                .collect(), 
            orders: input.orders,
            warehouse_orders
        }
    }
}