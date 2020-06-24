import shutil
import sys
import os

dlls = [
    "System.Runtime"
]

profile = sys.argv[1]
out_dir = os.path.abspath("target/{}/libraries".format(profile))
if profile == "debug":
    profile = "Debug"
elif profile == "release":
    profile = "Release"
else:
    sys.exit(-1)

try:
    os.mkdir(out_dir)
except:
    pass

for dll in dlls:
    shutil.copy(
        "libraries/{0}/bin/{1}/netcoreapp3.1/{0}.dll".format(dll, profile), "{}/{}.dll".format(out_dir, dll))
