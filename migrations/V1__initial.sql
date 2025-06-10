CREATE TABLE instances (
	domain				TEXT NOT NULL PRIMARY KEY UNIQUE,
	--this is the main domain of the instance
	is_authoratative	BOOLEAN NOT NULL DEFAULT false,
	blocked				BOOLEAN NOT NULL DEFAULT false,
	reason				TEXT NULL,
	allowlisted			BOOLEAN NOT NULL DEFAULT false
);

-- in this protocol users will be at a defined endpoint so 
-- we will not need to store a link for them
-- we will not store keys for users either and signing will be
-- the sole responsibility of the instance that is authoratative over them
CREATE TABLE users (
	-- we will generate a UUID for all users
	uid					UUID NOT NULL PRIMARY KEY UNIQUE,
	
	domain				TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	username			TEXT NOT NULL,
	display_name		TEXT NULL,
	summary				TEXT NULL, -- used as a user's bio

	banned				BOOLEAN NOT NULL DEFAULT false,
	reason				TEXT NULL,
	
	fetched_at			BIGINT NULL,

	-- only for users we are authoratative over
	-- all columns below except email should be not null if authoratative
	is_authoratative	BOOLEAN NOT NULL DEFAULT false,
	password			TEXT NULL, 	--stored with argon2
	email				TEXT NULL,

	verified			BOOLEAN NULL,
	is_admin			BOOLEAN NULL,
	instance_mod		BOOLEAN NULL,

	application_message	TEXT NULL,
	application_approved	BOOLEAN NULL,

	created				BIGINT NOT NULL,
	UNIQUE (domain, username)
);

CREATE TABLE proxies (
	proxy_id	UUID NOT NULL PRIMARY KEY UNIQUE,
	uid			UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	proxy_name	TEXT NOT NULL,
	proxy_bio	TEXT NULL
);

CREATE TABLE signup_token (
	-- these need to be v4 UUIDs with random content
	token_id		UUID NOT NULL PRIMARY KEY UNIQUE,
	-- the user that created the signup token, useful for auditing
	-- makes sure that if a user is removed their invites are also removed
	creator			UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	-- since these are using UUIDs that may not be the most secure
	-- we are going to make sure they always have an expiry so it
	-- doesn't stick around for too long
	expiry			BIGINT NOT NULL
);

CREATE TABLE registered_devices (
	device_id		UUID NOT NULL PRIMARY KEY UNIQUE,
	device_name		TEXT NULL,
	software		TEXT NULL,
	webpage			TEXT NULL,
	redirect_url	TEXT NULL,
	registered_at	BIGINT NOT NULL
);

-- auth flow is still very wip, expect this to change
-- to be better in line with oath 2.0 as things are stabalized
-- no scopes for the time being, need to introduce more granularity 
CREATE TABLE auth_tokens (
	-- these need to be v4 UUIDs with secure random content
	-- this is prob not ideal and will need to be reworked
	token_id		UUID NOT NULL PRIMARY KEY UNIQUE,
	device_id		UUID NOT NULL REFERENCES registered_devices(device_id) ON DELETE CASCADE,
	uid				UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	-- since our mechanisms are not super ideal, we're going to keep
	-- low lifetimes so stick to like 30-90 days
	expiry			BIGINT NOT NULL
);

CREATE TABLE friends (
	-- the user that created the friend request
	creator			UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	-- the user that is being friended
	target_user		UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	pending			BOOLEAN NOT NULL DEFAULT true,
	created			BIGINT NOT NULL,
	PRIMARY KEY(creator, target_user)
);

-- like servers on discord, a group of rooms
CREATE TABLE communities (
	-- all communities will have an internal generated UUID
	com_id 			UUID NOT NULL PRIMARY KEY UNIQUE,
	-- all communities wll be at a specified endpoint
	external_id		UUID NOT NULL,
	-- domain of the community owner
	domain			TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	owner			UUID NOT NULL REFERENCES users(uid) ON DELETE SET NULL,
	
	name			TEXT NOT NULL,
	description 	TEXT NULL,
	created			BIGINT NOT NULL,

	UNIQUE (external_id, domain)
);

