# Copyright 2021 Cargill Incorporated
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

FROM hyperledger/grid-dev:v12 as grid-po-builder

# Copy over Cargo.toml files
COPY Cargo.toml /build/Cargo.toml
COPY cli/Cargo.toml /build/cli/Cargo.toml
COPY daemon/Cargo.toml /build/daemon/Cargo.toml
COPY griddle/Cargo.toml /build/griddle/Cargo.toml
COPY sdk/Cargo.toml /build/sdk/Cargo.toml

COPY contracts/location/Cargo.toml /build/contracts/location/Cargo.toml
COPY contracts/pike/Cargo.toml /build/contracts/pike/Cargo.toml
COPY contracts/product/Cargo.toml /build/contracts/product/Cargo.toml
COPY contracts/purchase_order/Cargo.toml /build/contracts/purchase_order/Cargo.toml
COPY contracts/schema/Cargo.toml /build/contracts/schema/Cargo.toml
COPY contracts/track_and_trace/Cargo.toml /build/contracts/track_and_trace/Cargo.toml

# Copy over build files
COPY contracts/purchase_order /build/contracts/purchase_order
COPY sdk/ /build/sdk/

# Build the contract
ARG REPO_VERSION
RUN sed -i -e "0,/version.*$/ s/version.*$/version\ =\ \"${REPO_VERSION}\"/" contracts/purchase_order/Cargo.toml \
 && cargo build --manifest-path contracts/purchase_order/Cargo.toml --target wasm32-unknown-unknown --release

# Build a scar file
# Copy the packaging directory
COPY contracts/purchase_order/packaging/scar/* \
     /build/contracts/purchase_order/packaging/scar/

# Copy the contract to the packaging directory
RUN cp /build/target/wasm32-unknown-unknown/release/grid-purchase-order-tp.wasm \
    /build/contracts/purchase_order/packaging/scar

WORKDIR /build/contracts/purchase_order/packaging/scar

# Create .scar file
RUN tar -jcvf /tmp/grid-purchase-order_${REPO_VERSION}.scar .

# -------------=== purchase_order submitter build ===-------------

FROM hyperledger/sawtooth-sabre-cli:0.5

COPY --from=grid-po-builder \
     /build/target/wasm32-unknown-unknown/release/grid-purchase-order-tp.wasm /tmp

# Copy the contract definition
COPY contracts/purchase_order/purchase_order.yaml /tmp

COPY --from=grid-po-builder \
    /tmp/grid-purchase-order*.scar /tmp
