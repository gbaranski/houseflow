Import("env")
import sys

env.Execute("$PYTHONEXE -m pip install python-dotenv")

from dotenv import dotenv_values

config = {
    **dotenv_values(".env"),
    **dotenv_values(".env." + env["PIOENV"]),
}

env.Append(CPPDEFINES=config.items())
# env.Append(OTA_PASSWORD=config["DEVICE_SECRET"])
