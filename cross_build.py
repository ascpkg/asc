import http.client
import inspect
import json
import re
import os
import platform
import shutil
import subprocess


def color_print(text):
    print(f'\033[93m{text}\033[0m')


def shell(args):
    color_print('    ' + ' '.join(args))

    subprocess.run(args)


def is_located_china():
    conn = http.client.HTTPSConnection("ipinfo.io")
    conn.request("GET", "/json")

    response = conn.getresponse()
    text = response.read().decode()
    data = json.loads(text)

    return data.get('country') == 'CN'


def get_default_proxy() -> tuple:
    if not is_located_china():
        return ('', '', '')
    else:
        if not os.environ.get('WSL_DISTRO_NAME'):
            return ('http', 10809, '127.0.0.1')
        else:
            for line in subprocess.run(['ip', 'route'], stdout=subprocess.PIPE, universal_newlines=True).stdout.strip().splitlines():
                if line.startswith('default'):
                    host_ip = re.search(r'\s(\d+\.\d+\.\d+\.\d)\s', line).group(1)
                    return ('http', 10809, host_ip)
            return ('', '', '')

def set_env():
    color_print(inspect.currentframe().f_code.co_name)

    if platform.system() == 'Windows':
        path_delimiter = ';'
        zig_path = r'C:\zig-windows-x86_64-0.13.0'  # https://ziglang.org/download/0.13.0/zig-windows-x86_64-0.13.0.zip
        zig_lib_dir = rf'{zig_path}\lib'
        mac_os_sdk_path = r'C:\MacOSX11.3.sdk'  # https://github.com/ascpkg/asc/releases/download/MacOSX11.3.sdk/MacOSX11.3.sdk.tar.xz
    else:
        path_delimiter = ':'
        zig_path = '/mnt/c/zig-linux-x86_64-0.13.0'  # https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz
        zig_lib_dir = f'{zig_path}/lib'
        mac_os_sdk_path = '/mnt/c/MacOSX11.3.sdk'

    # set macOS sdk env
    os.environ['SDKROOT'] = os.environ.get('SDKROOT', mac_os_sdk_path)

    # set zig env
    os.environ['ZIG'] = zig_path
    os.environ['ZIG_LIB_DIR'] = zig_lib_dir
    os.environ['PATH'] = os.environ['ZIG'] + path_delimiter + os.environ['PATH']

    # set proxy env
    proxy_schema, proxy_port, proxy_ip = get_default_proxy()
    if proxy_schema and proxy_port and proxy_ip:
        proxy_host_port = f'{proxy_ip}:{proxy_port}'
        proxy_schema_host_port = f'{proxy_schema}://{proxy_host_port}'
        color_print(f'    set proxy {proxy_schema_host_port}')
        os.environ['HTTP_PROXY'] = os.environ.get('HTTP_PROXY', proxy_schema_host_port)
        os.environ['HTTPS_PROXY'] = os.environ.get('HTTPS_PROXY', proxy_schema_host_port)
        os.environ['NO_PROXY'] = os.environ.get('NO_PROXY', f'localhost,127.0.0.1,{proxy_host_port}')


def install_cargo_zig_build():
    color_print(inspect.currentframe().f_code.co_name)

    installed = [line.strip() for line in subprocess.run(['cargo', '--list'], stdout=subprocess.PIPE, universal_newlines=True).stdout.strip().splitlines()]
    if 'zigbuild' not in installed:
        shell(['cargo', 'install', 'cargo-zigbuild'])


def get_rust_targets(glibc_version = ''):
    return [
        'x86_64-pc-windows-msvc' if platform.system() == 'Windows' else 'x86_64-pc-windows-gnu',
        'aarch64-pc-windows-msvc' if platform.system() == 'Windows' else 'aarch64-pc-windows-gnullvm',
        'x86_64-apple-darwin',
        'aarch64-apple-darwin',
        f'x86_64-unknown-linux-gnu{glibc_version}',
        f'aarch64-unknown-linux-gnu{glibc_version}',
    ]


def add_rust_targets():
    color_print(inspect.currentframe().f_code.co_name)

    installed = subprocess.run(['rustup', 'target', 'list', '--installed'], stdout=subprocess.PIPE, universal_newlines=True).stdout.strip().splitlines()

    for target in get_rust_targets():
        if target not in installed:
            shell(['rustup', 'target', 'add', target])


def build_rust_targets():
    color_print(inspect.currentframe().f_code.co_name)

    for target in get_rust_targets('.2.17'):
        shell(['cargo', 'build' if '-windows-msvc' in target else 'zigbuild', '--release', '--target', target])


def get_package_version():
    color_print(inspect.currentframe().f_code.co_name)

    with open('asc_bin/Cargo.toml') as f:
        for line in f:
            if line.startswith('version'):
                return line.partition('=')[-1].strip().strip('"')


def package(target, version):
    color_print(inspect.currentframe().f_code.co_name)

    # make dirs
    publish_dir = 'target/publish'
    dir_name = f'{target}-{version}'
    dir_path = f'{publish_dir}/{dir_name}'
    os.makedirs(dir_path, exist_ok=True)
    color_print(f'    makedirs {dir_path}')

    # copy file
    src_file_path = f'target/{target}/release/asc{".exe" if "-windows-" in target else ""}'
    shutil.copy(src_file_path, dir_path)
    color_print(f'    copy {src_file_path} to {dir_path}')
    
    # compress
    os.chdir(publish_dir)
    shutil.make_archive(base_name=dir_name, base_dir=dir_name, format='zip' if '-windows-' in target else 'xztar')
    color_print(f'    compress {dir_name}')
    shutil.rmtree(dir_name)
    os.chdir('../..')


def package_rust_targets():
    version = get_package_version()
    for target in get_rust_targets():
        package(target, version)


if __name__ == '__main__':
    set_env()
    install_cargo_zig_build()
    add_rust_targets()
    build_rust_targets()
    package_rust_targets()
