-- Add weight column to rating_system_parameter for weighted calculations
ALTER TABLE rating_system_parameter
    ADD COLUMN weight NUMERIC(4,2) DEFAULT 1.0 NOT NULL;
