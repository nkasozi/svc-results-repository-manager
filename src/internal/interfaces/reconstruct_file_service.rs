use crate::internal::{
    models::view_models::{
        requests::reconstruct_file_from_chunks_request::ReconstructFileFromChunksRequest,
        responses::reconstruct_file_from_chunks_response::ReconstructFileFromChunksResponse,
    },
    shared_reconciler_rust_libraries::models::entities::app_errors::AppError,
};
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait ReconstructFileServiceInterface: Send + Sync {
    async fn rebuild_file(
        &self,
        file_upload_chunk: ReconstructFileFromChunksRequest,
    ) -> Result<ReconstructFileFromChunksResponse, AppError>;
}
