import json
import subprocess

# Read the data
with open('../js/data.json', 'r') as f:
    data = json.load(f)

# Pass data to Rust (serialise as JSON string)
rust_process = subprocess.Popen(
    ['cargo', 'run'], stdin=subprocess.PIPE, stdout=subprocess.PIPE, cwd='../rust_pipeline'
)
rust_output, _ = rust_process.communicate(input=json.dumps(data).encode())

# Print Rust's raw output for debugging
print('Raw output from Rust:', rust_output.decode())

# Deserialise the Rust output
try:
    result = json.loads(rust_output.decode())
    print('Results from Rust:', result)
except json.JSONDecodeError as e:
    print('Error decoding JSON:', e)
    print('Received output:', rust_output.decode())
