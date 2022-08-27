# %%
import json
import datetime
import sys

assert("--version" in sys.argv)

version_number = sys.argv[2]
version_number_stripped_v = version_number.replace("v", "")

PATH_TO_LINUX_SIG = f"./artifact/appimage/flightcore_{version_number_stripped_v}_amd64.AppImage.tar.gz.sig"
PATH_TO_WINDOWS_SIG = f"./artifact/msi/flightcore_{version_number_stripped_v}_x64_en-US.msi.zip.sig"

# Read signatures
with open(PATH_TO_LINUX_SIG, "rt") as f:
  linux_sig = f.read()
with open(PATH_TO_WINDOWS_SIG, "rt") as f:
  windows_sig = f.read()


current_datetime_string = str(datetime.datetime.utcnow().replace(microsecond=0).isoformat() + "Z")

release_dict = {
  "version": f"{version_number}",
  "notes": "Test version",
  "pub_date": current_datetime_string,
  "platforms": {
    "linux-x86_64": {
      "signature": linux_sig,
      "url": f"https://github.com/GeckoEidechse/FlightCore/releases/download/{version_number}/flightcore_{version_number_stripped_v}_amd64.AppImage.tar.gz"
    },
    "windows-x86_64": {
      "signature": windows_sig,
      "url": f"https://github.com/GeckoEidechse/FlightCore/releases/download/{version_number}/flightcore_{version_number_stripped_v}_x64_en-US.msi.zip"
    }
  }
}
json_string = json.dumps(release_dict, indent=4)
print(json_string)
# %%
RESULT_JSON_FILENAME = "latest-release.json"
with open(RESULT_JSON_FILENAME, "wt") as f:
    f.write(json_string)
# %%
