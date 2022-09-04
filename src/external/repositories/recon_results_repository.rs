use async_trait::async_trait;
use dapr::{dapr::dapr::proto::runtime::v1::dapr_client::DaprClient, Client};
use tonic::transport::Channel as TonicChannel;

use crate::internal::{
    interfaces::recon_results_repository::ReconResultsRepositoryInterface,
    shared_reconciler_rust_libraries::models::entities::{
        app_errors::{AppError, AppErrorKind},
        reconstructed_file::ReconstructedFile,
    },
};

pub struct ReconResultsRepository {
    //the dapr server ip
    pub dapr_grpc_server_address: String,

    //the dapr component name
    pub dapr_component_name: String,

    //the dapr state store name
    pub dapr_state_store_name: String,
}

#[async_trait]
impl ReconResultsRepositoryInterface for ReconResultsRepository {
    async fn get_reconstructed_file(
        &self,
        file_id: &String,
    ) -> Result<ReconstructedFile, AppError> {
        // Create the client
        let mut client = self.get_dapr_connection().await?;

        let get_response = client
            .get_state(
                self.dapr_state_store_name.clone(),
                String::from(file_id),
                None,
            )
            .await;

        match get_response {
            Ok(s) => {
                let retrieval_result: Result<ReconstructedFile, _> =
                    serde_json::from_slice(&s.data);
                match retrieval_result {
                    Ok(unmarshalled_file_details) => return Ok(unmarshalled_file_details),
                    Err(e) => return Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
                };
            }
            Err(e) => return Err(AppError::new(AppErrorKind::NotFound, e.to_string())),
        }
    }

    async fn save_reconstructed_file(
        &self,
        reconstructed_file: &ReconstructedFile,
    ) -> Result<String, AppError> {
        // Create the client
        let mut client = self.get_dapr_connection().await?;

        let key = reconstructed_file.file_id.clone();
        let val = serde_json::to_vec(&reconstructed_file).unwrap();

        // save key-value pair in the state store
        let save_result = client
            .save_state(self.dapr_state_store_name.clone(), vec![(key.clone(), val)])
            .await;

        match save_result {
            Ok(_s) => return Ok(key.clone()),
            Err(e) => return Err(AppError::new(AppErrorKind::InternalError, e.to_string())),
        }
    }
}

impl ReconResultsRepository {
    async fn get_dapr_connection(&self) -> Result<Client<DaprClient<TonicChannel>>, AppError> {
        // Create the client
        let dapr_grpc_server_address = self.dapr_grpc_server_address.clone();

        //connect to dapr
        let client_connect_result =
            dapr::Client::<dapr::client::TonicClient>::connect(dapr_grpc_server_address).await;

        //handle the connection result
        match client_connect_result {
            //connection succeeded
            Ok(s) => return Ok(s),
            //connection failed
            Err(e) => return Err(AppError::new(AppErrorKind::ConnectionError, e.to_string())),
        }
    }
}
