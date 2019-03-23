# -*- coding=utf-8 -*-
from html.parser import HTMLParser
from zipfile import ZipFile
from enum import Enum, unique
from io import BytesIO
from pathlib import Path
import requests
import platform
import os
import stat


@unique
class HashiProduct(Enum):
    """Enum of valid project types."""

    PACKER = 'packer'
    TERRAFORM = 'terraform'


class VersionParser(HTMLParser):
    """Parses versions using HTMLParser."""

    def __init__(self, product):
        super(VersionParser, self).__init__()
        self.versions = []
        self.product = product.value

    def handle_data(self, data):
        if data.startswith(self.product):
            self.versions.append(data.split('_')[1])


class HashiVersion():
    def __init__(self,
                 product,
                 version,
                 platform='linux',
                 architecture='amd64'):
        self.version = version
        self.platform = platform
        self.architecture = architecture
        self.product = product.value
        # https://releases.hashicorp.com/packer/1.3.5/packer_1.3.5_darwin_386.zip
        self.base_url = 'https://releases.hashicorp.com/{0}'
        self.base_version_url = 'https://releases.hashicorp.com/{0}/{1}/{2}_{3}_{4}_{5}.zip'

    def get_product_url(self):
        return self.base_version_url.format(self.product,
                                            self.version,
                                            self.product,
                                            self.version,
                                            self.get_platform(),
                                            self.get_architecture())

    def get_versions_url(self):
        return self.base_url.format(self.product)

    def get_architecture(self):
        machine = platform.machine()
        if machine == 'x86_64':
            return 'amd64'
        elif machine == 'i386':
            return '386'
        elif machine == 'arm':
            return 'arm'

    def get_platform(self):
        return platform.system().lower()


class Hashi():
    def __init__(self, product, install_path='.local/bin'):
        self.product = product
        self.install_path = install_path

    def install(self, version):
        installation_path = os.path.join(Path.home(), self.install_path)
        packerVersion = HashiVersion(HashiProduct.PACKER, version)

        print(packerVersion.get_versions_url())
        version_request = requests.get(packerVersion.get_versions_url())

        parser = VersionParser(HashiProduct.PACKER)
        parser.feed(version_request.text)

        print(packerVersion.get_product_url())
        package = requests.get(packerVersion.get_product_url())
        with ZipFile(BytesIO(package.content)) as zf:
            zf.extractall(installation_path)

        exe_path = os.path.join(installation_path, HashiProduct.PACKER.value)
        os.chmod(exe_path, stat.S_IXUSR)


# This is the usage
#hashi = Hashi(HashiProduct.PACKER, 'test/bin')
#hashi.install('1.3.2')
