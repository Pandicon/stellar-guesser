import pandas as pd

# Load the Hipparcos data CSV file
hipparcos_data = pd.read_csv("hipparcos_catalogue.csv")

# Print every HIP ID
print("HIP IDs:")
# for hip_id in hipparcos_data["HIP"]:
    # print(hip_id)

# Select and rename the desired columns
filtered_data = hipparcos_data.rename(columns={
    "HIP": "hip_id",
    "_RA.icrs": "ra",
    "_DE.icrs": "dec",
    "Vmag": "vmag",
    "B-V": "bv"
})[["hip_id", "ra", "dec", "vmag", "bv"]]

# Save the new data to a CSV file
filtered_data.to_csv("hipparcos_filtered.csv", index=False)

print(f"Filtered data saved to 'hipparcos_filtered.csv'.")
