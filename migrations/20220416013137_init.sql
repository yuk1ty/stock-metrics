create table stock (
	id varchar(36) primary key,
	name varchar(255) not null,
	ticker_symbol varchar(255) not null,
	market_kind varchar(36) not null,
	created_at datetime not null,
	updated_at datetime not null
) character set utf8mb4 collate utf8mb4_bin;

create table market_kind (
	id varchar(36) primary key,
	code varchar(36) not null,
	name varchar(255) not null,
	created_at datetime not null,
	updated_at datetime not null
) character set utf8mb4 collate utf8mb4_bin;