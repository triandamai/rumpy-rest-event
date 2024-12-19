use crate::common::api_response::{ApiResponse, PaginationRequest, PagingResponse};
use crate::common::app_state::AppState;
use crate::common::jwt::AuthContext;
use crate::common::lang::Lang;
use crate::common::minio::MinIO;
use crate::common::multipart_file::SingleFileExtractor;
use crate::common::orm::orm::Orm;
use crate::common::utils::{
    create_object_id_option, QUERY_ASC, QUERY_DESC, QUERY_LATEST, QUERY_OLDEST,
};

use crate::common::middleware::Json;
use crate::common::permission::permission::app;
use crate::dto::file_attachment_dto::FileAttachmentDTO;
use crate::dto::product_dto::ProductDTO;
use crate::entity::file_attachment::FileAttachment;
use crate::entity::product::Product;
use crate::feature::product::product_model::{CreateProductRequest, UpdateProductRequest};
use crate::translate;
use axum::extract::{Path, Query, State};
use bson::oid::ObjectId;
use bson::DateTime;
use log::info;
use validator::Validate;

pub async fn get_list_product(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    query: Query<PaginationRequest>,
) -> ApiResponse<PagingResponse<ProductDTO>> {
    info!(target: "product::list","{} trying to get list product",auth_context.claims.sub);
    if !auth_context.authorize(app::product::READ) {
        info!(target:"product::list","{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    if auth_context.branch_id.is_none() {
        info!(target:"product::list","failed to get branch id");
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let default = String::new();
    let filter = query.filter.clone().unwrap_or(default.clone());
    let mut get = Orm::get("product");

    if query.q.is_some() {
        let text = query.q.clone().unwrap_or(default);
        get = get.filter_string("$text", Some("$search"), text.as_str());
    }

    if filter == QUERY_ASC.to_string() {
        get = get.group_by_asc("product_name");
    }

    if filter == QUERY_DESC.to_string() {
        get = get.group_by_desc("product_name");
    }

    if filter == QUERY_LATEST.to_string() {
        get = get.group_by_desc("created_at");
    }

    if filter == QUERY_OLDEST.to_string() {
        get = get.group_by_asc("created_at");
    }

    let find_all_branch = get
        .and()
        .filter_bool("deleted", None, false)
        .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .join_one("file-attachment", "_id", "ref_id", "product_image")
        .join_one("account", "created_by_id", "_id", "created_by")
        .pageable::<ProductDTO>(query.page.unwrap_or(1), query.size.unwrap_or(10), &state.db)
        .await;
    info!(target: "product::list","successfully get list product");
    ApiResponse::ok(
        find_all_branch.unwrap(),
        translate!("product.list.success", lang).as_str(),
    )
}

pub async fn get_detail_product(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(product_id): Path<String>,
) -> ApiResponse<ProductDTO> {
    info!(target: "product::detail", "{} trying to get product detail",auth_context.claims.sub);
    if !auth_context.authorize(app::product::READ) {
        info!(target:"product::detail","{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    if auth_context.branch_id.is_none() {
        info!(target:"product::detail","failed to get branch id");
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let id = create_object_id_option(product_id.as_str());
    if id.is_none() {
        info!(target: "product::detail","Failed CREATE ObjectId");
        return ApiResponse::un_authorized(translate!("product.not-found", lang).as_str());
    }

    let find_product = Orm::get("product")
        .and()
        .filter_object_id("_id", &id.unwrap())
        .filter_object_id("branch_id", &auth_context.branch_id.unwrap())
        .join_one("file-attachment", "_id", "ref_id", "product_image")
        .join_one("account", "created_by_id", "_id", "created_by")
        .one::<ProductDTO>(&state.db)
        .await;

    if find_product.is_err() {
        info!(target: "product::detail","Not found");
        return ApiResponse::not_found(translate!("product.not-found", lang).as_str());
    }

    info!(target: "product::detail","Successfully get product detail");
    ApiResponse::ok(
        find_product.unwrap(),
        translate!("product.found", lang).as_str(),
    )
}

pub async fn create_product(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Json(body): Json<CreateProductRequest>
) -> ApiResponse<ProductDTO> {
    info!(target: "product::create", "{} trying to CREATE product", auth_context.claims.sub);
    if !auth_context.authorize(app::product::CREATE) {
        info!(target: "product::create", "{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    if auth_context.branch_id.is_none() {
        info!(target: "product::create", "{} not permitted beacause not associated with any branch", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("validation.error", lang).as_str(),
        );
    }

    let product = Product {
        id: Some(ObjectId::new()),
        branch_id: auth_context.branch_id,
        product_name: body.product_name.clone(),
        product_description: body.product_description.clone(),
        product_price: body.product_price,
        product_selling_price: body.product_selling_price,
        product_profit: body.product_profit,
        product_stock: body.product_stock,
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
        deleted: false,
        created_by_id: auth_context.user_id,
    };

    let save = Orm::insert("product").one(&product, &state.db).await;
    if save.is_err() {
        info!(target: "product::create", "Create Product Failed");
        return ApiResponse::failed(translate!("product.create.failed", lang).as_str());
    }

    info!(target: "product::create", "Successfully CREATE product");
    ApiResponse::ok(
        product.to_dto(),
        translate!("product.create.success", lang).as_str(),
    )
}

pub async fn update_product(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(product_id): Path<String>,
    Json(body): Json<UpdateProductRequest>,
) -> ApiResponse<ProductDTO> {
    info!(target: "product::update", "{} trying to UPDATE product", auth_context.claims.sub);
    if !auth_context.authorize(app::product::UPDATE) {
        info!(target: "product::update", "{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let validate = body.validate();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("product.update.failed", lang).as_str(),
        );
    }

    let product_id = create_object_id_option(product_id.as_str());
    if product_id.is_none() {
        info!(target: "product::update", "Failed CREATE ObjectId");
        return ApiResponse::un_authorized(translate!("product.not-found", lang).as_str());
    }

    let find_product = Orm::get("product")
        .filter_object_id("_id", &product_id.unwrap())
        .join_one("file-attachment", "_id", "ref_id", "product_image")
        .join_one("account", "created_by_id", "_id", "created_by")
        .one::<ProductDTO>(&state.db)
        .await;
    if find_product.is_err() {
        info!(target: "product::update","Product not found");
        return ApiResponse::not_found(translate!("product.not-found", lang).as_str());
    }
    let mut product = find_product.unwrap();

    let mut save = Orm::update("product");
    if body.product_name.is_some() {
        product.product_name = body.product_name.clone().unwrap();
        save = save.set_str("product_name", &body.product_name.clone().unwrap());
    }
    if body.product_description.is_some() {
        product.product_description = body.product_description.clone().unwrap();
        save = save.set_str(
            "product_description",
            &body.product_description.clone().unwrap(),
        );
    }
    if body.product_price.is_some() {
        product.product_price = body.product_price.clone().unwrap();
        save = save.set_float("product_price", &body.product_price.clone().unwrap());
    }
    if body.product_selling_price.is_some() {
        product.product_selling_price = body.product_selling_price.clone().unwrap();
        save = save.set_float(
            "product_selling_price",
            &body.product_selling_price.clone().unwrap(),
        );
    }
    if body.product_profit.is_some() {
        product.product_profit = body.product_profit.clone().unwrap();
        save = save.set_float("product_profit", &body.product_profit.clone().unwrap());
    }

    let save_data = save
        .filter_object_id("_id", &product_id.unwrap())
        .set_datetime("updated_at", DateTime::now())
        .execute_one(&state.db)
        .await;

    if save_data.is_err() {
        info!(target: "product::update", "Failed UPDATE product");
        return ApiResponse::failed(translate!("product.update.failed", lang).as_str());
    }
    info!(target: "product::update", "Successfully UPDATE product");
    ApiResponse::ok(product, translate!("product.update.success", lang).as_str())
}

pub async fn delete_product(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    Path(product_id): Path<String>,
) -> ApiResponse<String> {
    info!(target: "product::delete", "{} trying to DELETE product", auth_context.claims.sub);
    if !auth_context.authorize(app::product::DELETE) {
        info!(target: "product::delete", "{} trying to DELETE product", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let id = create_object_id_option(product_id.as_str());
    if id.is_none() {
        return ApiResponse::un_authorized(translate!("product.not-found", lang).as_str());
    }

    let update = Orm::update("product")
        .filter_object_id("_id", &id.unwrap())
        .set_bool("deleted", true)
        .execute_one(&state.db)
        .await;

    if update.is_err() {
        info!(target: "product::delete", "{} trying to DELETE product", auth_context.claims.sub);
        return ApiResponse::failed(translate!("product.delete.failed", lang).as_str());
    }
    info!(target: "product::delete", "successfully deleted product");
    ApiResponse::ok(
        "OK".to_string(),
        translate!("product.delete.success", lang).as_str(),
    )
}

pub async fn update_product_image(
    state: State<AppState>,
    auth_context: AuthContext,
    lang: Lang,
    multipart: SingleFileExtractor,
) -> ApiResponse<FileAttachmentDTO> {
    info!(target: "product::create", "{} trying to UPDATE product image", auth_context.claims.sub);
    if !auth_context.authorize("app::product::write") {
        info!(target: "product::create", "{} not permitted", auth_context.claims.sub);
        return ApiResponse::un_authorized(translate!("unauthorized", lang).as_str());
    }

    let validate = multipart.validate_body();
    if validate.is_err() {
        return ApiResponse::error_validation(
            validate.unwrap_err(),
            translate!("product.product-image.failed", lang).as_str(),
        );
    }

    let user_id = create_object_id_option(multipart.ref_id.as_str());
    if user_id.is_none() {
        info!(target: "product::create", "failed CREATE ObjectId");
        return ApiResponse::not_found(
            translate!("product.product-image.not-found", lang).as_str(),
        );
    }
    let find_exist_profile_picture = Orm::get("file-attachment")
        .filter_object_id("ref_id", &user_id.unwrap())
        .one::<FileAttachment>(&state.db)
        .await;

    let multipart = multipart.file();
    let minio = MinIO::new().await;
    let is_file_exists = find_exist_profile_picture.is_ok();
    let bucket_name = "product-image".to_string();

    let mut attachment = match find_exist_profile_picture {
        Ok(v) => v,
        Err(_) => FileAttachment {
            id: Some(ObjectId::new()),
            ref_id: create_object_id_option(multipart.ref_id.as_str()),
            filename: multipart.filename.clone(),
            mime_type: multipart.mime_type.clone(),
            extension: multipart.extension.clone(),
            kind: "PRODUCT".to_string(),
            create_at: DateTime::now(),
            updated_at: DateTime::now(),
        },
    };

    if is_file_exists {
        info!(target: "product::create", "file exist deleting {}",multipart.filename);
        let _delete_existing = minio
            .delete_file(attachment.filename.clone(), bucket_name.clone())
            .await;
    }

    //upload new
    let minio = minio
        .upload_file(
            multipart.temp_path.clone(),
            bucket_name,
            multipart.filename.clone(),
        )
        .await;

    if minio.is_err() {
        let err = minio.unwrap_err();
        info!(target: "product::update-image", "{}", err);
        let _remove = multipart.remove_file();
        return ApiResponse::failed(translate!("product.product-image.failed", lang).as_str());
    }

    let mut error_message = String::new();
    let success = match is_file_exists {
        true => {
            let update_profile_picture = Orm::update("file-attachment")
                .filter_object_id("ref_id", &user_id.unwrap())
                .set_str("filename", &multipart.filename.as_str())
                .set_str("mime-type", &multipart.mime_type.as_str())
                .set_str("extension", &multipart.extension.as_str())
                .execute_one(&state.db)
                .await;
            if update_profile_picture.is_err() {
                error_message = update_profile_picture.clone().unwrap_err();
            }
            update_profile_picture.is_ok()
        }
        false => {
            attachment.filename = multipart.filename.clone();
            let save_profile_picture = Orm::insert("file-attachment")
                .one(&attachment, &state.db)
                .await;
            if save_profile_picture.is_err() {
                error_message = save_profile_picture.clone().unwrap_err();
            }
            save_profile_picture.is_ok()
        }
    };

    if !success {
        info!(target: "product::update-image", "{}", error_message);
        let _remove = multipart.remove_file();
        return ApiResponse::failed(translate!("product.product-image.failed", lang).as_str());
    }

    let _remove = multipart.remove_file();
    info!(target: "product::update-image", "suucessfully UPDATE product image");
    ApiResponse::ok(
        attachment.to_dto(),
        translate!("product.product-image.success", lang).as_str(),
    )
}
