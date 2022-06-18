create table staff (
    staff_username text primary key,
    staff_info text not null default '',
    staff_is_manager boolean not null default false,
    staff_is_seller boolean not null default false,
    staff_fullname text not null default '',
    staff_password text not null default ''
);

create table medicine_info (
    medicine_code text primary key,
    medicine_name text not null default '',
    medicine_price integer not null default 0,
    medicine_register text not null default '',
    medicine_content text not null default '',
    medicine_active_ingredients text not null default '',
    medicine_prescripted boolean not null default false,
    medicine_pack_form text not null default '',
    medicine_group text not null default '',
    medicine_route text not null default '',
    medicine_locations text not null default ''
);

create table medicine (
    medicine_id serial primary key,
    medicine_expire_date timestamptz not null default now(),
    medicine_code text references medicine_info(medicine_code) not null,
    medicine_supplier text not null default ''
);

create table inventory_bill(
    inventory_bill_id serial primary key, 
    inventory_bill_time timestamptz not null default now(),
    inventory_bill_complete boolean not null default false,
    staff_username text references staff(staff_username) not null
);

create table medicine_inventory_bill (
    inventory_bill_id  integer references inventory_bill(inventory_bill_id),
    medicine_id integer references medicine(medicine_id),
    medicine_inventory_price integer not null default 0,
    medicine_inventory_quantity integer not null
);

create table customer (
    customer_id serial primary key, 
    customer_name text not null default '', 
    customer_age text not null default '',
    customer_gender text not null default '',
    customer_phone text not null default '', 
    customer_citizen_id text,
    customer_address text not null default ''
);

create table sell_bill (
    sell_bill_id serial primary key, 
    staff_username text references staff(staff_username), 
    inventory_bill_id integer references inventory_bill(inventory_bill_id), 
    discout integer not null default 0,
    customer_id integer references customer(customer_id),
    is_prescripted boolean
);
