use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::auth::model::{
    groups::{GroupDto, GroupType, NewGroupDto},
    user::UserDto,
};
use crate::auth::route::{auth, groups, user};
use crate::eve::model::character::CharacterAffiliationDto;

pub fn routes() -> Router {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            auth::login,
            auth::logout,
            user::get_user,
            user::get_user_main_character,
            user::get_user_characters,
            groups::create_group,
            groups::get_groups,
            groups::get_group_by_id,
            groups::update_group,
            groups::delete_group
        ),
        components(schemas(UserDto, NewGroupDto, GroupType, CharacterAffiliationDto, GroupDto)),
        tags(
            (name = "Black Rose Auth API", description = "Black Rose Auth API endpoints")
        )
    )]
    struct ApiDoc;

    use crate::auth::route::auth::auth_routes;
    use crate::auth::route::groups::group_routes;
    use crate::auth::route::user::user_routes;

    let routes = Router::new()
        .nest("/auth", auth_routes())
        .nest("/user", user_routes())
        .nest("/groups", group_routes());

    if cfg!(debug_assertions) {
        routes.merge(SwaggerUi::new("/docs").url("/openapi.json", ApiDoc::openapi()))
    } else {
        routes
    }
}
