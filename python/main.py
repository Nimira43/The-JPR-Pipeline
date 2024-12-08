import json
import subprocess

with open('../js/data.json', 'r') as f:
  data = json.load(f)

# Pass data to Rust (serialise as JSON string)
rust_process = subprocess.Popen(
  ['cargo', 'run'], stdin = subprocess.PIPE, stdout = subprocess.PIPE
)
rust_output, _ = rust_process.communicate(input = json.dumps(data).encode())