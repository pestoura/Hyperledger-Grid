// Copyright 2018-2021 Cargill Incorporated
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

pub mod models;
mod operations;
pub(crate) mod schema;

use diesel::connection::AnsiTransactionManager;
use diesel::r2d2::{ConnectionManager, Pool};

use super::diesel::models::{
    LocationAttributeModel, LocationModel, NewLocationAttributeModel, NewLocationModel,
};
use super::{
    LatLongValue, Location, LocationAttribute, LocationList, LocationStore, LocationStoreError,
};
use crate::commits::MAX_COMMIT_NUM;
use crate::error::ResourceTemporarilyUnavailableError;

use operations::add_location::LocationStoreAddLocationOperation as _;
use operations::delete_location::LocationStoreDeleteLocationOperation as _;
use operations::get_location::LocationStoreGetLocationOperation as _;
use operations::list_locations::LocationStoreListLocationsOperation as _;
use operations::LocationStoreOperations;

/// Manages creating organizations in the database
#[derive(Clone)]
pub struct DieselLocationStore<C: diesel::Connection + 'static> {
    connection_pool: Pool<ConnectionManager<C>>,
}

impl<C: diesel::Connection> DieselLocationStore<C> {
    /// Creates a new DieselLocationStore
    ///
    /// # Arguments
    ///
    ///  * `connection_pool`: connection pool to the database
    pub fn new(connection_pool: Pool<ConnectionManager<C>>) -> Self {
        DieselLocationStore { connection_pool }
    }
}

#[cfg(feature = "postgres")]
impl LocationStore for DieselLocationStore<diesel::pg::PgConnection> {
    fn add_location(&self, location: Location) -> Result<(), LocationStoreError> {
        let attributes = make_location_attribute_models(&location.attributes, None);
        let current_commit_num = location.start_commit_num;
        LocationStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            LocationStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .add_location(location.into(), attributes, current_commit_num)
    }

    fn get_location(
        &self,
        location_id: &str,
        service_id: Option<&str>,
    ) -> Result<Option<Location>, LocationStoreError> {
        LocationStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            LocationStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .get_location(location_id, service_id)
    }

