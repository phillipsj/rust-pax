# -*- coding=utf-8 -*-
import sys
from collections import OrderedDict

from knack import CLI, ArgumentsContext, CLICommandsLoader
from knack.commands import CommandGroup


def show_c():
    return "Show C"


def show_w():
    return "Show W"


class PaxCommandsLoader(CLICommandsLoader):
    def load_command_table(self, args):
        with CommandGroup(self, 'show', '__main__#{}') as group:
            group.command('c', 'show_c')
            group.command('w', 'show_w')
        return OrderedDict(self.command_table)


pax = CLI(cli_name='pax', commands_loader_cls=PaxCommandsLoader)
exit_code = pax.invoke(sys.argv[1:])
sys.exit(exit_code)
