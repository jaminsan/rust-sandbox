create table purchase
(
    purchase_id text not null,
    customer_id text not null,
    primary key (purchase_id)
);

create table purchase_item
(
    purchase_id text    not null references purchase (purchase_id),
    item_id     text    not null references item_stock(item_id),
    quantity    integer not null
);