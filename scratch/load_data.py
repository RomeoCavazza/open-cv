import json
import subprocess
import os

instance_id = '8b28bb99-f882-4514-93d5-0cdb02c4ef65'
base_path = 'data/instances/credit_agricole_stage_alternance_ia_2026_110627'

with open(os.path.join(base_path, 'resume.json'), 'r') as f:
    resume = f.read()
with open(os.path.join(base_path, 'cover-letter.json'), 'r') as f:
    cover = f.read()

with open("scratch/restore.sql", "w") as f:
    f.write("UPDATE instances SET resume_json = :'res', cover_letter_json = :'cov', status = 'ready' WHERE id = :'id';\n")

subprocess.run([
    "psql", "-h", "localhost", "-U", "alternance", "-d", "alternance",
    "-v", f"res={resume}",
    "-v", f"cov={cover}",
    "-v", f"id={instance_id}",
    "-f", "scratch/restore.sql"
])
os.remove("scratch/restore.sql")
print("Done")
