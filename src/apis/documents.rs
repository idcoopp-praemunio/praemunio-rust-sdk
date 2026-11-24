/*
 * Documents API
 *
 * Praemunio Documents API
 */

use crate::apis::configuration::Configuration;
use crate::apis::default_api;
use crate::apis::Error;
use crate::models;
use std::path::PathBuf;

/// Documents API client.
pub struct DocumentsApi<'a> {
    configuration: &'a Configuration,
}

impl<'a> DocumentsApi<'a> {
    /// Create a new DocumentsApi instance.
    pub fn new(configuration: &'a Configuration) -> Self {
        Self { configuration }
    }

    /// List documents in a workspace.
    pub async fn list(&self, workspace_id: Option<&str>) -> Result<serde_json::Value, Error<default_api::DocumentsGetError>> {
        default_api::documents_get(self.configuration, workspace_id).await
    }

    /// Get document details.
    pub async fn details(&self, document_id: Option<&str>) -> Result<serde_json::Value, Error<default_api::DocumentsDetailsGetError>> {
        default_api::documents_details_get(self.configuration, document_id).await
    }

    /// Upload a document.
    pub async fn upload(&self, workspace_id: Option<&str>, folder_id: Option<&str>, file1: Option<PathBuf>, file2: Option<PathBuf>) -> Result<serde_json::Value, Error<default_api::DocumentsUploadPostError>> {
        default_api::documents_upload_post(self.configuration, workspace_id, folder_id, file1, file2).await
    }

    /// Stream a document.
    pub async fn stream(&self, body: serde_json::Value, document_id: Option<&str>) -> Result<serde_json::Value, Error<default_api::DocumentsStreamGetError>> {
        default_api::documents_stream_get(self.configuration, body, document_id).await
    }

    /// Archive a document.
    pub async fn archive(&self, request: models::DocumentsArchivePostRequest) -> Result<serde_json::Value, Error<default_api::DocumentsArchivePostError>> {
        default_api::documents_archive_post(self.configuration, request).await
    }

    /// Restore an archived document.
    pub async fn restore(&self, request: models::DocumentsRestorePostRequest) -> Result<serde_json::Value, Error<default_api::DocumentsRestorePostError>> {
        default_api::documents_restore_post(self.configuration, request).await
    }

    /// Move a document.
    pub async fn move_document(&self, request: models::DocumentsMovePostRequest) -> Result<serde_json::Value, Error<default_api::DocumentsMovePostError>> {
        default_api::documents_move_post(self.configuration, request).await
    }

    /// Update a document.
    pub async fn update(&self, request: models::DocumentsUpdatePutRequest) -> Result<serde_json::Value, Error<default_api::DocumentsUpdatePutError>> {
        default_api::documents_update_put(self.configuration, request).await
    }

    /// Delete a document.
    pub async fn delete(&self, request: models::DocumentsDeleteDeleteRequest) -> Result<serde_json::Value, Error<default_api::DocumentsDeleteDeleteError>> {
        default_api::documents_delete_delete(self.configuration, request).await
    }
}