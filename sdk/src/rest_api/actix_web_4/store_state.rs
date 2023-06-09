// Copyright 2018-2022 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::store::TransactionalStoreFactory;
#[cfg(any(
    feature = "batch-store",
    feature = "location",
    feature = "pike",
    feature = "product",
    feature = "purchase-order",
    feature = "schema",
    feature = "track-and-trace",
))]
use std::sync::Arc;

#[cfg(feature = "diesel")]
use diesel::r2d2::{ConnectionManager, Pool};

#[cfg(feature = "postgres")]
use crate::store::postgres::PgStoreFactory;
#[cfg(feature = "sqlite")]
use crate::store::sqlite::SqliteStoreFactory;

#[derive(Clone)]
pub struct StoreState {
    pub store_factory: Arc<dyn TransactionalStoreFactory>,
}

#[allow(clippy::redundant_clone)]
#[allow(unused_variables)]
#[cfg(feature = "postgres")]
impl StoreState {
    pub fn with_pg_pool(
        connection_pool: Pool<ConnectionManager<diesel::pg::PgConnection>>,
    ) -> Self {
        Self {
            store_factory: Arc::new(PgStoreFactory::new(connection_pool)),
        }
    }

    #[cfg(feature = "sqlite")]
    pub fn with_sqlite_pool(
        connection_pool: Pool<ConnectionManager<diesel::sqlite::SqliteConnection>>,
    ) -> Self {
        Self {
            store_factory: Arc::new(SqliteStoreFactory::new(connection_pool)),
        }
    }
}
