#!/usr/bin/env python
from jinja2 import Template
import click

@click.command()
@click.option('-o', '--out', default=None)
@click.argument("src")
def main(out, src):
    tpl = Template(open(src).read())
    s = tpl.render()
    if not out:
        print (s)
    else:
        print (s, file=open(out, 'w'))

if __name__ == "__main__":
    main()
