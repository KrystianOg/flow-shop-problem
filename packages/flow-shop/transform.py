import os
import zipfile
import shutil
from scipy.io import loadmat
import numpy as np
import pandas as pd

# Step 1: Extract zip
zip_path = 'data/Problems.zip'
extracted_dir = 'extracted_mat_files'
os.makedirs(extracted_dir, exist_ok=True)

with zipfile.ZipFile(zip_path, 'r') as zip_ref:
    zip_ref.extractall(extracted_dir)

# Step 2: Convert .mat files
output_dir = 'converted_files'
os.makedirs(output_dir, exist_ok=True)

# Only extract meaningful keys
keys_to_extract = ['TPO', ]

for root, _, files in os.walk(extracted_dir):
    for file in files:
        if file.endswith('.mat'):
            mat_path = os.path.join(root, file)
            data = loadmat(mat_path)
            base_name = os.path.splitext(file)[0]

            for key in keys_to_extract:
                if key in data:
                    arr = data[key]

                    # Remove leading singleton dims
                    arr = np.squeeze(arr)

                    if arr.ndim > 2:
                        print(f"Skipping {key} in {file} (dim={arr.ndim})")
                        continue  # Optional: only save 2D matrices

                    df = pd.DataFrame(arr)
                    csv_name = f"{base_name}_{key}.csv"
                    df.to_csv(os.path.join(output_dir, csv_name), index=False, header=False)


shutil.rmtree(extracted_dir)