-- Table: public.devices
-- DROP TABLE IF EXISTS public.devices;

CREATE TABLE IF NOT EXISTS public.devices
(
    device_id character varying(36) COLLATE pg_catalog."default" NOT NULL DEFAULT (uuid_generate_v1())::text,
    name character varying(200) COLLATE pg_catalog."default" NOT NULL,
    registered_at timestamp without time zone NOT NULL DEFAULT (now())::timestamp without time zone,
    status character varying(30),
    CONSTRAINT devices_pkey PRIMARY KEY (device_id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.devices OWNER to admin;
ALTER TABLE IF EXISTS public.devices OWNER to api;
GRANT ALL ON TABLE public.devices TO admin;
GRANT ALL ON TABLE public.devices TO api;

-- Load data for testing

INSERT INTO public.devices(name) VALUES ('Arduino UNI');
INSERT INTO public.devices(name, status) VALUES ('Arduino Nano', 'RUNING');