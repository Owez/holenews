"""Formats map hexes from warapi into a Rust enumeration"""

import re
import requests

MAP_LOCATION_TRAIT = """/// Common trait for map locations of a given hex
pub trait MapLocation {
    /// Checks if this map location if major or not
    pub fn is_major(&self) -> bool;

    /// The `x` and `y` coordinates of this location
    pub fn location(&self) -> (f64, f64);
}
"""

print("Starting map formatting..")

buffer = "" # primary buffer, instead of printing

def add_buffer(input):
    buffer += f"{input}\n"

hexes = requests.get("https://war-service-live.foxholeservices.com/api/worldconquest/maps").json()
hex_enum = "/// Hex name of a single map tile, topmost of all map details\npub enum MapHex {\n"

for pos in hexes:
    name = pos[:-3]
    hex_enum += f"    {name}({name}),\n"

    print(f"Getting details for {name} hex..")

    detailed = requests.get(f"https://war-service-live.foxholeservices.com/api/worldconquest/maps/{pos}/static").json()["mapTextItems"]

    add_buffer(f"/// Specific map details for the `{pos}` tile\npub enum {name} {{")
    hex_impl_major = f"impl MapLocation for {name} {{\n    fn is_major(&self) -> bool {{\n        match self {{\n"
    hex_impl_location = f"    fn location(&self) -> (f64, f64) {{\n        match self {{\n"

    for detail in detailed:
        detailrawname = detail["text"]
        detailname = detailrawname.replace(" ", "").replace("'", "")
        morm = detail["mapMarkerType"].lower()
        is_morm = str(morm == "major").lower()

        print(f"  Getting details for {detailname} position..")

        _location_x = str(detail["x"])
        _location_y = str(detail["y"])
        location = f"({_location_x}, {_location_y})"

        add_buffer(f"    /// {detailrawname} is a {morm} location\n    {detailname},")
        hex_impl_major += f"            {name}::{detailname} => {is_morm},\n"
        hex_impl_location += f"            {name}::{detailname} => {location},\n"


    add_buffer("}\n")

    add_buffer(hex_impl_major + "    }\n")
    add_buffer(hex_impl_location + "    }\n}\n")

hex_enum += "}"

add_buffer("--------------------------\n--------------------------\n--------------------------\nADD ALL BELOW TO THE TOP OF FILE\n--------------------------\n--------------------------\n--------------------------\n")

add_buffer(MAP_LOCATION_TRAIT)
add_buffer(hex_enum)

print("Saving buffer..")

with open("mapdump.txt", "w+") as file:
    file.write(buffer)
