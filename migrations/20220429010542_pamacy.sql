CREATE TABLE public.staff (
    staff_username text PRIMARY KEY,
    staff_password text NOT NULL,
    staff_fullname text NOT NULL
);

CREATE TABLE public.bill (
    bill_id serial4 PRIMARY KEY,
    bill_time timestamptz NOT NULL DEFAULT now(),
    bill_prescripted boolean NOT NULL default false,
    bill_done boolean NOT NULL default false,
    customer_phone text NOT NULL,
    customer_name text NOT NULL,
    customer_address text NOT NULL,
    staff_username text NOT NULL REFERENCES staff(staff_username)
);

CREATE TABLE public.medicine (
    medicine_id serial4 PRIMARY KEY,
    medicine_name text NOT NULL,
    medicine_type text NOT NULL,
    medicine_prescripted boolean NOT NULL default false,
    medicine_price int4 NOT NULL,
    medicine_expire_date timestamptz NOT NULL default now(),
    medicine_import_date timestamptz NOT NULL,
    medicine_quantity int4 NOT NULL,
    medicine_location text NOT NULL
);

CREATE TABLE public.medicine_bill (
    bill_id int4 NOT NULL REFERENCES bill(bill_id),
    medicine_id int4 NOT NULL REFERENCES medicine(medicine_id),
    medicine_bill_price int4 NOT NULL,
    medicine_bill_quantity int4 NOT NULL
);
