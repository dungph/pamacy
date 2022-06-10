-- public.customer definition

-- Drop table

-- DROP TABLE public.customer;

CREATE TABLE public.customer (
    customer_id serial4 NOT NULL,
    customer_name text NOT NULL,
    customer_phone text NOT NULL,
    customer_address text NOT NULL,
    CONSTRAINT customer_pkey PRIMARY KEY (customer_id)
);


-- public."location" definition

-- Drop table

-- DROP TABLE public."location";

CREATE TABLE public."location" (
    location_id serial4 NOT NULL,
    location_name text NOT NULL,
    CONSTRAINT location_pkey PRIMARY KEY (location_id)
);


-- public.staff definition

-- Drop table

-- DROP TABLE public.staff;

CREATE TABLE public.staff (
    staff_id serial4 NOT NULL,
    staff_name text NOT NULL,
    staff_username text NOT NULL UNIQUE,
    staff_password text NOT NULL,
    CONSTRAINT staff_pkey PRIMARY KEY (staff_id)
);


-- public.bill definition

-- Drop table

-- DROP TABLE public.bill;

CREATE TABLE public.bill (
    bill_id serial4 NOT NULL,
    bill_time timestamptz NOT NULL DEFAULT now(),
    bill_prescripted boolean NOT NULL default false,
    staff_id int4 NOT NULL,
    customer_id int4 NOT NULL,
    CONSTRAINT bill_pkey PRIMARY KEY (bill_id),
    CONSTRAINT bill_customer_id_fkey FOREIGN KEY (customer_id) REFERENCES public.customer(customer_id),
    CONSTRAINT bill_staff_id_fkey FOREIGN KEY (staff_id) REFERENCES public.staff(staff_id)
);


-- public.medicine definition

-- Drop table

-- DROP TABLE public.medicine;

CREATE TABLE public.medicine (
    medicine_id serial4 NOT NULL,
    medicine_name text NOT NULL,
    medicine_type text NOT NULL,
    medicine_price int4 NOT NULL,
    medicine_expire_date timestamptz NOT NULL default now(),
    medicine_import_date timestamptz NOT NULL,
    medicine_quantity int4 NOT NULL,
    medicine_location_id int4 NOT NULL,
    medicine_prescripted boolean NOT NULL default false,
    CONSTRAINT medicine_pkey PRIMARY KEY (medicine_id),
    CONSTRAINT medicine_medicine_location_id_fkey FOREIGN KEY (medicine_location_id) REFERENCES public."location"(location_id)
);


-- public.bill_medicine definition

-- Drop table

-- DROP TABLE public.bill_medicine;

CREATE TABLE public.bill_medicine (
    bill_id int4 NOT NULL,
    medicine_id int4 NOT NULL,
    medicine_bill_price int4 NOT NULL,
    medicine_bill_quantity int4 NOT NULL,
    CONSTRAINT bill_medicine_bill_id_fkey FOREIGN KEY (bill_id) REFERENCES public.bill(bill_id),
    CONSTRAINT bill_medicine_medicine_id_fkey FOREIGN KEY (medicine_id) REFERENCES public.medicine(medicine_id)
);
