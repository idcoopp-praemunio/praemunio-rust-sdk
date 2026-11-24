/*
 * Teams Members API
 *
 * Praemunio Teams Members API
 */

use crate::apis::configuration::Configuration;
use crate::apis::default_api;
use crate::apis::Error;
use crate::models;

/// Teams Members API client.
pub struct TeamsMembersApi<'a> {
    configuration: &'a Configuration,
}

impl<'a> TeamsMembersApi<'a> {
    /// Create a new TeamsMembersApi instance.
    pub fn new(configuration: &'a Configuration) -> Self {
        Self { configuration }
    }

    /// List team members with pagination.
    pub async fn list(&self, team_id: Option<&str>, page: Option<f64>, limit: Option<f64>) -> Result<serde_json::Value, Error<default_api::TeamsMembersGetError>> {
        default_api::teams_members_get(self.configuration, team_id, page, limit).await
    }

    /// Add a member to a team.
    pub async fn add(&self, request: models::TeamsMembersAddPostRequest) -> Result<serde_json::Value, Error<default_api::TeamsMembersAddPostError>> {
        default_api::teams_members_add_post(self.configuration, request).await
    }
}