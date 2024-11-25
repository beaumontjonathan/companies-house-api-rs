use bytes::BytesMut;
use std::{str::from_utf8, time::Duration};
use tokio::time::timeout;

use crate::streaming::operation::StreamItem;

use super::{error::CompaniesHouseStreamingNextError, operation::CompaniesHouseStreamingOperation};

pub struct StreamConnection<O: CompaniesHouseStreamingOperation> {
    pub(super) buffer: BytesMut,
    pub(super) response: reqwest::Response,
    pub(super) max_chunk_timeout: Duration,
    pub(super) _operation: O,
}

impl<O: CompaniesHouseStreamingOperation> StreamConnection<O> {
    pub async fn next(&mut self) -> Result<StreamItem<O::Data>, CompaniesHouseStreamingNextError> {
        loop {
            if let Some((index, _)) = self.buffer.iter().enumerate().find(|(_, c)| **c == b'\n') {
                let left = self.buffer.split_to(index + 1);
                let str = from_utf8(&left)
                    .map_err(CompaniesHouseStreamingNextError::BadItemEncoding)?
                    .trim();
                if str.is_empty() {
                    log::trace!("Buffer contains empty line");
                } else {
                    log::trace!(length = str.len(); "Buffer contains next stream item");

                    let value_err = match serde_json::from_str(str) {
                        Ok(data) => return Ok(data),
                        Err(err) => err,
                    };

                    if let Ok(value) = serde_json::from_str(str) {
                        return Err(CompaniesHouseStreamingNextError::BadItemData {
                            inner: value_err,
                            value,
                        });
                    };

                    return Err(CompaniesHouseStreamingNextError::BadItemJson {
                        inner: value_err,
                        text: str.to_owned(),
                    });
                }
            }

            log::trace!("Buffer contains no items, reading next response chunk");

            let Some(chunk) = timeout(self.max_chunk_timeout, self.response.chunk())
                .await
                .map_err(|_| {
                    log::info!("Chunk timeout exceeded");
                    CompaniesHouseStreamingNextError::ChunkTimeout
                })?
                .map_err(CompaniesHouseStreamingNextError::BadChunk)?
            else {
                return Err(CompaniesHouseStreamingNextError::StreamComplete);
            };

            log::trace!(bytes = chunk.len(); "Stream chunk received");

            if self.buffer.is_empty() && chunk.len() == 1 && chunk[0] == b'\n' {
                log::info!("Heartbeat chunk received");
            } else {
                self.buffer.extend_from_slice(&chunk);
            }
        }
    }
}
