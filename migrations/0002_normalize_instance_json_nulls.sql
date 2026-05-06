UPDATE instances
SET restitution = NULL
WHERE restitution = 'null'::jsonb;

UPDATE instances
SET resume_json = NULL
WHERE resume_json = 'null'::jsonb;

UPDATE instances
SET cover_letter_json = NULL
WHERE cover_letter_json = 'null'::jsonb;
