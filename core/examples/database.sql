create temporary table serie(
    n serial primary key
);

insert into serie select * from generate_series(1, 10);
