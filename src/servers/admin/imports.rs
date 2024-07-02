use std::io::{BufReader, BufWriter, Write};

use mada_immo_admin_tonic::{
    imports_server::Imports, Empty, ImportBienRequest, ImportCommissionRequest,
    ImportLocationRequest,
};
use mada_immo_csv_import::{CSVBien, CSVCommission, CSVLocation};
use tokio_stream::StreamExt;
use tonic::{Request, Response, Streaming};

use crate::{servers::TonicRpcResult, DbPool};

#[derive(Debug, Clone)]
pub struct ImportsService {
    pub pool: DbPool,
}

#[tonic::async_trait]
impl Imports for ImportsService {
    async fn bien(
        &self,
        mut request: Request<Streaming<ImportBienRequest>>,
    ) -> TonicRpcResult<Empty> {
        let stream = request.get_mut();
        let mut buf = Vec::<u8>::new();
        let mut buf_writer = BufWriter::new(&mut buf);
        while let Some(bufs) = stream.next().await {
            buf_writer.write_all(&bufs?.content)?;
        }
        drop(buf_writer);
        let pool = self.pool.clone();
        crate::spawn_blocking(move || -> crate::Result<()> {
            let mut con = pool.get()?;
            let buf_reader = BufReader::new(buf.as_slice());
            let mut reader = csv::Reader::from_reader(buf_reader);
            for bien in reader.deserialize::<CSVBien>().flatten() {
                bien.insert(&mut con)?;
            }
            Ok(())
        })
        .await??;
        Ok(Response::new(Empty {}))
    }
    async fn location(
        &self,
        mut request: Request<Streaming<ImportLocationRequest>>,
    ) -> TonicRpcResult<Empty> {
        let stream = request.get_mut();
        let mut buf = Vec::<u8>::new();
        let mut buf_writer = BufWriter::new(&mut buf);
        while let Some(bufs) = stream.next().await {
            buf_writer.write_all(&bufs?.content)?;
        }
        drop(buf_writer);
        let pool = self.pool.clone();
        crate::spawn_blocking(move || -> crate::Result<()> {
            let mut con = pool.get()?;
            let buf_reader = BufReader::new(buf.as_slice());
            let mut reader = csv::Reader::from_reader(buf_reader);
            for bien in reader.deserialize::<CSVLocation>().flatten() {
                bien.insert(&mut con)?;
            }
            Ok(())
        })
        .await??;
        Ok(Response::new(Empty {}))
    }
    async fn commission(
        &self,
        mut request: Request<Streaming<ImportCommissionRequest>>,
    ) -> TonicRpcResult<Empty> {
        let stream = request.get_mut();
        let mut buf = Vec::<u8>::new();
        let mut buf_writer = BufWriter::new(&mut buf);
        while let Some(bufs) = stream.next().await {
            buf_writer.write_all(&bufs?.content)?;
        }
        drop(buf_writer);
        let pool = self.pool.clone();
        crate::spawn_blocking(move || -> crate::Result<()> {
            let mut con = pool.get()?;
            let buf_reader = BufReader::new(buf.as_slice());
            let mut reader = csv::Reader::from_reader(buf_reader);
            for bien in reader.deserialize::<CSVCommission>().flatten() {
                bien.insert(&mut con)?;
            }
            Ok(())
        })
        .await??;
        Ok(Response::new(Empty {}))
    }
}
