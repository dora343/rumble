create table if not exists gamble.users (
	id BIGINT primary key,
	tokens BIGINT default 0,
	rate smallint default 5000,
	crit_rate smallint default 0,
	crit_mul INTEGER default 10000,
	revive_tokens BIGINT default 10000,
	auto_revive boolean default FALSE
);
