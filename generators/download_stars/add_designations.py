from astroquery.simbad import Simbad
import pandas as pd
import time

CHUNK_SIZE = 1000
constellations = "And , Ant, Aps, Aqr, Aql, Ara, Ari, Aur, Boo, Cae, Cam, Cnc, CVn, CMa, CMi, Cap, Car, Cas, Cen, Cep, Cet, Cha, Cir, Col, Com, CrA, CrB, Crv, Crt, Cru, Cyg, Del, Dor, Dra, Equ, Eri, For, Gem, Gru, Her, Hor, Hya, Hyi, Ind, Lac, Leo, LMi, Lep, Lib, Lup, Lyn, Lyr, Men, Mic, Mon, Mus, Nor, Oct, Oph, Ori, Pav, Peg, Per, Phe, Pic, Psc, PsA, Pup, Pyx, Ret, Sge, Sgr, Sco, Scl, Sct, Ser, Sex, Tau, Tel, Tri, TrA, Tuc, UMa, UMi, Vel, Vir, Vol, Vul"

hip_data = pd.read_csv("hipparcos_filtered.csv")

hip_ids_int = hip_data["hip_id"].astype(int).tolist()
hip_ids_int.sort()
hip_ids = []
for hip_id in hip_ids_int:
    hip_ids.append(hip_id)
print(hip_ids[0], hip_ids[-1], hip_ids[len(hip_ids)//2])

custom_simbad = Simbad()
custom_simbad.TIMEOUT = 120  # Increase timeout for large queries
custom_simbad.add_votable_fields('ids', 'main_id')  # Request the main ID and alternate designations
# custom_simbad.ADDITIONAL_VOTABLE_FIELDS = ['ids', 'main_id'] 

def query_batch(hip_ids_batch):
    try:
        result = custom_simbad.query_objects(hip_ids_batch)
        if result is None:
            print("No result returned for the batch.")
            return []
        star_info = []
        if not "MAIN_ID" in result.columns or not "SCRIPT_NUMBER_ID" in result.columns or not "IDS" in result.columns:
            raise Exception("Missing some fields...")
        for i in range(len(result["SCRIPT_NUMBER_ID"])):
            h = hip_ids_batch[int(result["SCRIPT_NUMBER_ID"][i])-1][4:]
            data = {
                "hip": hip_ids_batch[int(result["SCRIPT_NUMBER_ID"][i])-1][4:],
                "name": "",
                "bayer": "",
                "flamsteed": "",
            }
            names_spl = result["IDS"][i].split("|")
            for name in names_spl:
                name = name.strip()
                if name.startswith("NAME "):
                    data["name"] = name[5:]
                if name.startswith("*") and name[-3:].lower() in constellations.lower():
                    name_unstarred = name[1:].strip()
                    cs = []
                    for char in name_unstarred:
                        if char.isalnum() or char == " ":
                            cs.append(char)
                    name_unstarred = ''.join(cs)
                    if name_unstarred[0].isnumeric():
                        data["flamsteed"] = name_unstarred
                    else:
                        data["bayer"] = name_unstarred
                    print(f"hip: {h}, name raw: {name}, name processed: {name_unstarred}")
            # print(data)
            star_info.append(data)
        return star_info
    except Exception as e:
        print(f"Error in batch query: {e}")
        return []

all_star_info = []

i = 0
end_lim = len(hip_ids)
while i < end_lim:
    # Get the current batch of HIP IDs
    end = min(end_lim, i + CHUNK_SIZE)
    hip_ids_batch = []
    for j in hip_ids[i:end]:
        hip_ids_batch.append(f"HIP {j}")
    if len(hip_ids_batch) == 0:
        i += CHUNK_SIZE
        continue
    print(hip_ids_batch[0], hip_ids_batch[-1])
    
    # Query the batch
    batch_info = query_batch(hip_ids_batch)
    # print(batch_info)
    all_star_info.extend(batch_info)
    
    # Sleep for a short time to avoid overloading the server
    time.sleep(2)

    print(f"Processed batch {i // CHUNK_SIZE + 1} of {len(hip_ids) // CHUNK_SIZE + 1}")
    i += CHUNK_SIZE

new_data = {
    "hip": [],
    "name": [],
    "bayer": [],
    "flamsteed": []
}
named_count = 0
bayer_count = 0
flamsteed_count = 0
for s_i in all_star_info:
    new_data["hip"].append(s_i["hip"])
    new_data["name"].append(s_i["name"])
    new_data["bayer"].append(s_i["bayer"])
    new_data["flamsteed"].append(s_i["flamsteed"])
    if s_i["name"].strip() != "":
        named_count += 1
    if s_i["bayer"] != "":
        bayer_count += 1
    if s_i["flamsteed"] != "":
        flamsteed_count += 1

print(new_data)

print(f"Statistics:\nNamed stars: {named_count}\nBayer designated stars: {bayer_count}\nFlamsteed designated stars: {flamsteed_count}")

new_data_df = pd.DataFrame(new_data)

hip_data["hip_id"] = hip_data["hip_id"].astype(str)
new_data_df["hip"] = new_data_df["hip"].astype(str)

merged_data = pd.merge(hip_data, new_data_df, left_on="hip_id", right_on="hip", how="left")
merged_data.drop(columns=["hip"], inplace=True)

# Save the extended dataset to a new CSV file
merged_data.to_csv("hipparcos_data_with_names.csv", index=False)
print("Star names and Bayer designations saved to 'hipparcos_data_with_names.csv'.")
