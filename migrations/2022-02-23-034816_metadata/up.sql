-- Your SQL goes here
CREATE TABLE video_metadatas (
    id SERIAL PRIMARY KEY,

    video_duration numeric null,
    video_width numeric null,
    video_height numeric null,
    video_codec varchar null,
    audio_track_id numeric null,
    audio_codec varchar null,
    media_item_id UUID NOT NULL references media_data(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE image_metadatas (
    id SERIAL PRIMARY KEY,

    exif_version decimal null,
    x_pixel_dimension int null,
    y_pixel_dimension int null,
    x_resolution int null,
    y_resolution int null,
    date_of_image timestamp null,
    flash boolean null,
    make varchar null,
    model varchar null,
    exposure_time varchar null,
    f_number varchar null,
    aperture_value numeric null,
    location geography(point, 4326) not null,
    altitude numeric null,
    speed numeric null,
    media_item_id UUID NOT NULL references media_data(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

