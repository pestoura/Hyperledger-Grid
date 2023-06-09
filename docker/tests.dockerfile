# Copyright 2019 Cargill Incorporated
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

FROM hyperledger/grid-dev:v12

RUN apt-get update \
 && apt-get install -yq --no-install-recommends \
    postgresql-client \
    sqlite3 \
 && apt-get clean \
 && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml /build/Cargo.toml
COPY cli/ /build/cli
COPY daemon/ /build/daemon/
COPY griddle /build/griddle
COPY sdk /build/sdk
COPY contracts/location /build/contracts/location
COPY contracts/pike /build/contracts/pike
COPY contracts/product /build/contracts/product
COPY contracts/purchase_order /build/contracts/purchase_order
COPY contracts/schema /build/contracts/schema
COPY contracts/track_and_trace /build/contracts/track_and_trace

COPY justfile /build/justfile

WORKDIR /build
