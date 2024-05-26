#![allow(dead_code)]

use crate::model::Batch;
use crate::model::OrderLine;
use anyhow::Result;
use chrono::Datelike;
use edgedb_protocol::model::LocalDate;
use edgedb_tokio::Client as EdgeDbConnection;

trait Repository {
    async fn add(&self, batch: &Batch) -> Result<()>;
    async fn add_orderline(&self, orderline: &OrderLine) -> Result<()>;
    async fn get(&self, reference: &str) -> Result<Batch>;
    async fn get_all(&self) -> Result<Vec<Batch>>;
    async fn allocate(&self, orderline: &OrderLine, batch: &Batch) -> Result<()>;
}

#[derive(Debug)]
struct EdgeDbRepository {
    conn: EdgeDbConnection,
}

impl EdgeDbRepository {
    const fn new(conn: EdgeDbConnection) -> Self {
        Self { conn }
    }
}

#[allow(clippy::cast_possible_truncation)]
impl Repository for EdgeDbRepository {
    async fn add(&self, batch: &Batch) -> Result<()> {
        let eta = batch
            .eta()
            .map(|eta| LocalDate::from_ymd(eta.year(), eta.month() as u8, eta.day() as u8));
        let query = if eta.is_some() {
            r"
            insert Batch {
                reference := <str>$0,
                sku := <str>$1,
                purchased_quantity := <int32>$2,
                eta := <cal::local_date>$3,
            }"
        } else {
            r"
            insert Batch {
                reference := <str>$0,
                sku := <str>$1,
                purchased_quantity := <int32>$2,
            }"
        };
        if eta.is_some() {
            self.conn
                .query_required_single_json(
                    query,
                    &(
                        batch.reference(),
                        batch.sku(),
                        batch.purchased_quantity(),
                        eta,
                    ),
                )
                .await?;
        } else {
            self.conn
                .query_required_single_json(
                    query,
                    &(batch.reference(), batch.sku(), batch.purchased_quantity()),
                )
                .await?;
        };

        for allocation in batch.allocations() {
            let query = r"
            update Batch
            filter .reference = <str>$0
            set {
                allocations += (select OrderLine filter .orderid = <str>$1),
            };
            ";
            self.conn
                .query_required_single_json(query, &(batch.reference(), allocation.orderid()))
                .await?;
        }
        Ok(())
    }

    async fn add_orderline(&self, orderline: &OrderLine) -> Result<()> {
        let query = r"
            insert OrderLine {
                orderid := <str>$0,
                sku := <str>$1,
                qty := <int32>$2,
            }
            ";
        self.conn
            .query_required_single_json(
                query,
                &(orderline.orderid(), orderline.sku(), orderline.qty()),
            )
            .await?;
        Ok(())
    }

    async fn get(&self, reference: &str) -> Result<Batch> {
        let query = r"
        select Batch {
            reference,
            sku,
            purchased_quantity,
            eta,
            allocations := (select OrderLine {
                orderid,
                sku,
                qty,
            }),
        } filter .reference = <str>$0
        ";
        let batch = self
            .conn
            .query_required_single_json(query, &(reference,))
            .await?;
        let batch = serde_json::from_str(&batch)?;
        Ok(batch)
    }

    async fn get_all(&self) -> Result<Vec<Batch>> {
        let query = r"
        select Batch {
            reference,
            sku,
            purchased_quantity,
            eta,
            allocations
        }
        ";
        let batches = self.conn.query_json(query, &()).await?;
        let batches = serde_json::from_str(&batches)?;
        Ok(batches)
    }