CREATE TABLE join_token (
	-- these need to be v4 UUIDs with random content
	token_id		UUID NOT NULL PRIMARY KEY UNIQUE,
	-- the user that created the signup token, useful for auditing
	-- makes sure that if a user is removed their invites are also removed
	creator			UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	com_id			UUID NOT NULL REFERENCES communities(com_id) ON DELETE CASCADE,
	-- since these are using UUIDs that may not be the most secure
	-- we are going to make sure they always have an expiry so it
	-- doesn't stick around for too long
	expiry			BIGINT NOT NULL
);

-- todo 
-- - create trigger on delete to check if a community no longer
-- has members and to then delete it if so
CREATE TABLE community_membership (
	com_id		UUID NOT NULL REFERENCES communities(com_id) ON DELETE CASCADE,
	uid			UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	joined		BIGINT NOT NULL,
	PRIMARY KEY(com_id, uid)
);

-- used to group rooms
CREATE TABLE categories (
	cat_id 		UUID NOT NULL PRIMARY KEY UNIQUE,
	community	UUID NOT NULL REFERENCES communities(com_id) ON DELETE CASCADE,
	name		TEXT NOT NULL,
	UNIQUE (community, name)
);

-- messages are in rooms. rooms can be direct messages, group chats, or part of a community
CREATE TABLE rooms (
	-- all rooms will have a generated UUID
	room_id 	UUID NOT NULL PRIMARY KEY UNIQUE,
	external_id		UUID NOT NULL,
	domain		TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	community	UUID NULL REFERENCES communities(com_id) ON DELETE CASCADE,
	system_channel	BOOLEAN NOT NULL,
	created			BIGINT NOT NULL,
	-- allows for lazy loading of federated rooms to prevent undue strain
	known_complete	BOOLEAN NOT NULL,

	is_dm 		BOOLEAN NOT NULL DEFAULT false,
	user_a		UUID NULL REFERENCES users(uid) ON DELETE CASCADE,
	user_b		UUID NULL REFERENCES users(uid) ON DELETE CASCADE,

	name			TEXT NOT NULL,
	description 	TEXT NULL,
	category	UUID NULL REFERENCES categories(cat_id) ON DELETE SET NULL,
	-- groups that are part of a community will be ordered from 
	-- smallest to largest. to reorder, incriment all groups part of
	-- a community that are greater than or equal to the position you
	-- want to move one to and then update the room to be at that position
	display_order	BIGINT NOT NULL DEFAULT 0	
);

CREATE OR REPLACE FUNCTION room_integrity() RETURNS TRIGGER AS
$$
BEGIN
	-- only one system channel integrity
	IF (NEW.system_channel = true AND OLD.system_channel = false) THEN
		UPDATE rooms SET system_channel = false 
		WHERE community = NEW.community;
	END IF;

	-- shift down room display orders to make room for new order when inserting
	IF (NEW.display_order <> OLD.display_order) THEN
		UPDATE rooms 
		SET display_order = display_order + 1 
		WHERE community = NEW.community 
			AND display_order >= NEW.display_order
			AND room_id <> NEW.room_id;
	END IF;

	RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_room_integrity
    BEFORE UPDATE ON rooms
    FOR EACH ROW
    EXECUTE FUNCTION room_integrity();

-- used for group chats not part of a community
-- todo, create on delete trigger to delete room if no more memberships exist for room
CREATE TABLE room_membership (
	room_id 	UUID NOT NULL REFERENCES rooms(room_id) ON DELETE CASCADE,
	uid			UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	joined		BIGINT NOT NULL,
	PRIMARY KEY(room_id, uid)
);

