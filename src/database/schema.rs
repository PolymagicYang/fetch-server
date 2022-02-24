use diesel_derive_enum::DbEnum;
use serde::Deserialize;

table! {
    use diesel::sql_types::{Uuid, Varchar, Bool, Int4, Timestamp};
    use super::{MediaEnumMapping, MediaAudienceTypeMapping, LocationEnumMapping};
    media_data (id) {
        id -> Uuid,
        name -> Varchar,
        location -> Varchar,
        location_type -> LocationEnumMapping,
        media_type -> MediaEnumMapping,
        media_audience_type -> MediaAudienceTypeMapping, 
        published -> Bool,
        size -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        device_id -> Uuid,
    }
}


#[derive(DbEnum, Debug, Eq, PartialEq, Clone, Deserialize)]
pub enum MediaEnum {
    Image,
    Video,
    Music,
}

#[derive(DbEnum, Debug, Eq, PartialEq, Clone, Deserialize)]
pub enum LocationEnum {
    S3,
    Local, 
}

#[derive(DbEnum, Debug, Eq, PartialEq, Clone, Deserialize)]
pub enum MediaAudienceType {
    Personal,
    Friends,
    Family,
}

table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        media_item_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    health_checks (id) {
        id -> Int4,
        device_id -> Uuid,
        data -> Json,
        userid -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(comments -> media_data (media_item_id));


table! {
    image_metadatas (id) {
        id -> Int4,
        exif_version -> Nullable<Numeric>,
        x_pixel_dimension -> Nullable<Int4>,
        y_pixel_dimension -> Nullable<Int4>,
        x_resolution -> Nullable<Int4>,
        y_resolution -> Nullable<Int4>,
        date_of_image -> Nullable<Timestamp>,
        flash -> Nullable<Bool>,
        make -> Nullable<Varchar>,
        model -> Nullable<Varchar>,
        exposure_time -> Nullable<Varchar>,
        f_number -> Nullable<Varchar>,
        aperture_value -> Nullable<Numeric>,
        location -> String,
        altitude -> Nullable<Numeric>,
        speed -> Nullable<Numeric>,
        media_item_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        srtext -> Nullable<Varchar>,
        proj4text -> Nullable<Varchar>,
    }
}

table! {
    video_metadatas (id) {
        id -> Int4,
        video_duration -> Nullable<Numeric>,
        video_width -> Nullable<Numeric>,
        video_height -> Nullable<Numeric>,
        video_codec -> Nullable<Varchar>,
        audio_track_id -> Nullable<Numeric>,
        audio_codec -> Nullable<Varchar>,
        media_item_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(image_metadatas -> media_data (media_item_id));
joinable!(video_metadatas -> media_data (media_item_id));

allow_tables_to_appear_in_same_query!(
    comments,
    health_checks,
    image_metadatas,
    media_data,
    spatial_ref_sys,
    video_metadatas,
);
