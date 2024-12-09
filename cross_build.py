import inspect
import os
import platform
import shutil
import subprocess


def color_print(text):
    print(f'\033[93m{text}\033[0m')


def shell(args):
    color_print('    ' + ' '.join(args))
    subprocess.run(args)


def set_env():
    color_print(inspect.currentframe().f_code.co_name)
    
    # set macOS sdk env
    # set zig path (https://github.com/ascpkg/asc/releases/tag/zig-0.13.0-cf90dfd-20240607)
    if platform.system() == 'Windows':
        proxy_ip = '127.0.0.1'
        os.environ['SDKROOT'] = r'C:\MacOSX11.3.sdk'
        os.environ['PATH'] = r'C:\zig;' + os.environ['PATH']
    else:
        proxy_ip = '172.26.240.1'
        os.environ['SDKROOT'] = '/opt/MacOSX11.3.sdk'
        os.environ['PATH'] = '/opt/zig:' + os.environ['PATH']

    # set proxy env
    proxy_schema = 'http'
    proxy_port = 10809
    os.environ['HTTP_PROXY'] = os.environ.get('HTTP_PROXY', f'{proxy_schema}://{proxy_ip}:{proxy_port}')
    os.environ['HTTPS_PROXY'] = os.environ.get('HTTPS_PROXY', f'{proxy_schema}://{proxy_ip}:{proxy_port}')
    os.environ['NO_PROXY'] = os.environ.get('NO_PROXY', f'localhost,127.0.0.1,{proxy_ip}:{proxy_port}')


def install_cargo_zig_build():
    color_print(inspect.currentframe().f_code.co_name)

    shell(['cargo', 'install', 'cargo-zigbuild'])


def get_rust_targets(glibc_version = ''):
    return [
        'x86_64-pc-windows-msvc' if platform.system() == 'Windows' else 'x86_64-pc-windows-gnu',
        'aarch64-pc-windows-msvc' if platform.system() == 'Windows' else 'aarch64-pc-windows-gnullvm',  # not working
        'x86_64-apple-darwin',
        'aarch64-apple-darwin',
        f'x86_64-unknown-linux-gnu{glibc_version}',
        f'aarch64-unknown-linux-gnu{glibc_version}',
    ]


def add_rust_targets():
    color_print(inspect.currentframe().f_code.co_name)

    for target in get_rust_targets():
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
    src_file_path = f'target/{target}/release/asc{".exe" if "-windows-msvc" in target else ""}'
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
