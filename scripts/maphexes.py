"""Formats map hexes from warapi into a Rust enumeration"""

import requests

HEX_URL = "https://war-service-live.foxholeservices.com/api/worldconquest/maps"
LOCATION_PREFIX_URL = (
    "https://war-service-live.foxholeservices.com/api/worldconquest/maps/"
)
LOCATION_SUFFIX_URL = "/static"
DIV = "--------------------------"

print("Starting map formatting..")
buffer = ""  # primary buffer, instead of printing

hexes = requests.get(HEX_URL).json()
hex_enum = (
    "/// Hex name of a single map tile, topmost of all map details\npub enum MapHex {\n"
)

for pos in hexes:
    name = pos[:-3]
    hex_enum += f"    {name}({name}),\n"

    print(f"Getting details for {name} hex..")

    detailed = requests.get(f"{LOCATION_PREFIX_URL}{pos}{LOCATION_SUFFIX_URL}").json()[
        "mapTextItems"
    ]

    buffer += (
        f"/// Specific map details for the `{pos}` tile\npub enum {name} {{"
    ) + "\n"
    hex_impl_major = f"impl MapLocation for {name} {{\n    fn is_major(&self) -> bool {{\n        match self {{\n"
    hex_impl_location = (
        f"    fn location(&self) -> (f64, f64) {{\n        match self {{\n"
    )

    for detail in detailed:
        detailrawname = detail["text"]
        detailname = detailrawname.replace(" ", "").replace("'", "")
        morm = detail["mapMarkerType"].lower()
        is_morm = str(morm == "major").lower()

        print(f"  Getting details for {detailname} position..")

        _location_x = str(detail["x"])
        _location_y = str(detail["y"])
        location = f"({_location_x}, {_location_y})"

        buffer += (
            f"    /// {detailrawname} is a {morm} location\n    {detailname},"
        ) + "\n"
        hex_impl_major += f"            {name}::{detailname} => {is_morm},\n"
        hex_impl_location += f"            {name}::{detailname} => {location},\n"

    buffer += ("}\n") + "\n"
    buffer += (hex_impl_major + "        }\n    }\n") + "\n"
    buffer += (hex_impl_location + "        }\n    }\n}\n") + "\n"

hex_enum += "}"

buffer += (
    "{DIV}\n{DIV}\n{DIV}\nADD ALL BELOW TO THE TOP OF FILE\n{DIV}\n{DIV}\n{DIV}\n"
) + "\n"
buffer += (hex_enum) + "\n"

print("Saving buffer..")

with open("mapdump.txt", "w+") as file:
    file.write(buffer)
