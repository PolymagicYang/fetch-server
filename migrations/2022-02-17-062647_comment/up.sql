-- Your SQL goes here
CREATE TYPE MEDIA_ENUM AS ENUM ('music', 'audio', 'video');
CREATE TYPE LOCATION_ENUM AS ENUM ('s3', 'local');
CREATE TYPE MEDIA_AUDIENCE_TYPE AS ENUM ('personal', 'friends', 'family');

CREATE TABLE health_checks (
    id SERIAL PRIMARY KEY, 
    device_id UUID NOT NULL,
    data Json NOT NULL,
    userid UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE media_data (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    note VARCHAR,
    media_type MEDIA_ENUM NOT NULL,
    location VARCHAR NOT NULL,
    location_type LOCATION_ENUM NOT NULL,
    media_audience_type MEDIA_AUDIENCE_TYPE NOT NULL,
    published BOOLEAN NOT NULL,
    size int NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    device_id UUID NOT NULL
);

CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    body TEXT NOT NULL,
    media_item_id UUID NOT NULL references media_data(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


