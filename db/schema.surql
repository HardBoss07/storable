-- Namespace & DB
DEFINE NAMESPACE storable;
DEFINE DATABASE main;

USE NAMESPACE storable DATABASE main;

--------------------------------------------------
-- USERS
--------------------------------------------------
DEFINE TABLE user SCHEMAFULL;

DEFINE FIELD username ON TABLE user TYPE string;
DEFINE FIELD password_hash ON user TYPE string;
DEFINE FIELD is_admin ON TABLE user TYPE bool DEFAULT false;
DEFINE FIELD created_at ON TABLE user TYPE datetime DEFAULT time::now();

DEFINE INDEX user_username_unique
ON TABLE user
COLUMNS username
UNIQUE;

--------------------------------------------------
-- FILESYSTEM NODES (files + folders)
--------------------------------------------------
DEFINE TABLE node SCHEMAFULL;

DEFINE FIELD owner ON TABLE node TYPE record<user>;
DEFINE FIELD parent ON TABLE node TYPE option<record<node>>;
DEFINE FIELD name ON TABLE node TYPE string;

DEFINE FIELD kind ON TABLE node TYPE string ASSERT $value IN ["file", "folder"];

DEFINE FIELD size ON TABLE node TYPE option<number>;
DEFINE FIELD mime ON TABLE node TYPE option<string>;
DEFINE FIELD storage_key ON TABLE node TYPE option<string>;

DEFINE FIELD created_at ON TABLE node TYPE datetime DEFAULT time::now();
DEFINE FIELD modified_at ON TABLE node TYPE datetime DEFAULT time::now();

-- Prevent duplicate names inside one folder
DEFINE INDEX node_unique_per_parent
ON TABLE node
COLUMNS parent, name
UNIQUE;

-- Fast directory listing
DEFINE INDEX node_parent_idx
ON TABLE node
COLUMNS parent;