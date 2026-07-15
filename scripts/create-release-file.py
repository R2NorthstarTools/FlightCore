# %%
import json
import datetime
import sys

assert("--version" in sys.argv)

version_number = sys.argv[2]
version_number_stripped_v = version_number.replace("v", "")

PATH_TO_LINUX_SIG = f"./linux-artifacts/FlightCore_{version_number_stripped_v}_amd64.AppImage.tar.gz.sig"
PATH_TO_WINDOWS_SIG = f"./windows-artifacts/bundle/msi/FlightCore_{version_number_stripped_v}_x64_en-US.msi.zip.sig"

# Text to show in update notification
RELEASE_TEXT = "See the following link for release notes: https://github.com/R2NorthstarTools/FlightCore/releases"

# Read signatures
with open(PATH_TO_LINUX_SIG, "rt") as f:
  linux_sig = f.read()
with open(PATH_TO_WINDOWS_SIG, "rt") as f:
  windows_sig = f.read()


current_datetime_string = str(datetime.datetime.utcnow().replace(microsecond=0).isoformat() + "Z")

release_dict = {
  "version": f"{version_number}",
  "notes": f"{RELEASE_TEXT}",
  "pub_date": current_datetime_string,
  "platforms": {
    "linux-x86_64": {
      "signature": linux_sig,
      "url": f"https://github.com/R2NorthstarTools/FlightCore/releases/download/{version_number}/flight-core_{version_number_stripped_v}_amd64.AppImage.tar.gz"
    },
    "windows-x86_64": {
      "signature": windows_sig,
      "url": f"https://github.com/R2NorthstarTools/FlightCore/releases/download/{version_number}/FlightCore_{version_number_stripped_v}_x64_en-US.msi.zip"
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
