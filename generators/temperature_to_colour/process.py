with open('raw.txt', 'r') as file:
    lines = file.readlines()

processed_data = []

for i in range(0, len(lines), 2):
    line = lines[i]
    parts_all = line.strip().split(" ")
    parts = []
    for part in parts_all:
        if part.strip() != "":
            parts.append(part.strip())
    temperature = parts[0]
    color_values = parts[-4:-1]
    result = f"({temperature}, Color32::from_rgb({color_values[0]}, {color_values[1]}, {color_values[2]}))"
    processed_data.append(result)

with open('processed.txt', 'w') as output_file:
    for data in processed_data:
        output_file.write(data + '\n')

for data in processed_data:
    print(data)
