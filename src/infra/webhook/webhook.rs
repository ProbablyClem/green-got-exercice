use crate::models::output_transaction::OutputTransaction;

pub trait Webhook {
    fn send(&self, output_transaction: OutputTransaction);
}