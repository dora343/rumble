create table if not exists gallery.album (
	id TEXT primary key not null,
	image_key TEXT not null,
	user_id BIGINT not null
);
