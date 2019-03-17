# -*- coding=utf-8 -*-
from html.parser import HTMLParser
from zipfile import ZipFile
from io import BytesIO
import requests
import platform
import os
import stat


class PackerVersionParser(HTMLParser):
    def __init__(self):
        super(PackerVersionParser, self).__init__()
        self.versions = []

    def handle_data(self, data):
        if data.startswith('packer'):
            self.versions.append(data.split('_')[1])


class PlatformParser():
    def get_platform(self):
        machine = platform.machine()
        if machine == 'x86_64':
            return 'amd64'
        elif machine == 'i386':
            return '386'
        elif machine == 'arm':
            return 'arm'

    def get_system(self):
        return platform.system().lower()


class PackerVersion():
    def __init__(self, version='latest',
                 platform='linux',
                 architecture='amd64'):
        self.version = version
        self.platform = platform
        self.architecture = architecture
        self.base_url = "https://releases.hashicorp.com/packer/{0}/packer_{1}_{2}_{3}.zip"

    def get_url(self):
        return self.base_url.format(self.version, self.version,
                                    self.platform, self.architecture)


url = "https://releases.hashicorp.com/packer/"
version_request = requests.get(url)

parser = PackerVersionParser()
parser.feed(version_request.text)

platformParser = PlatformParser()

packerVersion = PackerVersion('1.3.2',
                              platformParser.get_system(),
                              platformParser.get_platform())

file_name = 'packer_1.3.2_linux_amd64.zip'

print(packerVersion.get_url())

# Downloads and extracts it
package = requests.get(packerVersion.get_url())
with ZipFile(BytesIO(package.content)) as zf:
    # need to put path ~.local/bin
    zf.extractall()

# Makes it executable
os.chmod('packer', stat.S_IXUSR)

#os.remove(file_name)
