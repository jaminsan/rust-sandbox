create table item_stock
(
    item_id  text    not null,
    quantity integer not null,
    primary key (item_id)
);

insert into item_stock values ('01FD7SB2YW5X36A5Q267N7DAS9', 10);