    async fn allocate(&self, orderline: &OrderLine, batch: &Batch) -> Result<()> {
        let query = r"
        update Batch
        filter .reference = <str>$0
        set {
            allocations += (select OrderLine filter .orderid = <str>$1),
        };
        ";
        self.conn
            .query_required_single_json(query, &(batch.reference(), orderline.orderid()))
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use crate::model::{allocate, Batch, OrderLine, Quantity, Reference, Sku};

    async fn createdb_config() -> edgedb_tokio::Config {
        edgedb_tokio::Builder::new()
            .instance("ddd-rust-edgedb-test")
            .unwrap()
            .build_env()
            .await
            .unwrap()
    }

    fn create_client(config: edgedb_tokio::Config) -> EdgeDbConnection {
        EdgeDbConnection::new(&config)
    }

    async fn reset_db(conn: &EdgeDbConnection) {
        conn.execute(r#"delete Batch"#, &()).await.unwrap();
        conn.execute(r#"delete OrderLine"#, &()).await.unwrap();
    }

    async fn insert_orderline(conn: &EdgeDbConnection, orderid: &str, qty: i32, sku: Option<&str>) {
        let query = if sku.is_some() {
            r#"
            insert OrderLine {
                orderid := <str>$0,
                sku := <str>$1,
                qty := <int32>$2,
            }
            "#
        } else {
            r#"
            insert OrderLine {
                orderid := <str>$0,
                qty := <int32>$1,
            }
            "#
        };
        if let Some(sku) = sku {
            conn.query_required_single_json(query, &(orderid, sku, qty))
                .await
                .unwrap();
        } else {
            conn.query_required_single_json(query, &(orderid, qty))
                .await
                .unwrap();
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_repository_can_save_a_batch() {
        let config = createdb_config().await;
        let conn = create_client(config);
        reset_db(&conn).await;

        let repo = EdgeDbRepository { conn };
        let batch = Batch::new(
            Reference("batch1".to_string()),
            Sku("sku1".to_string()),
            Quantity(100),
            None,
        );
        repo.add(&batch).await.unwrap();
        let saved_batch = repo.get(batch.reference()).await.unwrap();
        assert_eq!(batch, saved_batch);
    }

    #[tokio::test]
    #[serial]
    async fn test_repository_can_retrieve_all_batches() {
        let config = createdb_config().await;
        let conn = create_client(config);
        reset_db(&conn).await;

        let repo = EdgeDbRepository { conn };
        let batch1 = Batch::new(
            Reference("batch1".to_string()),
            Sku("sku1".to_string()),
            Quantity(100),
            None,
        );
        let batch2 = Batch::new(
            Reference("batch2".to_string()),
            Sku("sku2".to_string()),
            Quantity(200),
            None,
        );
        repo.add(&batch1).await.unwrap();
        repo.add(&batch2).await.unwrap();
        let batches = repo.get_all().await.unwrap();
        assert_eq!(vec![batch1, batch2], batches);
    }

    #[tokio::test]
    #[serial]
    async fn test_repository_can_retrieve_a_batch_by_reference() {
        let config = createdb_config().await;
        let conn = create_client(config);
        reset_db(&conn).await;

        let repo = EdgeDbRepository { conn };
        let batch1 = Batch::new(
            Reference("batch1".to_string()),
            Sku("sku1".to_string()),
            Quantity(100),
            None,
        );
        let batch2 = Batch::new(
            Reference("batch2".to_string()),
            Sku("sku2".to_string()),
            Quantity(200),
            None,
        );
        repo.add(&batch1).await.unwrap();
        repo.add(&batch2).await.unwrap();
        let retrieved_batch = repo.get(&batch1.reference()).await.unwrap();
        assert_eq!(batch1, retrieved_batch);
    }

    #[tokio::test]
    #[serial]
    async fn test_repository_can_allocate_to_a_batch() {
        let config = createdb_config().await;
        let conn = create_client(config);
        reset_db(&conn).await;

        let repo = EdgeDbRepository { conn };
        let batch = Batch::new(
            Reference("batch1".to_string()),
            Sku("sku1".to_string()),
            Quantity(100),
            None,
        );
        let batch_ref = batch.reference();
        let line = OrderLine::new("order1".to_string(), Sku("sku1".to_string()), Quantity(10));
        let mut batches = vec![batch.clone()];
        allocate(&line, &mut batches).expect("allocation failed!");
        repo.add(&batches[0]).await.unwrap();
        repo.add_orderline(&line).await.unwrap();
        repo.allocate(&line, &batch).await.unwrap();
        let retrieved_batch = repo.get(batch_ref).await.unwrap();
        assert_eq!(retrieved_batch.available_quantity(), 90);
        assert_eq!(retrieved_batch.allocations().collect::<Vec<_>>().len(), 1);
    }
}
