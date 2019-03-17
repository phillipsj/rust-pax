# -*- coding=utf-8 -*-
import click


@click.group()
def pax():
    pass


@pax.group()
def show():
    pass


@show.command()
def c():
    click.echo('Show C')


@show.command()
def w():
    click.echo('Show W')


if __name__ == '__main__':
    pax()
