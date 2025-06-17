create table if not exists gamble.records (
	id SERIAL primary key,
	user_id BIGINT references gamble.users(id),
	play_count INTEGER not null,
	bet BIGINT not null,
	success boolean not null,
	is_crit boolean not null,
	rate smallint not null,
	crit_rate smallint not null,
	tokens_before BIGINT default 0,
	tokens_after BIGINT default 0,
	time_stamp TIMESTAMP default Now()
);
