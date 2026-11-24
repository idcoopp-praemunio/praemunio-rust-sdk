/*
 * Teams API
 *
 * Praemunio Teams API
 */

use crate::apis::configuration::Configuration;
use crate::apis::default_api;
use crate::apis::Error;
use crate::models;

/// Teams API client.
pub struct TeamsApi<'a> {
    configuration: &'a Configuration,
}

impl<'a> TeamsApi<'a> {
    /// Create a new TeamsApi instance.
    pub fn new(configuration: &'a Configuration) -> Self {
        Self { configuration }
    }

    /// List all teams.
    pub async fn list(&self) -> Result<serde_json::Value, Error<default_api::TeamsGetError>> {
        default_api::teams_get(self.configuration).await
    }

    /// Get teams dashboard data.
    pub async fn dashboard(&self) -> Result<serde_json::Value, Error<default_api::TeamsDashboardGetError>> {
        default_api::teams_dashboard_get(self.configuration).await
    }

    /// Get team information.
    pub async fn info(&self, body: serde_json::Value, team_id: Option<Vec<String>>) -> Result<serde_json::Value, Error<default_api::TeamsInfoGetError>> {
        default_api::teams_info_get(self.configuration, body, team_id).await
    }

    /// Get workspaces associated with teams.
    pub async fn workspaces(&self, team_id: Option<&str>) -> Result<serde_json::Value, Error<default_api::TeamsWorkspacesGetError>> {
        default_api::teams_workspaces_get(self.configuration, team_id).await
    }

    /// Get team logs.
    pub async fn logs(&self, team_id: Option<Vec<String>>, page: Option<&str>, limit: Option<&str>) -> Result<serde_json::Value, Error<default_api::TeamsLogsGetError>> {
        default_api::teams_logs_get(self.configuration, team_id, page, limit).await
    }

    /// Create a new team.
    pub async fn create(&self, request: models::TeamsCreatePostRequest) -> Result<serde_json::Value, Error<default_api::TeamsCreatePostError>> {
        default_api::teams_create_post(self.configuration, request).await
    }

    /// Update a team.
    pub async fn update(&self, request: models::TeamsUpdatePutRequest) -> Result<serde_json::Value, Error<default_api::TeamsUpdatePutError>> {
        default_api::teams_update_put(self.configuration, request).await
    }

    /// Delete a team.
    pub async fn delete(&self, request: models::TeamsDeleteDeleteRequest) -> Result<serde_json::Value, Error<default_api::TeamsDeleteDeleteError>> {
        default_api::teams_delete_delete(self.configuration, request).await
    }
}