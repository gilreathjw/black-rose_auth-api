//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "auth_group_permission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub group_id: i32,
    pub permission_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::auth_group::Entity",
        from = "Column::GroupId",
        to = "super::auth_group::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AuthGroup,
    #[sea_orm(
        belongs_to = "super::auth_permission::Entity",
        from = "Column::PermissionId",
        to = "super::auth_permission::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AuthPermission,
}

impl Related<super::auth_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthGroup.def()
    }
}

impl Related<super::auth_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthPermission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
