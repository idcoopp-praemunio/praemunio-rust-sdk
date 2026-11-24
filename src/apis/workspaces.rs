/*
 * Workspaces API
 *
 * Praemunio Workspaces API
 */

use crate::apis::configuration::Configuration;
use crate::apis::default_api;
use crate::apis::Error;
use crate::models;

/// Workspaces API client.
pub struct WorkspacesApi<'a> {
    configuration: &'a Configuration,
}

impl<'a> WorkspacesApi<'a> {
    /// Create a new WorkspacesApi instance.
    pub fn new(configuration: &'a Configuration) -> Self {
        Self { configuration }
    }

    /// List all workspaces, optionally filtered by team.
    pub async fn list(&self, team_id: Option<&str>) -> Result<serde_json::Value, Error<default_api::WorkspacesGetError>> {
        default_api::workspaces_get(self.configuration, team_id).await
    }

    /// Get the drive (root folders) for the authenticated user.
    pub async fn drive(&self) -> Result<serde_json::Value, Error<default_api::WorkspacesDriveGetError>> {
        default_api::workspaces_drive_get(self.configuration).await
    }

    /// Get teams associated with workspaces.
    pub async fn teams(&self, team_id: Option<&str>) -> Result<models::WorkspacesTeamsGet200Response, Error<default_api::WorkspacesTeamsGetError>> {
        default_api::workspaces_teams_get(self.configuration, team_id).await
    }

    /// Get workspace details.
    pub async fn details(&self, workspace_id: Option<&str>) -> Result<models::WorkspacesDetailsGet200Response, Error<default_api::WorkspacesDetailsGetError>> {
        default_api::workspaces_details_get(self.configuration, workspace_id).await
    }

    /// Create a new workspace.
    pub async fn create(&self, request: models::WorkspacesCreatePostRequest) -> Result<models::WorkspacesCreatePost201Response, Error<default_api::WorkspacesCreatePostError>> {
        default_api::workspaces_create_post(self.configuration, request).await
    }

    /// Update a workspace.
    pub async fn update(&self, request: models::WorkspacesUpdatePutRequest) -> Result<models::WorkspacesUpdatePut200Response, Error<default_api::WorkspacesUpdatePutError>> {
        default_api::workspaces_update_put(self.configuration, request).await
    }

    /// Delete a workspace.
    pub async fn delete(&self, request: models::WorkspacesDeleteDeleteRequest) -> Result<serde_json::Value, Error<default_api::WorkspacesDeleteDeleteError>> {
        default_api::workspaces_delete_delete(self.configuration, request).await
    }
}