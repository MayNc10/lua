#!/bin/python3

import sys
import subprocess
import os
import pathlib

script_directory = pathlib.Path(__file__).parent.resolve()
os.chdir(f"{script_directory}/Lua")

benchs = ["DeltaBlue", "Richards", "Json", "CD", "Havlak", "List", "NBody"]
args = [["1"], ["1"], ["1"], ["10"], ["1"], ["1"], ["1"]]

if (sys.argv[1] != "all"):
    idx = benchs.index(sys.argv[1])
    benchs = [benchs[idx]]
    args = [args[idx]]

for (bench, args) in zip(benchs, args):
    subcmds = [f"lua harness.lua {bench} 1 {' '.join(args)}", 
               f"../../target/release/lua harness.lua {bench} 1 {' '.join(args)}"] 
    cmd = ["hyperfine"] + subcmds + sys.argv[2:]
    print(cmd)

    subprocess.run(cmd, text=True)