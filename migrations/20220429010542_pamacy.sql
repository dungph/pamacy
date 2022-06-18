create table supplier (
    supplier_id serial primary key,
    supplier_name text
);

create table staff (
    staff_id serial primary key,
    staff_info text,
    staff_is_manager boolean,
    staff_is_seller boolean,
    staff_username text,
    staff_password text
);

create table medicine_info (
    medicine_code text primary key,
    medicine_name text,
    medicine_price integer,
    medicine_register text,
    medicine_content text,
    medicine_active_ingredients text,
    medicine_pack_form text,
    medicine_group text,
    medicine_route text,
    medicine_locations text
);

create table medicine (
    medicine_id serial primary key,
    medicine_expire_date timestamptz,
    medicine_code text references medicine_info(medicine_code),
    supplier_id integer references supplier(supplier_id)
);

create table inventory_bill(
    inventory_bill_id serial primary key, 
    inventory_bill_time timestamptz,
    inventory_bill_complete boolean,
    staff_id integer references staff(staff_id)
);

create table medicine_inventory_bill (
    inventory_bill_id  integer references inventory_bill(inventory_bill_id),
    medicine_id integer references medicine(medicine_id),
    medicine_inventory_price integer,
    medicine_inventory_quantity integer
);

create table customer (
    customer_id serial primary key, 
    customer_name text, 
    customer_age text,
    customer_gender text,
    customer_phone text, 
    customer_address text
);

create table sell_bill (
    sell_bill_id serial primary key, 
    sell_bill_receive integer, 
    sraff_id integer references staff(staff_id), 
    inventory_bill_id integer references inventory_bill(inventory_bill_id), 
    discout integer,
    customer_id integer references customer(customer_id),
    customer_citizen_id text,
    is_prescripted boolean check ((is_prescripted = false) OR customer_citizen_id is not null)
);
