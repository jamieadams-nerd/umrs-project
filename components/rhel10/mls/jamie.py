#!/usr/bin/python3

import json

new_data = {}

with open("notes/umrs-mls-mcs.json", "r") as file:
    extra_info = json.load(file)

for k in extra_info["categories"].keys():
    label = extra_info["categories"][k]['name']
    desc = extra_info["categories"][k]['description']
    new_data[label] = desc

with open('cui-mls-labels.json', 'r') as file:
    data_dict = json.load(file)

# Mapping of new_data
for k,v in new_data.items():
    print(k,v)

# PRIVACY
# EXPORT
# FINPROT
# PROCURE
### LEGAL
# TECHDATA


data_dict["markings"]["CUI//GOVT/LEGL"]['description'] = new_data['LEGAL']


with open("output.json", "w") as json_file:
    json.dump(data_dict, json_file, indent=4)

with open("descriptions.json", "w") as json_file:
    json.dump(new_data, json_file, indent=4)