CREATE TABLE messages (
	m_id 		UUID NOT NULL PRIMARY KEY UNIQUE,
	external_id		UUID NOT NULL,
	domain		TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	uid			UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE,

	room_id		UUID NOT NULL REFERENCES rooms(room_id) ON DELETE CASCADE,
	published	BIGINT NOT NULL,
	edited		BIGINT NULL,
	fetched_at	BIGINT NULL,

	is_reply	BOOLEAN NOT NULL,
	in_reply_to	UUID NULL REFERENCES messages(m_id) ON DELETE SET NULL,

	content		TEXT NOT NULL,
	format		TEXT NOT NULL,
	language	TEXT NULL,
	proxy_id	UUID NULL REFERENCES proxies(proxy_id) ON DELETE SET NULL,
	
	UNIQUE (external_id, domain)
);

CREATE TABLE files (
	file_id		UUID NOT NULL PRIMARY KEY UNIQUE,
	description	TEXT NULL,
	-- nullable in case files are not owned by a user
	uid			UUID NULL REFERENCES users(uid) ON DELETE CASCADE,
	-- todo surtypes and sutypes for mime types
	path 		TEXT NOT NULL
);

CREATE TABLE emoji_packs (
	emoji_pack_id	UUID NOT NULL PRIMARY KEY UNIQUE,
	emoji_pack_external_id TEXT NOT NULL,

	-- domain is nullable as emoji may be hosted by static sites
	domain		TEXT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	-- website should be the same as domain when emoji are from a domain
	website		TEXT NOT NULL,
	preview		UUID NOT NULL,
	license		TEXT NULL,
	-- last time a pack was modified in the database 
	-- used to tell when out of sync with external packs
	last_updated	BIGINT NOT NULL,
	-- for packs created by a user on an instance 
	uid			UUID NULL REFERENCES users(uid) ON DELETE CASCADE,
	UNIQUE (emoji_pack_external_id, website)
);

CREATE TABLE custom_emoji (
	emoji_id		UUID NOT NULL PRIMARY KEY UNIQUE,
	emoji_pack_id	UUID NOT NULL REFERENCES emoji_packs(emoji_pack_id) ON DELETE CASCADE,
	file_id			UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE
);

CREATE TABLE reactions (
	-- react_id	TEXT NOT NULL UNIQUE,
	uid			UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	m_id 		UUID NOT NULL REFERENCES messages(m_id) ON DELETE CASCADE,
	-- the unicode emoji of the reaction or the shortcode of the custom emoji
	react_unicode	TEXT NOT NULL,
	-- if the emoji is deleted we will just display the old shortcode, also useful
	-- for if the emoji fails to federate
	emoji_id	UUID NULL REFERENCES custom_emoji(emoji_id) ON DELETE SET NULL,
	-- used to ensure that reactions are still unique even if the emoji is deleted
	-- or fails to federate
	archive_id	UUID NULL, 
	published	BIGINT NOT NULL,
	UNIQUE NULLS NOT DISTINCT (uid, m_id, react_unicode, archive_id)
);

CREATE TABLE embedded_emoji (
	m_id 		UUID NULL REFERENCES messages(m_id) ON DELETE CASCADE,
	-- emoji present in display name or bio
	uid			UUID NULL REFERENCES users(uid) ON DELETE CASCADE,
	proxy_id	UUID NULL REFERENCES proxies(proxy_id) ON DELETE CASCADE,
	room_id		UUID NULL REFERENCES rooms(room_id) ON DELETE CASCADE,
	community	UUID NULL REFERENCES communities(com_id) ON DELETE CASCADE,
	-- allow clients to set shortcodes so they may differentiate multiple
	-- emoji with the same name
	shortcode	TEXT NOT NULL,
	emoji_id	UUID NOT NULL REFERENCES custom_emoji(emoji_id) ON DELETE CASCADE
);

CREATE TABLE pins (
	uid			UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	m_id 		UUID NOT NULL REFERENCES messages(m_id) ON DELETE CASCADE,
	room_id 	UUID NOT NULL REFERENCES rooms(room_id) ON DELETE CASCADE,
	created		BIGINT NOT NULL,
	PRIMARY KEY(room_id, m_id)
);