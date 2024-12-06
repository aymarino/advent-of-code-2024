#!/usr/bin/env python3
import argparse
import subprocess
import sys

with open(".session") as f:
    SESSION = f.read()  # Session cookie from adventofcode.com

parser = argparse.ArgumentParser(description="Read input for 2024")
parser.add_argument("--day", type=int, required=True)
args = parser.parse_args()

cmd = f'curl https://adventofcode.com/2024/day/{args.day}/input --cookie "session={SESSION}"'
output = subprocess.check_output(cmd, shell=True)
output = output.decode("utf-8")
with open(f"inputs/{args.day}.txt", "w") as f:
    f.write(output)

print("\n".join(output.split("\n")[:10]), file=sys.stderr)
