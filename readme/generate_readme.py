#!/usr/bin/env python3

from jinja2 import Template
import subprocess

COMMAND_SHOW = """
### {}
```console
user@pc:~$ gmadrs {} 
```
```
{}
```
"""

HEADINGS = []

def show_command(args_in):
    full_args = ["../target/debug/gmadrs"] + args_in
    subcommand = args_in[0]

    heading = "[{}](#cmd-{})".format(subcommand.capitalize(), subcommand)
    HEADINGS.append(heading)
    args_string = " ".join(args_in)

    output = subprocess.run(full_args, capture_output=True).stdout.decode("utf-8")
    return COMMAND_SHOW.format(heading, args_string, output)

readme_template = open("README_template.md", "r").read()

readme_t = Template(readme_template)
readme = readme_t.render(show_command=show_command)

open("../README.md", "w").write(readme)

for h in HEADINGS:
    print("* {}".format(h))