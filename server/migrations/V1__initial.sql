CREATE TABLE instances (
	domain				TEXT NOT NULL PRIMARY KEY UNIQUE,
	--this is the main domain of the instance
	is_primary			BOOLEAN NOT NULL DEFAULT false,
	--we will support multiple domains and if we are
	--also authoratative over a dmain it will be true
	is_authoratative	BOOLEAN NOT NULL DEFAULT false,
	blocked				BOOLEAN NOT NULL DEFAULT false,
	allowlisted			BOOLEAN NOT NULL DEFAULT false,
	protocol			TEXT NULL,
	favicon				BYTEA NULL
);

CREATE TABLE users (
	-- we will generate a uuid for all users
	uid					uuid NOT NULL PRIMARY KEY UNIQUE,
	-- this is the id field of activitypub and the url for versia
	resource_link		TEXT NOT NULL UNIQUE,
	-- this will just be the resource link for ap users
	versia_id			TEXT NOT NULL,
	-- used for the actual webpage for the user not the versia url
	url					TEXT NOT NULL,
	domain				TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	username			TEXT NOT NULL,
	display_name		TEXT NULL,
	summary				TEXT NULL, -- used as a user's bio
	public_key_pem		TEXT NOT NULL,
	public_key_id		TEXT NOT NULL,
	manual_followers	BOOLEAN NOT NULL DEFAULT false, -- manually approves followers

	banned				BOOLEAN NOT NULL DEFAULT false,
	reason				TEXT NULL,

	-- links
	inbox				TEXT NOT NULL,
	outbox				TEXT NOT NULL,
	followers			TEXT NOT NULL,
	following			TEXT NOT NULL,
	--only for users we are authoratative over
	password			TEXT NULL, 	--stored with argon2
	email				TEXT NULL,
	private_key_pem		TEXT NULL,
	permission_level 	SMALLINT NULL,

	UNIQUE (domain, username)
);

CREATE TABLE ap_instance_actor (
	private_key_pem		TEXT NOT NULL,
	public_key_pem		TEXT NOT NULL,
	algorithm			TEXT NOT NULL
);

CREATE TABLE following (
	-- the user that is following
	follower		uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	-- the user that is being followed
	target_user		uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	pending			BOOLEAN NOT NULL DEFAULT true,
	published		BIGINT NOT NULL,
	PRIMARY KEY(follower, target_user)
);

-- like servers on discord, a group of groups
CREATE TABLE communities (
	-- all communities will have a generated uuid
	com_id 		TEXT NOT NULL PRIMARY KEY UNIQUE,
	url			TEXT NOT NULL UNIQUE,
	-- the uuid of the community
	id			TEXT NOT NULL,
	domain		TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	-- link to collection of members and groups
	members		TEXT NOT NULL UNIQUE,
	groups		TEXT NOT NULL UNIQUE,
	-- name and description hold the json text content format
	name		TEXT NULL,
	description TEXT NULL,
	UNIQUE (domain, id)
);

-- groups will be used for messaging like discord channels
CREATE TABLE groups (
	-- all groups will have a generated uuid
	group_id 	TEXT NOT NULL PRIMARY KEY UNIQUE,
	url			TEXT NOT NULL UNIQUE,
	-- the uuid of the group
	id			TEXT NOT NULL,
	domain		TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,
	community	TEXT NULL REFERENCES communities(com_id) ON DELETE CASCADE,
	-- groups that are part of a community will be ordered from 
	-- smallest to largest. to reorder, incriment all groups part of
	-- a community that are greater than or equal to the position you
	-- want to move one to and then update the group to be at that position
	display_order	BIGINT NOT NULL DEFAULT 0,
	-- link to collection of members and notes
	members		TEXT NOT NULL UNIQUE,
	notes		TEXT NULL UNIQUE,
	-- name and description hold the json text content format
	name		TEXT NULL,
	description TEXT NULL,
	UNIQUE (domain, id)
);

CREATE TABLE posts (
	-- pid is generated locally and used by the api to 
	-- fetch user posts
	pid 		uuid NOT NULL PRIMARY KEY UNIQUE,
	-- uses the versia url
	id			TEXT NOT NULL UNIQUE,
	-- uses the activitypub id if activitypub
	versia_id	TEXT NOT NULL,
	domain		TEXT NOT NULL REFERENCES instances(domain) ON DELETE CASCADE,

	surtype		TEXT NOT NULL,
	subtype		TEXT NOT NULL,
	category	TEXT NOT NULL,

	likes		BIGINT NOT NULL DEFAULT 0,
	boosts		BIGINT NOT NULL DEFAULT 0,
	reactions	TEXT NULL,

	local_only	BOOLEAN NOT NULL DEFAULT false,
	followers_only	BOOLEAN NOT NULL DEFAULT false,
	in_group		TEXT NULL REFERENCES groups(group_id) ON DELETE CASCADE,
	published	BIGINT NOT NULL,

	-- does not use a constraint as its prob better not to 
	-- alter the post if another post is deleted and it would
	-- prevent inserting replies to a post that failed to federate
	-- we may decide to go back to enforcing it at some point but
	-- for now, for simplicity's sake we'll just do this	
	in_reply_to	TEXT NULL,
	
	-- need to iron this out but something of the sort is planned
	block_replies BOOLEAN NOT NULL DEFAULT false,
	restrict_replies BOOLEAN NOT NULL DEFAULT false, --only those followed by or mentoned by the creator can comment
	local_only_replies BOOLEAN NOT NULL DEFAULT false,

	content		TEXT NULL,
	-- used for questions
	multi_select 		BOOLEAN NULL,
	options				TEXT NULL, -- the array of json options in text
	closed				BIGINT NULL,
	local_only_voting 	BOOLEAN NULL,

	actor	uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE
);

CREATE TABLE likes (
	-- uses the id from versia or just slap in the id url from ap
	-- needs to be here for versia compatibility
	id			TEXT NOT NULL,
	url			TEXT NOT NULL UNIQUE,
	actor		uuid NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
	post 		TEXT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
	published	BIGINT NOT NULL,
	PRIMARY KEY(actor, post)
);