    fn list_locations(
        &self,
        service_id: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<LocationList, LocationStoreError> {
        LocationStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            LocationStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .list_locations(service_id, offset, limit)
    }

    fn delete_location(
        &self,
        address: &str,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError> {
        LocationStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            LocationStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .delete_location(address, current_commit_num)
    }
}

#[cfg(feature = "sqlite")]
impl LocationStore for DieselLocationStore<diesel::sqlite::SqliteConnection> {
    fn add_location(&self, location: Location) -> Result<(), LocationStoreError> {
        let attributes = make_location_attribute_models(&location.attributes, None);
        let current_commit_num = location.start_commit_num;
        LocationStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            LocationStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .add_location(location.into(), attributes, current_commit_num)
    }

    fn get_location(
        &self,
        location_id: &str,
        service_id: Option<&str>,
    ) -> Result<Option<Location>, LocationStoreError> {
        LocationStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            LocationStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .get_location(location_id, service_id)
    }

    fn list_locations(
        &self,
        service_id: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<LocationList, LocationStoreError> {
        LocationStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            LocationStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .list_locations(service_id, offset, limit)
    }

    fn delete_location(
        &self,
        address: &str,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError> {
        LocationStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            LocationStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .delete_location(address, current_commit_num)
    }
}

pub struct DieselConnectionLocationStore<'a, C>
where
    C: diesel::Connection<TransactionManager = AnsiTransactionManager> + 'static,
    C::Backend: diesel::backend::UsesAnsiSavepointSyntax,
{
    connection: &'a C,
}

impl<'a, C> DieselConnectionLocationStore<'a, C>
where
    C: diesel::Connection<TransactionManager = AnsiTransactionManager> + 'static,
    C::Backend: diesel::backend::UsesAnsiSavepointSyntax,
{
    pub fn new(connection: &'a C) -> Self {
        DieselConnectionLocationStore { connection }
    }
}

#[cfg(feature = "postgres")]
impl<'a> LocationStore for DieselConnectionLocationStore<'a, diesel::pg::PgConnection> {
    fn add_location(&self, location: Location) -> Result<(), LocationStoreError> {
        let attributes = make_location_attribute_models(&location.attributes, None);
        let current_commit_num = location.start_commit_num;
        LocationStoreOperations::new(self.connection).add_location(
            location.into(),
            attributes,
            current_commit_num,
        )
    }

    fn get_location(
        &self,
        location_id: &str,
        service_id: Option<&str>,
    ) -> Result<Option<Location>, LocationStoreError> {
        LocationStoreOperations::new(self.connection).get_location(location_id, service_id)
    }

    fn list_locations(
        &self,
        service_id: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<LocationList, LocationStoreError> {
        LocationStoreOperations::new(self.connection).list_locations(service_id, offset, limit)
    }

    fn delete_location(
        &self,
        address: &str,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError> {
        LocationStoreOperations::new(self.connection).delete_location(address, current_commit_num)
    }
}

#[cfg(feature = "sqlite")]
impl<'a> LocationStore for DieselConnectionLocationStore<'a, diesel::sqlite::SqliteConnection> {
    fn add_location(&self, location: Location) -> Result<(), LocationStoreError> {
        let attributes = make_location_attribute_models(&location.attributes, None);
        let current_commit_num = location.start_commit_num;
        LocationStoreOperations::new(self.connection).add_location(
            location.into(),
            attributes,
            current_commit_num,
        )
    }

    fn get_location(
        &self,
        location_id: &str,
        service_id: Option<&str>,
    ) -> Result<Option<Location>, LocationStoreError> {
        LocationStoreOperations::new(self.connection).get_location(location_id, service_id)
    }

    fn list_locations(
        &self,
        service_id: Option<&str>,
        offset: i64,
        limit: i64,
    ) -> Result<LocationList, LocationStoreError> {
        LocationStoreOperations::new(self.connection).list_locations(service_id, offset, limit)
    }

    fn delete_location(
        &self,
        address: &str,
        current_commit_num: i64,
    ) -> Result<(), LocationStoreError> {
        LocationStoreOperations::new(self.connection).delete_location(address, current_commit_num)
    }
}

#[cfg(feature = "diesel")]
impl From<Location> for NewLocationModel {
    fn from(location: Location) -> Self {
        NewLocationModel {
            location_id: location.location_id,
            location_address: location.location_address,
            location_namespace: location.location_namespace,
            owner: location.owner,
            start_commit_num: location.start_commit_num,
            end_commit_num: MAX_COMMIT_NUM,
            service_id: location.service_id,
        }
    }
}

pub fn make_location_attribute_models(
    attributes: &[LocationAttribute],
    parent_property_name: Option<String>,
) -> Vec<NewLocationAttributeModel> {
    let mut attrs = Vec::new();

    for attr in attributes {
        attrs.push(NewLocationAttributeModel {
            location_id: attr.location_id.to_string(),
            location_address: attr.location_address.to_string(),
            property_name: attr.property_name.to_string(),
            parent_property_name: parent_property_name.clone(),
            data_type: attr.data_type.to_string(),
            bytes_value: attr.bytes_value.clone(),
            boolean_value: attr.boolean_value,
            number_value: attr.number_value,
            string_value: attr.string_value.clone(),
            enum_value: attr.enum_value,
            latitude_value: attr.lat_long_value.clone().map(|lat_long| lat_long.0),
            longitude_value: attr.lat_long_value.clone().map(|lat_long| lat_long.1),
            start_commit_num: attr.start_commit_num,
            end_commit_num: MAX_COMMIT_NUM,
            service_id: attr.service_id.clone(),
        });

        if attr.struct_values.is_some() {
            let vals = attr.struct_values.as_ref().unwrap();
            if !vals.is_empty() {
                attrs.append(&mut make_location_attribute_models(
                    vals,
                    Some(attr.property_name.to_string()),
                ));
            }
        }
    }

    attrs
}

impl From<(i64, i64)> for LatLongValue {
    fn from((lat, long): (i64, i64)) -> Self {
        Self(lat, long)
    }
}

impl From<LocationAttributeModel> for LocationAttribute {
    fn from(model: LocationAttributeModel) -> Self {
        Self {
            location_id: model.location_id,
            location_address: model.location_address,
            property_name: model.property_name,
            data_type: model.data_type,
            bytes_value: model.bytes_value,
            boolean_value: model.boolean_value,
            number_value: model.number_value,
            string_value: model.string_value,
            enum_value: model.enum_value,
            struct_values: None,
            lat_long_value: create_lat_long_value(model.latitude_value, model.longitude_value),
            start_commit_num: model.start_commit_num,
            end_commit_num: model.end_commit_num,
            service_id: model.service_id,
        }
    }
}

impl From<(LocationAttributeModel, Vec<LocationAttribute>)> for LocationAttribute {
    fn from((model, children): (LocationAttributeModel, Vec<LocationAttribute>)) -> Self {
        Self {
            location_id: model.location_id,
            location_address: model.location_address,
            property_name: model.property_name,
            data_type: model.data_type,
            bytes_value: model.bytes_value,
            boolean_value: model.boolean_value,
            number_value: model.number_value,
            string_value: model.string_value,
            enum_value: model.enum_value,
            struct_values: Some(children),
            lat_long_value: create_lat_long_value(model.latitude_value, model.longitude_value),
            start_commit_num: model.start_commit_num,
            end_commit_num: model.end_commit_num,
            service_id: model.service_id,
        }
    }
}

impl From<(LocationModel, Vec<LocationAttribute>)> for Location {
    fn from((location, attributes): (LocationModel, Vec<LocationAttribute>)) -> Self {
        Self {
            location_id: location.location_id,
            location_address: location.location_address,
            location_namespace: location.location_namespace,
            owner: location.owner,
            attributes,
            start_commit_num: location.start_commit_num,
            end_commit_num: location.end_commit_num,
            service_id: location.service_id,
            last_updated: location.last_updated.map(|d| d.timestamp()),
        }
    }
}

impl From<LocationModel> for Location {
    fn from(location: LocationModel) -> Self {
        Self {
            location_id: location.location_id,
            location_address: location.location_address,
            location_namespace: location.location_namespace,
            owner: location.owner,
            attributes: Vec::new(),
            start_commit_num: location.start_commit_num,
            end_commit_num: location.end_commit_num,
            service_id: location.service_id,
            last_updated: location.last_updated.map(|d| d.timestamp()),
        }
    }
}

impl From<NewLocationModel> for Location {
    fn from(location: NewLocationModel) -> Self {
        Self {
            location_id: location.location_id,
            location_address: location.location_address,
            location_namespace: location.location_namespace,
            owner: location.owner,
            attributes: Vec::new(),
            start_commit_num: location.start_commit_num,
            end_commit_num: location.end_commit_num,
            service_id: location.service_id,
            last_updated: None,
        }
    }
}

pub fn create_lat_long_value(lat: Option<i64>, long: Option<i64>) -> Option<LatLongValue> {
    if let Some(latitude) = lat {
        long.map(|longitude| LatLongValue::from((latitude, longitude)))
    } else {
        None
    }
}
