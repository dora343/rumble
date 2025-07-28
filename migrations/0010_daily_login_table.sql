create table if not exists gamble.daily_login (
	id BIGINT primary key references gamble.users(id) not null,
	login_combo INT default 0 not null,
	buff_remaining_rounds INT default 0 not null,
	last_login TIMESTAMPTZ default '2025-01-01 00:00:00+08:00'
);

-- port users id to this table
insert into gamble.daily_login   
select id from gamble.users;
