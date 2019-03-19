# -*- coding: utf-8 -*-

"""Main `pax` CLI."""

import os
import sys
import json
import collections

import click

from pax import __version__


def version_msg():
    """Return the Pac version, location and Python powering it."""
    python_version = sys.version[:3]
    location = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    message = u'Pax %(version)s from {} (Python {})'
    return message.format(location, python_version)


@click.group()
@click.version_option(__version__, u'-V', u'--version', message=version_msg())
def pax():
    pass


@pax.command()
@click.argument(u'license_info')
def show(license_info):
    '''Shows the license file section.'''

    if license_info == 'c':
        click.echo('show C')
    elif license_info == 'w':
        click.echo('show w')
    else:
        click.echo('Not supported')


@pax.command()
@click.argument('tool')
@click.option('--version',
              default='latest',
              help='Version to install.')
def install(tool, version='latest'):
    '''Install tool for platform and architecture.'''

    click.echo('Called install Terraform')


if __name__ == '__main__':
    pax()
