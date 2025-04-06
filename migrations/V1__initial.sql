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
	-- we will generate a uuid for all users
	uid					uuid NOT NULL PRIMARY KEY UNIQUE,
	
	domain				TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	username			TEXT NOT NULL,
	display_name		TEXT NULL,
	summary				TEXT NULL, -- used as a user's bio
	-- serde encoded json of custom emoji in display name and summary
	custom_emoji 	TEXT NULL,

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

CREATE TABLE signup_token (
	-- these need to be v4 uuids with random content
	token_id		uuid NOT NULL PRIMARY KEY UNIQUE,
	-- the user that created the signup token, useful for auditing
	-- makes sure that if a user is removed their invites are also removed
	creator			uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	-- since these are using uuids that may not be the most secure
	-- we are going to make sure they always have an expiry so it
	-- doesn't stick around for too long
	expiry			BIGINT NOT NULL
);

CREATE TABLE registered_devices (
	device_id		uuid NOT NULL PRIMARY KEY UNIQUE,
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
	-- these need to be v4 uuids with secure random content
	-- this is prob not ideal and will need to be reworked
	token_id		uuid NOT NULL PRIMARY KEY UNIQUE,
	device_id		uuid NOT NULL REFERENCES registered_devices(device_id) ON DELETE CASCADE,
	uid				uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	-- since our mechanisms are not super ideal, we're going to keep
	-- low lifetimes so stick to like 30-90 days
	expiry			BIGINT NOT NULL
);

CREATE TABLE friends (
	-- the user that created the friend request
	creator			uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	-- the user that is being friended
	target_user		uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	pending			BOOLEAN NOT NULL DEFAULT true,
	created			BIGINT NOT NULL,
	PRIMARY KEY(creator, target_user)
);

-- like servers on discord, a group of rooms
CREATE TABLE communities (
	-- all communities will have a generated uuid
	com_id 			UUID NOT NULL PRIMARY KEY UNIQUE,
	-- all communities wll be at a specified endpoint
	external_id		UUID NOT NULL,
	domain			TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	
	name			TEXT NOT NULL,
	description 	TEXT NULL,
	-- custom emoji present in the name and description
	custom_emoji 	TEXT NULL,
	created			BIGINT NOT NULL,

	UNIQUE (external_id, domain)
);

CREATE TABLE join_token (
	-- these need to be v4 uuids with random content
	token_id		uuid NOT NULL PRIMARY KEY UNIQUE,
	-- the user that created the signup token, useful for auditing
	-- makes sure that if a user is removed their invites are also removed
	creator			uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	commmunity		uuid NOT NULL REFERENCES communities(com_id) ON DELETE CASCADE,
	-- since these are using uuids that may not be the most secure
	-- we are going to make sure they always have an expiry so it
	-- doesn't stick around for too long
	expiry			BIGINT NOT NULL
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
	-- all rooms will have a generated uuid
	room_id 	UUID NOT NULL PRIMARY KEY UNIQUE,
	external_id		UUID NOT NULL,
	domain		TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	community	UUID NULL REFERENCES communities(com_id) ON DELETE CASCADE,
	category	UUID NULL REFERENCES categories(cat_id) ON DELETE SET NULL,
	-- groups that are part of a community will be ordered from 
	-- smallest to largest. to reorder, incriment all groups part of
	-- a community that are greater than or equal to the position you
	-- want to move one to and then update the room to be at that position
	display_order	BIGINT NOT NULL DEFAULT 0,
	name			TEXT NOT NULL,
	description 	TEXT NULL,
	created			BIGINT NOT NULL,

	is_dm 		BOOLEAN NOT NULL DEFAULT false,
	user_a		uuid NULL REFERENCES users(uid) ON DELETE CASCADE,
	user_b		uuid NULL REFERENCES users(uid) ON DELETE CASCADE
);

-- used for group chats not part of a community
CREATE TABLE room_membership (
	room_id 	uuid NOT NULL REFERENCES rooms(room_id) ON DELETE CASCADE,
	uid			uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	joined		BIGINT NOT NULL
);

CREATE TABLE messages (
	m_id 		uuid NOT NULL PRIMARY KEY UNIQUE,
	external_id		UUID NOT NULL,
	domain		TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	uid			uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,

	room		uuid NOT NULL REFERENCES rooms(room_id) ON DELETE CASCADE,
	published	BIGINT NOT NULL,

	is_reply	BOOLEAN NOT NULL,
	in_reply_to	uuid NULL REFERENCES messages(m_id) ON DELETE SET NULL,

	content			TEXT NULL,
	-- serde encoded json of custom emoji
	custom_emoji 	TEXT NULL,

	fetched_at		BIGINT NULL,
	UNIQUE (external_id, domain)
);

CREATE TABLE reactions (
	react_id	TEXT NOT NULL UNIQUE,
	uid			uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	m_id 		uuid NOT NULL REFERENCES messages(m_id) ON DELETE CASCADE,
	reaction	TEXT NOT NULL,
	published	BIGINT NOT NULL,
	PRIMARY KEY(uid, m_id, reaction)
);

CREATE TABLE pins (
	uid			uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	m_id 		uuid NOT NULL REFERENCES messages(m_id) ON DELETE CASCADE,
	room_id 	uuid NOT NULL REFERENCES rooms(room_id) ON DELETE CASCADE,
	created		BIGINT NOT NULL,
	PRIMARY KEY(room_id, m_id)
);