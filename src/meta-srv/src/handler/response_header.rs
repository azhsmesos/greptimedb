// Copyright 2023 Greptime Team
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

use api::v1::meta::{HeartbeatRequest, ResponseHeader, PROTOCOL_VERSION};

use crate::error::Result;
use crate::handler::{HeartbeatAccumulator, HeartbeatHandler};
use crate::metasrv::Context;

pub struct ResponseHeaderHandler;

#[async_trait::async_trait]
impl HeartbeatHandler for ResponseHeaderHandler {
    async fn handle(
        &self,
        req: &HeartbeatRequest,
        _ctx: &Context,
        acc: &mut HeartbeatAccumulator,
    ) -> Result<()> {
        let HeartbeatRequest { header, .. } = req;
        let res_header = ResponseHeader {
            protocol_version: PROTOCOL_VERSION,
            cluster_id: header.as_ref().map_or(0, |h| h.cluster_id),
            ..Default::default()
        };
        acc.header = Some(res_header);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;

    use api::v1::meta::{HeartbeatResponse, RequestHeader};

    use super::*;
    use crate::handler::Context;
    use crate::service::store::memory::MemStore;

    #[tokio::test]
    async fn test_handle_heartbeat_resp_header() {
        let kv_store = Arc::new(MemStore::new());
        let ctx = Context {
            datanode_lease_secs: 30,
            server_addr: "127.0.0.1:0000".to_string(),
            kv_store,
            election: None,
            skip_all: Arc::new(AtomicBool::new(false)),
        };

        let req = HeartbeatRequest {
            header: Some(RequestHeader::new((1, 2))),
            ..Default::default()
        };
        let mut acc = HeartbeatAccumulator::default();

        let response_handler = ResponseHeaderHandler {};
        response_handler.handle(&req, &ctx, &mut acc).await.unwrap();
        let header = std::mem::take(&mut acc.header);
        let res = HeartbeatResponse {
            header,
            payload: acc.into_payload(),
        };
        assert_eq!(1, res.header.unwrap().cluster_id);
    }
}
