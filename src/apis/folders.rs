/*
 * Folders API
 *
 * Praemunio Folders API
 */

use crate::apis::configuration::Configuration;
use crate::apis::default_api;
use crate::apis::Error;
use crate::models;

/// Folders API client.
pub struct FoldersApi<'a> {
    configuration: &'a Configuration,
}

impl<'a> FoldersApi<'a> {
    /// Create a new FoldersApi instance.
    pub fn new(configuration: &'a Configuration) -> Self {
        Self { configuration }
    }

    /// List folders in a workspace.
    pub async fn list(&self, workspace_id: &str) -> Result<models::FoldersGet200Response, Error<default_api::FoldersGetError>> {
        default_api::folders_get(self.configuration, workspace_id).await
    }

    /// Get folder content.
    pub async fn content(&self, folder_id: Option<Vec<String>>) -> Result<models::FoldersContentGet200Response, Error<default_api::FoldersContentGetError>> {
        default_api::folders_content_get(self.configuration, folder_id).await
    }

    /// Get folder details.
    pub async fn details(&self, folder_id: Option<&str>) -> Result<models::FoldersDetailsGet200Response, Error<default_api::FoldersDetailsGetError>> {
        default_api::folders_details_get(self.configuration, folder_id).await
    }

    /// Create a new folder.
    pub async fn create(&self, request: models::FoldersCreatePostRequest) -> Result<models::FoldersCreatePost201Response, Error<default_api::FoldersCreatePostError>> {
        default_api::folders_create_post(self.configuration, request).await
    }

    /// Update a folder.
    pub async fn update(&self, request: models::FoldersUpdatePutRequest) -> Result<serde_json::Value, Error<default_api::FoldersUpdatePutError>> {
        default_api::folders_update_put(self.configuration, request).await
    }

    /// Move a folder.
    pub async fn move_folder(&self, request: models::FoldersDeleteDeleteRequest) -> Result<serde_json::Value, Error<default_api::FoldersMovePostError>> {
        default_api::folders_move_post(self.configuration, request).await
    }

    /// Delete a folder.
    pub async fn delete(&self, request: models::FoldersDeleteDeleteRequest) -> Result<serde_json::Value, Error<default_api::FoldersDeleteDeleteError>> {
        default_api::folders_delete_delete(self.configuration, request).await
    }
}