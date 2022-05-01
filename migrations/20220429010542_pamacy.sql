-- Add migration script here
create table location(
    location_id serial primary key, 
    location_name text 
);
create table medicine_warehouse_bill (
    medicine_bill_id serial primary key, 
    warehouse_bill_id  integer,
    medicine_id integer,
    medicine_quantity integer
);

create table warehouse_bill(
    warehouse_bill_id serial primary key, 
    warehouse_bill_time timestamp, 
    sraff_id integer, 
    warehouse_is_import_bill boolean
);

create table staff(
    staff_id serial primary key, 
    staff_into text
);
create table supplier (
    supplier_id serial primary key,
    supplier_name text
);
create table medicine_info(
    medicine_code text primary key, 
    medicine_name text, 
    medicine_content text, 
    medicine_element text, 
    medicine_group text
);
create table medicine(
    medicine_id serial primary key, 
    medicine_expire_date timestamp, 
    medicine_price integer,
    medicine_code text, 
    supplier_id integer
);
 create table medicine_location(
     medicine_location_id serial primary key, 
     medicine_id integer, 
     location_id integer
 );

 create table medicine_quantity(
     medicine_quantity_id serial primary key, 
     medicine_id integer, 
     quantity integer
 ); 

create table pos_bill(
    pos_bill_id serial primary key, 
    pos_bill_time timestamp, 
    pos_bill_receive integer, 
    sraff_id integer, 
    pos_is_sell_bill boolean, 
    warehouse_bill_id integer, 
    finance_bill_id integer, 
    customer_id integer
);

create table customer(
    customer_id serial primary key, 
    customer_name text, 
    customer_phone text, 
    customer_address text
);

create table finance_bill(
    finance_bill_id serial primary key, 
    finance_bill_time timestamp, 
    finance_bill_change integer, 
    finance_is_spend_bill boolean, 
    finance_bill_content text, 
    staff_id integer
);
