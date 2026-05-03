#!/bin/bash
INSTANCE_ID="8b28bb99-f882-4514-93d5-0cdb02c4ef65"
RESUME_JSON=$(cat data/instances/credit_agricole_stage_alternance_ia_2026_110627/resume.json)
COVER_JSON=$(cat data/instances/credit_agricole_stage_alternance_ia_2026_110627/cover-letter.json)

psql -h localhost -U alternance -d alternance -c "UPDATE instances SET resume_json = '$RESUME_JSON', cover_letter_json = '$COVER_JSON', status = 'ready' WHERE id = '$INSTANCE_ID';"
