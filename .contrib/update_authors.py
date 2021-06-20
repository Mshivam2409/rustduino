import toml
import json
import os

cargo = toml.load('Cargo.toml')

authors = sorted(os.listdir('./.contrib'))

authors_list = []

for author in authors:
    if author.endswith("json"):
        js = json.load(open("./.contrib/"+author))
        authors_list.append("{} <{}>".format(js["name"],js["email"]))

cargo["package"]["authors"] = authors_list

with open('Cargo.toml','w') as f:
    toml.dump(cargo,f)


