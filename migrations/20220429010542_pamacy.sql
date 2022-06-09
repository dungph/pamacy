-- Add migration script here
create table location(
    location_id             serial primary key,
    location_name           text
);

create table medicine (
    medicine_id             serial primary key,
    medicine_name           text,
    medicine_type           text,
    medicine_price          integer,
    medicine_expire_date    timestamptz,
    medicine_import_date    timestamptz,
    medicine_location_id    integer references location(location_id)
);

create table quantity (
    medicine_id 	        integer references medicine(medicine_id),
    medicine_quantity 	    integer
);

create table staff (
    staff_id 	            serial primary key,
    staff_name 	            text,
    staff_username 	        text,
    staff_password 	        text
);

create table customer (
    customer_id 	        serial primary key,
    customer_name 	        text,
    customer_phone 	        text,
    customer_address 	    text
);

create table bill (
    bill_id 	            serial primary key,
    bill_time 	            timestamptz default now(),
    staff_id 	            integer references staff(staff_id),
    customer_id 	        integer references customer(customer_id)
);

create table bill_medicine (
    bill_id 	            integer references bill(bill_id),
    medicine_id 	        integer references medicine(medicine_id),
    medicine_bill_price 	integer,
    medicine_bill_quantity 	integer
);
