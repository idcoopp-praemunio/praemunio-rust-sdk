# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**auth_auth_token_post**](DefaultApi.md#auth_auth_token_post) | **POST** /auth/auth-token | API KEY Authentification
[**documents_archive_post**](DefaultApi.md#documents_archive_post) | **POST** /documents/archive | Archive Document
[**documents_delete_delete**](DefaultApi.md#documents_delete_delete) | **DELETE** /documents/delete | Delete Document
[**documents_details_get**](DefaultApi.md#documents_details_get) | **GET** /documents/details | Get Document Info
[**documents_get**](DefaultApi.md#documents_get) | **GET** /documents | Get Document By Workspace
[**documents_move_post**](DefaultApi.md#documents_move_post) | **POST** /documents/move | Move Document
[**documents_restore_post**](DefaultApi.md#documents_restore_post) | **POST** /documents/restore | Restore Document 
[**documents_stream_get**](DefaultApi.md#documents_stream_get) | **GET** /documents/stream | Stream Document Copy
[**documents_update_put**](DefaultApi.md#documents_update_put) | **PUT** /documents/update | Update Document
[**documents_upload_post**](DefaultApi.md#documents_upload_post) | **POST** /documents/upload | Upload Document 
[**folders_content_get**](DefaultApi.md#folders_content_get) | **GET** /folders/content | Get Folder Content
[**folders_create_post**](DefaultApi.md#folders_create_post) | **POST** /folders/create | Create Folder
[**folders_delete_delete**](DefaultApi.md#folders_delete_delete) | **DELETE** /folders/delete | Delete Folder
[**folders_details_get**](DefaultApi.md#folders_details_get) | **GET** /folders/details | Get Folder Details
[**folders_get**](DefaultApi.md#folders_get) | **GET** /folders | Get Folders 
[**folders_move_post**](DefaultApi.md#folders_move_post) | **POST** /folders/move | Move Folder
[**folders_update_put**](DefaultApi.md#folders_update_put) | **PUT** /folders/update | Update Folder
[**teams_create_post**](DefaultApi.md#teams_create_post) | **POST** /teams/create | Create Team
[**teams_dashboard_get**](DefaultApi.md#teams_dashboard_get) | **GET** /teams/dashboard | Get Team Data
[**teams_delete_delete**](DefaultApi.md#teams_delete_delete) | **DELETE** /teams/delete | Delete Team
[**teams_get**](DefaultApi.md#teams_get) | **GET** /teams | Get Account Teams
[**teams_info_get**](DefaultApi.md#teams_info_get) | **GET** /teams/info | Get Team Details
[**teams_logs_get**](DefaultApi.md#teams_logs_get) | **GET** /teams/logs | Get Team logs
[**teams_members_add_post**](DefaultApi.md#teams_members_add_post) | **POST** /teams/members/add | Add Team Member Copy
[**teams_members_get**](DefaultApi.md#teams_members_get) | **GET** /teams/members | Get Team Members
[**teams_update_put**](DefaultApi.md#teams_update_put) | **PUT** /teams/update | Update Team Data
[**teams_workspaces_get**](DefaultApi.md#teams_workspaces_get) | **GET** /teams/workspaces | Get Team workspaces
[**workspaces_create_post**](DefaultApi.md#workspaces_create_post) | **POST** /workspaces/create | Create New Workspace
[**workspaces_delete_delete**](DefaultApi.md#workspaces_delete_delete) | **DELETE** /workspaces/delete | Delete workspace
[**workspaces_details_get**](DefaultApi.md#workspaces_details_get) | **GET** /workspaces/details | Get Workspace Details
[**workspaces_drive_get**](DefaultApi.md#workspaces_drive_get) | **GET** /workspaces/drive | Get Account Drive
[**workspaces_get**](DefaultApi.md#workspaces_get) | **GET** /workspaces | Get Workspaces By Team
[**workspaces_teams_get**](DefaultApi.md#workspaces_teams_get) | **GET** /workspaces/teams | Get Team Workspaces
[**workspaces_update_put**](DefaultApi.md#workspaces_update_put) | **PUT** /workspaces/update | Update workspace



## auth_auth_token_post

> serde_json::Value auth_auth_token_post(auth_auth_token_post_request)
API KEY Authentification



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**auth_auth_token_post_request** | [**AuthAuthTokenPostRequest**](AuthAuthTokenPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## documents_archive_post

> serde_json::Value documents_archive_post(documents_archive_post_request)
Archive Document



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**documents_archive_post_request** | [**DocumentsArchivePostRequest**](DocumentsArchivePostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## documents_delete_delete

> serde_json::Value documents_delete_delete(documents_delete_delete_request)
Delete Document



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**documents_delete_delete_request** | [**DocumentsDeleteDeleteRequest**](DocumentsDeleteDeleteRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## documents_details_get

> serde_json::Value documents_details_get(document_id)
Get Document Info



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**document_id** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## documents_get

> serde_json::Value documents_get(workspace_id)
Get Document By Workspace



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## documents_move_post

> serde_json::Value documents_move_post(documents_move_post_request)
Move Document



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**documents_move_post_request** | [**DocumentsMovePostRequest**](DocumentsMovePostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## documents_restore_post

> serde_json::Value documents_restore_post(documents_restore_post_request)
Restore Document 



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**documents_restore_post_request** | [**DocumentsRestorePostRequest**](DocumentsRestorePostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## documents_stream_get

> serde_json::Value documents_stream_get(body, document_id)
Stream Document Copy



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**document_id** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## documents_update_put

> serde_json::Value documents_update_put(documents_update_put_request)
Update Document



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**documents_update_put_request** | [**DocumentsUpdatePutRequest**](DocumentsUpdatePutRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## documents_upload_post

> serde_json::Value documents_upload_post(workspace_id, folder_id, file1, file2)
Upload Document 



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | Option<**String**> |  |  |
**folder_id** | Option<**String**> |  |  |
**file1** | Option<**std::path::PathBuf**> |  |  |
**file2** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_content_get

> models::FoldersContentGet200Response folders_content_get(folder_id)
Get Folder Content



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**models::FoldersContentGet200Response**](_folders_content_get_200_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_create_post

> models::FoldersCreatePost201Response folders_create_post(folders_create_post_request)
Create Folder



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folders_create_post_request** | [**FoldersCreatePostRequest**](FoldersCreatePostRequest.md) |  | [required] |

### Return type

[**models::FoldersCreatePost201Response**](_folders_create_post_201_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_delete_delete

> serde_json::Value folders_delete_delete(folders_delete_delete_request)
Delete Folder



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folders_delete_delete_request** | [**FoldersDeleteDeleteRequest**](FoldersDeleteDeleteRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_details_get

> models::FoldersDetailsGet200Response folders_details_get(folder_id)
Get Folder Details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folder_id** | Option<**String**> |  |  |

### Return type

[**models::FoldersDetailsGet200Response**](_folders_details_get_200_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_get

> models::FoldersGet200Response folders_get(workspace_id)
Get Folders 



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | **String** |  | [required] |

### Return type

[**models::FoldersGet200Response**](_folders_get_200_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_move_post

> serde_json::Value folders_move_post(folders_delete_delete_request)
Move Folder



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folders_delete_delete_request** | [**FoldersDeleteDeleteRequest**](FoldersDeleteDeleteRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## folders_update_put

> serde_json::Value folders_update_put(folders_update_put_request)
Update Folder



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**folders_update_put_request** | [**FoldersUpdatePutRequest**](FoldersUpdatePutRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_create_post

> serde_json::Value teams_create_post(teams_create_post_request)
Create Team



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**teams_create_post_request** | [**TeamsCreatePostRequest**](TeamsCreatePostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_dashboard_get

> serde_json::Value teams_dashboard_get()
Get Team Data



### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_delete_delete

> serde_json::Value teams_delete_delete(teams_delete_delete_request)
Delete Team



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**teams_delete_delete_request** | [**TeamsDeleteDeleteRequest**](TeamsDeleteDeleteRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_get

> serde_json::Value teams_get()
Get Account Teams



### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_info_get

> serde_json::Value teams_info_get(body, team_id)
Get Team Details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | **serde_json::Value** |  | [required] |
**team_id** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_logs_get

> serde_json::Value teams_logs_get(team_id, page, limit)
Get Team logs



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | Option<[**Vec<String>**](String.md)> | team Id |  |
**page** | Option<**String**> |  |  |
**limit** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_members_add_post

> serde_json::Value teams_members_add_post(teams_members_add_post_request)
Add Team Member Copy



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**teams_members_add_post_request** | [**TeamsMembersAddPostRequest**](TeamsMembersAddPostRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_members_get

> serde_json::Value teams_members_get(team_id, page, limit)
Get Team Members



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | Option<**String**> |  |  |
**page** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_update_put

> serde_json::Value teams_update_put(teams_update_put_request)
Update Team Data



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**teams_update_put_request** | [**TeamsUpdatePutRequest**](TeamsUpdatePutRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## teams_workspaces_get

> serde_json::Value teams_workspaces_get(team_id)
Get Team workspaces



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | Option<**String**> | team Id |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_create_post

> models::WorkspacesCreatePost201Response workspaces_create_post(workspaces_create_post_request)
Create New Workspace



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspaces_create_post_request** | [**WorkspacesCreatePostRequest**](WorkspacesCreatePostRequest.md) |  | [required] |

### Return type

[**models::WorkspacesCreatePost201Response**](_workspaces_create_post_201_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_delete_delete

> serde_json::Value workspaces_delete_delete(workspaces_delete_delete_request)
Delete workspace



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspaces_delete_delete_request** | [**WorkspacesDeleteDeleteRequest**](WorkspacesDeleteDeleteRequest.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_details_get

> models::WorkspacesDetailsGet200Response workspaces_details_get(workspace_id)
Get Workspace Details



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspace_id** | Option<**String**> |  |  |

### Return type

[**models::WorkspacesDetailsGet200Response**](_workspaces_details_get_200_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_drive_get

> serde_json::Value workspaces_drive_get()
Get Account Drive



### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_get

> serde_json::Value workspaces_get(team_id)
Get Workspaces By Team



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | Option<**String**> | team Id |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_teams_get

> models::WorkspacesTeamsGet200Response workspaces_teams_get(team_id)
Get Team Workspaces



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**team_id** | Option<**String**> |  |  |

### Return type

[**models::WorkspacesTeamsGet200Response**](_workspaces_teams_get_200_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## workspaces_update_put

> models::WorkspacesUpdatePut200Response workspaces_update_put(workspaces_update_put_request)
Update workspace



### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**workspaces_update_put_request** | [**WorkspacesUpdatePutRequest**](WorkspacesUpdatePutRequest.md) |  | [required] |

### Return type

[**models::WorkspacesUpdatePut200Response**](_workspaces_update_put_200_response.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

