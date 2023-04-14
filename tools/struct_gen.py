import json
import requests
import bs4
import re


class Struct:
    members: dict
    name: str
    enum: bool

    def __init__(self, name: str, enum: bool = False) -> None:
        self.members = {}
        self.name = name
        self.enum = enum

    def to_string(self, enum_overrides: dict[str, str]) -> str:
        if not self.enum:
            out = f"#[derive(Serialize, Deserialize, Debug, Clone)]\npub struct {self.name} {chr(123)}\n"
            for i in self.members.items():
                if i[1][1] != "none" and i[1][1] != "None":
                    out += f"    // {i[1][1]}\n"
                if i[0].lower() == i[0] and i[0] != "type":
                    out += f"    pub {i[0]}: {i[1][0]},\n"
                else:
                    out += f"    #[serde(rename = \"{i[0]}\")]\n    pub {struct_change_case(i[0])}: {i[1][0]},\n"

            out += "}"
            return out
        else:
            out = f"cf_enum! {chr(123)}{self.name}, [derive(Serialize_repr, PartialEq, Debug, Clone)],\n"
            for n, i in enumerate(self.members.items()):
                out += f"    {enum_get_var_prefix(i,n,self)}{enum_change_case(i[1])} = {i[0]}{(',' if n < len(self.members) - 1 else '')}\n"
            out += "}"
            return out


def find_schema(tag: bs4.Tag) -> bool:
    if len(tag.get_attribute_list("id")) > 0 and tag.get_attribute_list("id")[0] != None:
        return re.match(r"tocS\_.+", tag.get_attribute_list("id")[0]) != None and tag.name == "h2"
    else:
        return False


def enum_get_var_prefix(i, n, self):
    if ((n == 0 and not self.name in enum_overrides.keys()) or
            (self.name in enum_overrides.keys() and enum_overrides[self.name]) == enum_change_case(i[1])):
        return "@"
    else:
        return ""


def struct_change_case(str):
    if str == "type":
        return "type_type"
    res = [str[0].lower()]
    for c in str[1:]:
        if c in ('ABCDEFGHIJKLMNOPQRSTUVWXYZ'):
            res.append('_')
            res.append(c.lower())
        else:
            res.append(c)

    return ''.join(res)


def enum_change_case(str):
    res = [str[0]]
    next_upper = False
    for c in str[1:]:
        if c == "_":
            next_upper = True
        elif next_upper:
            res.append(c.upper())
        else:
            res.append(c)

    return ''.join(res)


def find_tag(tag):
    type(tag) == bs4.Tag


def parse_type(info):
    if type(info) == bs4.Tag:
        sus = "".join(info.strings)
    else:
        sus = str(info)
    if sus[-5:] == "¦null":
        lets = parse_type(sus[:-5])
        return (f"Option<{lets[0]}>", lets[1])
    elif sus[0] == "[" and sus[-1] == "]":
        lets = parse_type(sus[1:-1])
        return (f"Vec<{lets[0]}>", lets[1])
    elif sus[:7] == "integer":
        return ("i64", [])
    elif sus == "boolean":
        return ("bool", [])
    elif sus == "string":
        return ("CFString", [])
    elif sus == "string(date-time)":
        return ("CFString", [])
    elif sus == "object":
        return ("serde_json::Value", [])
    else:
        return (sus, [sus])


if __name__ == "__main__":
    print("Converts the CF Webpage for types into rust structs")

    with open("enum_default_overrides.json") as f:
        enum_overrides = json.load(f)

    r = requests.get("https://docs.curseforge.com/?python")
    soup = bs4.BeautifulSoup(r.content, features="lxml")
    type_list_tag: bs4.Tag = soup.find(
        "a", string="Schemas").find_next_sibling("ul")
    type_list: list = {}

    for e in type_list_tag.children:
        if type(e) == bs4.Tag:
            if not " " in e.find("a").string:
                type_list[e.find("a").string] = e.a["href"]

    sus = []
    for i, t in enumerate(type_list.keys()):
        print(f"{i+1}: {t}")
        sus.append(t)

    mogus = input(
        "What structs do you want transcribing? (comma seperated or * for all): ")
    if mogus != "*":
        selection = [sus[int(x) - 1]
                     for x in mogus.replace(" ", "").split(",")]
    else:
        selection = sus

    structs = []
    processed = []
    while len(selection) > 0:
        current = selection.pop()
        data = soup.find(id=type_list[current][1:]).findNextSibling(
            class_="highlight").findNextSibling(lambda tag: type(tag) == bs4.Tag)
        if data.contents == ["Properties"]:
            processed.append(current)
            struct = Struct(current.replace(" ", ""))
            table = data.findNextSibling(lambda tag: type(tag) == bs4.Tag)
            for i in table.tbody.children:
                if type(i) == bs4.Tag:
                    data = [x for x in i.contents if type(x) == bs4.Tag]
                    megus = parse_type(data[1])
                    struct.members[str(data[0].string)] = (
                        megus[0], data[2].string)
                    if len(megus[1]) > 0:
                        selection.extend(
                            [i for i in megus[1] if i not in processed])
                        selection = [*set(selection)]
            structs.append(struct)
        elif data.contents == ["Possible enum values: "]:
            processed.append(current)
            struct = Struct(current.replace(" ", ""), enum=True)
            quit_loop = False
            while not quit_loop:
                data = data.findNextSibling(lambda tag: type(tag) == bs4.Tag)
                if data.name != "p":
                    quit_loop = True
                else:
                    struct.members[data.string.split(
                        "=")[0]] = data.string.split("=")[1]
            structs.append(struct)

    with open("../src/cf_api/generated.rs", "w") as f:
        f.write(
            """use crate::cf_enum;
use serde::{Deserialize, Serialize};
use serde_repr::Serialize_repr;

use super::CFString;
/*
* DO NOT EDIT THIS FILE MANUALLY
* IT WAS GENERATED USING tools/struct_gen.py
* To change enums default values, add the enum name and the default value to tools/enum_default_overrides.json
*/

""")
        for struct in structs:
            f.write(struct.to_string(enum_overrides))
            f.write("\n\n")
