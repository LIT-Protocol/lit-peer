use ethers::types::{Address, I256, TxHash};
use std::collections::HashMap;
use tokio::sync::Mutex;

const MAX_BATCH_SIZE: usize = 50;
const MAX_PAYMENT_TRIAL: u8 = 3;

#[derive(Clone, Debug)]
pub struct PendingPayment {
    pub payer: Address,
    pub price: I256,
    pub spending_limit: I256,
}

#[derive(Default, Debug)]
pub struct Batch {
    batch_id: u64,
    payments: Vec<PendingPayment>,
    spending_per_payer: HashMap<Address, I256>,
    tx_hash: Option<TxHash>,
    submission_counter: u8,
}

impl Batch {
    pub fn new_empty(id: u64) -> Batch {
        Batch {
            batch_id: id,
            payments: vec![],
            spending_per_payer: HashMap::new(),
            tx_hash: None,
            submission_counter: 0,
        }
    }

    pub fn new(pp: PendingPayment, id: u64) -> Batch {
        let mut spending_per_payer = HashMap::new();
        spending_per_payer.insert(pp.payer, pp.price);
        Batch {
            batch_id: id,
            payments: vec![pp],
            spending_per_payer,
            tx_hash: None,
            submission_counter: 0,
        }
    }

    pub fn id(&self) -> u64 {
        self.batch_id
    }

    pub fn is_empty(&self) -> bool {
        self.payments.len() == 0
    }

    pub fn is_full(&self) -> bool {
        self.payments.len() >= MAX_BATCH_SIZE
    }

    pub fn add(&mut self, pp: PendingPayment) {
        self.spending_per_payer
            .entry(pp.payer)
            .and_modify(|price| *price += pp.price)
            .or_insert(pp.price);
        self.payments.push(pp);
    }

    pub fn into_vecs(&self) -> (Vec<Address>, Vec<I256>) {
        let mut addresses = vec![];
        let mut prices = vec![];
        self.payments.iter().for_each(|pp| {
            addresses.push(pp.payer);
            prices.push(pp.price);
        });
        (addresses, prices)
    }

    pub fn get_spending(&self, payer: &Address) -> I256 {
        *self.spending_per_payer.get(payer).unwrap_or(&I256::zero())
    }

    pub fn set_tx_hash_and_increment_counter(&mut self, tx_hash: Option<TxHash>) {
        self.tx_hash = tx_hash;
        self.submission_counter += 1;
    }

    pub fn get_tx_hash(&self) -> Option<TxHash> {
        self.tx_hash
    }

    pub fn increment_counter(&mut self) {
        self.submission_counter += 1;
    }

    pub fn max_trial_count_exceeded(&self) -> bool {
        self.submission_counter >= MAX_PAYMENT_TRIAL
    }
}

#[derive(Default)]
pub struct Batches {
    batches: Mutex<Vec<Batch>>,
}

impl Batches {
    pub async fn add(&self, pp: PendingPayment) {
        let mut batches = self.batches.lock().await;
        match batches.last_mut() {
            Some(batch) if !batch.is_full() => batch.add(pp),
            Some(batch) => {
                let id = batch.batch_id + 1;
                batches.push(Batch::new(pp, id));
            }
            None => batches.push(Batch::new(pp, 0)),
        }
    }

    pub async fn take_batches_for_payment(&self) -> Vec<Batch> {
        let mut batches = self.batches.lock().await;
        let old_batches = batches.drain(..).collect::<Vec<_>>();
        let next_batch_id = match old_batches.last() {
            Some(batch) => batch.batch_id + 1,
            None => 0,
        };
        *batches = vec![Batch::new_empty(next_batch_id)];

        for batch in old_batches.iter() {
            trace!("Finalized Batch {}: {:?}", batch.id(), batch);
        }

        old_batches
    }

    pub async fn get_unregistered_spending(&self, payer: &Address) -> I256 {
        let batches = self.batches.lock().await;
        batches.iter().map(|b| b.get_spending(payer)).sum()
    }
}

#[cfg(test)]
mod test {
    use crate::payment::batches::{Batches, MAX_BATCH_SIZE, PendingPayment};
    use ethers::types::{H160, I256};

    #[tokio::test]
    async fn test_unregistered_spending() {
        let batches = Batches::default();
        let address = H160::random();
        let other_address = H160::random();

        // Empty batch returns 0.
        assert_eq!(
            batches.get_unregistered_spending(&address).await,
            I256::zero()
        );

        let others_payment = PendingPayment {
            payer: other_address,
            price: I256::from(10000),
            spending_limit: I256::from(10000),
        };

        batches.add(others_payment.clone()).await;
        batches.add(others_payment.clone()).await;

        // A batch with no entry for this payer returns 0.
        assert_eq!(
            batches.get_unregistered_spending(&address).await,
            I256::zero()
        );

        let payment_1 = PendingPayment {
            payer: address,
            price: I256::from(1000),
            spending_limit: I256::from(1000),
        };

        let payment_2 = PendingPayment {
            payer: address,
            price: I256::from(5000),
            spending_limit: I256::from(1000),
        };

        batches.add(payment_1.clone()).await;
        batches.add(payment_2.clone()).await;

        // The patch returns the sum of all entries in this batch.
        assert_eq!(
            batches.get_unregistered_spending(&address).await,
            payment_1.price + payment_2.price
        );

        // Make sure a new batch gets started.
        for _ in 0..MAX_BATCH_SIZE {
            batches.add(others_payment.clone()).await;
        }

        let payment_3 = PendingPayment {
            payer: address,
            price: I256::from(3000),
            spending_limit: I256::from(1000),
        };

        let payment_4 = PendingPayment {
            payer: address,
            price: I256::from(1500),
            spending_limit: I256::from(1500),
        };

        batches.add(payment_3.clone()).await;
        batches.add(payment_4.clone()).await;

        // The patch returns the sum of all entries in all batches.
        assert_eq!(
            batches.get_unregistered_spending(&address).await,
            payment_1.price + payment_2.price + payment_3.price + payment_4.price
        );
    }
}
