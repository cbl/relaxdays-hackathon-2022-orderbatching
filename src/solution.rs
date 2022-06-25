use std::{collections::{HashSet, HashMap}, hash::{Hash, Hasher, self}};
use serde::{Serialize, Deserialize};
use fxhash::{hash32, FxBuildHasher};
use linked_hash_set::LinkedHashSet;

use crate::{model::{Wave, Batch, Model, Cost, Warehouse, BatchId, ArticleId, OrderId, WaveId, Order, Aisle}, config::{MAX_BATCH_VOLUME, MAX_WAVE_SIZE}};

#[derive(Clone, Serialize, Debug)]
pub struct Solution {
    #[serde(rename = "Waves")]
    pub waves: Vec<Wave>,

    #[serde(rename = "Batches")]
    pub batches: Vec<Batch>,

    #[serde(skip_serializing)]
    pub model: Model,

    #[serde(skip_serializing)]
    pub wave_warehouse_batch: HashMap<WaveId, HashMap<Warehouse, BatchId>>,

    #[serde(skip_serializing)]
    pub warehouse_wave: HashMap<Warehouse, WaveId>
}

impl PartialEq for Solution {
    fn eq(&self, other: &Solution) -> bool {
        self.waves == other.waves && self.batches == other.batches
    }
}

impl Eq for Solution {}

impl Hash for Solution {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.waves.hash(state);
        self.batches.hash(state);
    }
}

impl From<Model> for Solution {
    /// Converts a Solution type from the given Model.
    fn from(model: Model) -> Solution {
        Solution {
            waves: Vec::new(),
            batches: Vec::new(),
            model,
            wave_warehouse_batch: HashMap::default(),
            warehouse_wave: HashMap::default(),
        }
    }
}

impl Solution {

    pub fn tour_cost(&self) -> Cost {
        let a: Cost = self.batches
            .iter()
            .map(|batch| {
                batch.warehouse_aisles
                    .iter()
                    .map(|(warehouse, _)| *warehouse)
                    .collect::<HashSet<Warehouse>>()
                    .len() * 10
            })
            .sum();

        let b: Cost = self.batches
            .iter()
            .map(|batch| {
                batch.warehouse_aisles
                    .iter()
                    .map(|(_, aisles)| aisles.iter().count())
                    .sum::<Cost>() * 5
            })
            .sum();

        a + b
    }

    pub fn rest_cost(&self) -> Cost {
        self.waves.len() * 10 + self.batches.len() * 5
    }

    pub fn total_cost(&self) -> Cost {
        self.tour_cost() + self.rest_cost()
    }

    pub fn next_wave_id(&self) -> WaveId {
        self.waves.len()
    }

    pub fn get_possible_batch_waves(&self, batch: &BatchId) -> Vec<Option<&Wave>> {
        Vec::new()
    }

    pub fn get_possible_article_batches(&self) -> Vec<Option<&Batch>> {
        let mut batches = self.batches
            .iter()
            .map(|batch| Some(batch))
            .collect::<Vec<Option<&Batch>>>();

        batches.push(None);

        batches
    }

    fn push_to_wave_or_create(&mut self, batch: Option<&Batch>) {
        // if let Some(b) = batch {
        //     let wave = Wave::from((self, b));
        // }
    }

    fn new_wave() {

    }

    pub fn get_batch_wave(&self, batch: &Batch) -> Option<&Wave> {
        // find wave for batch if not exists return None
        None
    }

    pub fn create_wave_for_batch(&mut self, batch: &Batch) {
        self.waves.push(self.new_wave_for_batch(batch))
    }

    fn new_wave_for_batch(&self, batch: &Batch) -> Wave {
        Wave::from((self, batch))
    }

    pub fn make_batch_for(&mut self, order_id: OrderId, article_id: ArticleId, wave_id: &WaveId) {
        let article_location = self.model.article_locations
            .get(&article_id)
            .unwrap();

        let warehouse: Warehouse = article_location.warehouse;
        let aisle: Aisle = article_location.aisle;
        let volume = self.model.article_volumes.get(&article_id).unwrap();

        let mut batch_id = self.batches.len();

        // when no batch exists, create new one
        if(batch_id == 0) {
            self.batches.push(Batch::new(0));
            self.waves.get_mut(*wave_id).unwrap().batch_ids.push(batch_id);
        } else if let Some(bid) = self.wave_warehouse_batch.get(wave_id).unwrap().get(&warehouse) {
            if (self.batches.get(*bid).unwrap().volume + volume) > MAX_BATCH_VOLUME {
                self.batches.push(Batch::new(batch_id));
                self.waves.get_mut(*wave_id).unwrap().batch_ids.push(batch_id);
            } else {
                batch_id = *bid;
            }
        } else {
            self.batches.push(Batch::new(batch_id));
            self.waves.get_mut(*wave_id).unwrap().batch_ids.push(batch_id);
        }

        self.batches.get_mut(batch_id).unwrap().push(
            order_id, article_id,
            article_location.warehouse, 
            article_location.aisle,
            *volume
        );
        self.wave_warehouse_batch
            .get_mut(wave_id)
            .unwrap()
            .insert(warehouse, batch_id);
    }

    pub fn make_wave_for(&mut self, order_id: OrderId, warehouse: Warehouse) -> bool {
        let mut wave_id = self.waves.len();
        let order = self.model.orders.get(order_id).unwrap();

        if let Some(wid) = self.warehouse_wave.get(&warehouse) {
            let wave = self.waves.get_mut(*wid).unwrap();

            if (wave.size + order.article_ids.len()) > MAX_WAVE_SIZE {
                self.waves.push(Wave::new(wave_id));
                self.wave_warehouse_batch.insert(wave_id, HashMap::default());
                self.waves.get_mut(wave_id).unwrap().push_order(order);
                self.warehouse_wave.insert(warehouse, wave_id);
            } else {
                wave.push_order(order);
            }

            return true;
        } else {
            self.waves.push(Wave::new(wave_id));
            self.wave_warehouse_batch.insert(wave_id, HashMap::default());
            self.waves.get_mut(wave_id).unwrap().push_order(order);
            self.warehouse_wave.insert(warehouse, wave_id);

            return true;
        }

        return false;
    }

}

pub fn search(solution: &mut Solution, model: &Model) {
    // let tabu: LinkedHashSet<u32, FxBuildHasher> = LinkedHashSet::default();

    // Step 1: Artikel in batches aufteilen
    

    // Step 1: Orders in waves aufteilen

    // 1. Orders nach warehouses sortieren
    // 2. Orders auf waves aufteilen bis sie voll sind
    // 3. Batches für die orders finden

    // let mut missing_orders: Vec<OrderId> = vec![];

    model.warehouse_orders
        .get(0)
        .unwrap()
        .iter()
        .for_each(|(warehouse, order_ids)| {
            for order_id in order_ids {
                if !solution.make_wave_for(*order_id, *warehouse) {
                    // missing_orders.push(*order_id);
                }
            }
        });

    solution.waves
        .clone()
        .iter()
        .for_each(|wave| {
            for order_id in wave.order_ids.iter() {
                for article_id in model.orders.get(*order_id).unwrap().article_ids.iter() {
                    solution.make_batch_for(
                        *order_id,
                        *article_id,
                        &wave.id
                    );
                }
            }
        });

    // println!(
    //     "{:?}", 
    //     solution.waves
    //         .iter()
    //         .map(|wave| wave.size)
    //         .filter(|size| *size < 230)
    //         .collect::<Vec<usize>>()
    // );
    
}