use client_api_test_util::generate_unique_registered_user_client;
use database_entity::dto::AFRole;
use shared_entity::dto::workspace_dto::CreateWorkspaceMember;
use shared_entity::dto::workspace_dto::CreateWorkspaceParam;

#[tokio::test]
async fn add_and_delete_workspace_for_user() {
  let (c, _user) = generate_unique_registered_user_client().await;
  let workspaces = c.get_workspaces().await.unwrap();
  assert_eq!(workspaces.0.len(), 1);
  let newly_addad_workspace = c
    .create_workspace(CreateWorkspaceParam {
      workspace_name: Some("my_workspace".to_string()),
    })
    .await
    .unwrap();
  let workspaces = c.get_workspaces().await.unwrap();
  assert_eq!(workspaces.0.len(), 2);

  let _ = workspaces
    .0
    .iter()
    .find(|w| {
      w.workspace_name == "my_workspace" && w.workspace_id == newly_addad_workspace.workspace_id
    })
    .unwrap();

  c.delete_workspace(&newly_addad_workspace.workspace_id.to_string())
    .await
    .unwrap();
  let workspaces = c.get_workspaces().await.unwrap();
  assert_eq!(workspaces.0.len(), 1);
}

#[tokio::test]
async fn add_and_delete_workspace_for_non_owner_user() {
  let (member, member_user) = generate_unique_registered_user_client().await;

  // Owner added member to workspace
  let (owner, _user) = generate_unique_registered_user_client().await;
  let owner_workspace = owner
    .create_workspace(CreateWorkspaceParam {
      workspace_name: Some("owner_workspace".to_string()),
    })
    .await
    .unwrap();

  owner
    .add_workspace_members(
      owner_workspace.workspace_id.to_string(),
      vec![CreateWorkspaceMember {
        email: member_user.email.clone(),
        role: AFRole::Member,
      }],
    )
    .await
    .unwrap();

  // Member should have 2 workspaces
  let member_workspaces = member.get_workspaces().await.unwrap();
  assert_eq!(member_workspaces.0.len(), 2);

  owner
    .remove_workspace_members(
      owner_workspace.workspace_id.to_string(),
      vec![member_user.email],
    )
    .await
    .unwrap();

  // Member should have 1 workspaces, because owner removed him
  let member_workspaces = member.get_workspaces().await.unwrap();
  assert_eq!(member_workspaces.0.len(), 1);
}
