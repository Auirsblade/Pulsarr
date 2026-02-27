-- Rating System Templates: reusable blueprints for rating systems
-- Each group gets its own independent rating_system instance, optionally cloned from a template

-- Template tables
CREATE TABLE rating_system_template (
    template_id       SERIAL PRIMARY KEY,
    name              TEXT NOT NULL,
    master_rating_type TEXT NOT NULL,
    rating_max        NUMERIC(8,3) NOT NULL
);

CREATE TABLE rating_system_template_parameter (
    template_parameter_id  SERIAL PRIMARY KEY,
    template_id            INTEGER NOT NULL
        REFERENCES rating_system_template(template_id) ON DELETE CASCADE,
    name                   TEXT NOT NULL,
    parameter_rating_max   NUMERIC(8,3) NOT NULL,
    weight                 NUMERIC(4,2) NOT NULL DEFAULT 1.0
);

-- Add optional FK from rating_system back to its source template
ALTER TABLE rating_system
    ADD COLUMN template_id INTEGER
    REFERENCES rating_system_template(template_id) ON DELETE SET NULL;

-- Seed templates from the two built-in default rating systems
INSERT INTO rating_system_template (name, master_rating_type, rating_max)
SELECT name, master_rating_type, rating_max
FROM rating_system WHERE rating_system_id IN (-1, -2);

INSERT INTO rating_system_template_parameter (template_id, name, parameter_rating_max, weight)
SELECT t.template_id, rsp.name, rsp.parameter_rating_max, rsp.weight
FROM rating_system_parameter rsp
JOIN rating_system rs ON rs.rating_system_id = rsp.rating_system_id
JOIN rating_system_template t ON t.name = rs.name AND t.master_rating_type = rs.master_rating_type
WHERE rs.rating_system_id IN (-1, -2);

-- Clone shared rating systems so each group has its own independent copy
DO $$
DECLARE
    grp RECORD;
    new_rs_id INTEGER;
    src_template_id INTEGER;
BEGIN
    FOR grp IN
        SELECT pg.pulsarr_group_id, pg.rating_system_id
        FROM pulsarr_group pg
        WHERE pg.rating_system_id IN (
            SELECT rating_system_id FROM pulsarr_group
            GROUP BY rating_system_id HAVING COUNT(*) > 1
        ) OR pg.rating_system_id < 0
    LOOP
        -- Find matching template (if this was a default system)
        SELECT t.template_id INTO src_template_id
        FROM rating_system_template t
        JOIN rating_system rs ON rs.name = t.name
            AND rs.master_rating_type = t.master_rating_type
        WHERE rs.rating_system_id = grp.rating_system_id
        LIMIT 1;

        INSERT INTO rating_system (master_rating_type, rating_max, name, template_id)
        SELECT master_rating_type, rating_max, name, src_template_id
        FROM rating_system WHERE rating_system_id = grp.rating_system_id
        RETURNING rating_system_id INTO new_rs_id;

        INSERT INTO rating_system_parameter (rating_system_id, name, parameter_rating_max, weight)
        SELECT new_rs_id, name, parameter_rating_max, weight
        FROM rating_system_parameter WHERE rating_system_id = grp.rating_system_id;

        UPDATE pulsarr_group SET rating_system_id = new_rs_id
        WHERE pulsarr_group_id = grp.pulsarr_group_id;

        UPDATE rating SET rating_system_id = new_rs_id
        WHERE pulsarr_group_id = grp.pulsarr_group_id
          AND rating_system_id = grp.rating_system_id;

        UPDATE rating_detail rd
        SET rating_system_parameter_id = new_param.new_id
        FROM (
            SELECT op.rating_system_parameter_id AS old_id,
                   np.rating_system_parameter_id AS new_id
            FROM rating_system_parameter op
            JOIN rating_system_parameter np ON op.name = np.name
                AND np.rating_system_id = new_rs_id
            WHERE op.rating_system_id = grp.rating_system_id
        ) new_param
        WHERE rd.rating_system_parameter_id = new_param.old_id
          AND rd.rating_id IN (
              SELECT rating_id FROM rating
              WHERE pulsarr_group_id = grp.pulsarr_group_id
          );
    END LOOP;
END $$;
