import argparse
import base64
import collections
import glob
import hashlib
import http.client
import inspect
import json
import logging
import re
import os
import platform
import shutil
import subprocess
import urllib.parse
import zipfile


ALL = "all"

TARGET = "target"
RELEASE_DIR_NAME = "release"
CROSS_BUILD_DIR_NAME = "cross_build"

PLATFORM_SYSTEM_WINDOWS = "Windows"

RUST_ARCH_AMD64 = 'x86_64'
RUST_ARCH_ARM64 = 'aarch64'

RUST_TARGET_WINDOWS_PATTERN = "-windows-"
RUST_TARGET_LINUX_PATTERN = '-linux-'
RUST_TARGET_DARWIN_PATTERN = '-apple-darwin'
RUST_TARGET_WINDOWS_MSVC_PATTERN = "-windows-msvc"

HTTP = "http"
HTTP_PROXY = "HTTP_PROXY"
HTTPS_PROXY = "HTTPS_PROXY"
NO_PROXY = "NO_PROXY"
LOCAL_HOST = "localhost"
LOCAL_HOST_IP = "127.0.0.1"
DEFAULT_PROXY_PORT = 10809
IP_PATTERN = r"\s(\d+\.\d+\.\d+\.\d)\s"
IP_INFO_IO_HOST = "ipinfo.io"

WSL_DISTRO_NAME = "WSL_DISTRO_NAME"

README_MD_PATH = "asc_bin/README.md"

CARGO_TOML_PATH = "asc_bin/Cargo.toml"
CARGO_TOML_PACKAGE_NAME = "name"
CARGO_TOML_PACKAGE_VERSION = "version"
CARGO_TOML_PACKAGE_DESCRIPTION = "description"
CARGO_TOML_PACKAGE_KEYWORDS = "keywords"
CARGO_TOML_PACKAGE_LICENSE = "license"
CARGO_TOML_PACKAGE_REPOSITORY = "repository"

PYTHON_WHEEL_METADATA_HEADER = ("Metadata-Version:", "2.4")
PYTHON_WHEEL_METADATA_NAME = "Name:"
PYTHON_WHEEL_METADATA_VERSION = "Version:"
PYTHON_WHEEL_METADATA_SUMMARY = "Summary:"
PYTHON_WHEEL_METADATA_KEYWORDS = "Keywords:"
PYTHON_WHEEL_METADATA_LICENSE = "License:"
PYTHON_WHEEL_METADATA_DESCRIPTION_CONTENT_TYPE =  ("Description-Content-Type:", "text/markdown; charset=UTF-8; variant=GFM")
PYTHON_WHEEL_METADATA_PROJECT_URL = "Project-URL: Source Code,"

PATH = "PATH"
ZIG = "ZIG"
ZIG_LIB_DIR = "ZIG_LIB_DIR"
MAC_OS_SDK_NAME = "MacOSX11.3.sdk"
SDK_ROOT = "SDKROOT"


def shell(args: list, silent=False):
    logging.warning(" ".join(args))

    if not silent:
        subprocess.run(args)
    else:
        subprocess.run(args, stdout=subprocess.PIPE, stderr=subprocess.PIPE)


def is_on_windows_subsystem_linux():
    return WSL_DISTRO_NAME in os.environ


class AutoProxy:
    def __init__(self):
        self.schema = HTTP
        self.host = LOCAL_HOST_IP
        self.port = DEFAULT_PROXY_PORT

    def set_default(self):
        logging.warning(inspect.currentframe().f_code.co_name)

        (schema, ip, port) = ("", "", 0)
        if not self.is_located_china():
            return (schema, ip, port)

        # windows
        if platform.system() == PLATFORM_SYSTEM_WINDOWS:
            (schema, ip, port) = (self.schema, self.host, self.port)

        # windows subsystem linux
        if is_on_windows_subsystem_linux():
            for line in (
                subprocess.run(
                    ["ip", "route"], stdout=subprocess.PIPE, universal_newlines=True
                )
                .stdout.strip()
                .splitlines()
            ):
                if line.startswith("default"):
                    host_ip = re.search(IP_PATTERN, line).group(1)
                    (schema, ip, port) = (self.schema, host_ip, self.port)
                    break

        # try to connect proxy
        if ip and port and not self.test_default_proxy(host=ip, port=port):
            (schema, ip, port) = ("", "", 0)

        if schema and ip and ip:
            proxy_host_port = f"{ip}:{port}"
            proxy_schema_host_port = f"{schema}://{proxy_host_port}"
            logging.warning(f"set proxy {proxy_schema_host_port}")
            os.environ[HTTP_PROXY] = os.environ.get(HTTP_PROXY, proxy_schema_host_port)
            os.environ[HTTPS_PROXY] = os.environ.get(
                HTTPS_PROXY, proxy_schema_host_port
            )
            os.environ[NO_PROXY] = os.environ.get(
                NO_PROXY, f"{LOCAL_HOST},{LOCAL_HOST_IP},{proxy_host_port}"
            )

    @staticmethod
    def is_located_china():
        logging.warning(inspect.currentframe().f_code.co_name)

        conn = http.client.HTTPSConnection(IP_INFO_IO_HOST, timeout=1)

        try:
            conn.request("GET", "/json")

            response = conn.getresponse()
            text = response.read().decode()
            data = json.loads(text)

            return data.get("country") == "CN"
        except Exception as _:
            return True
        finally:
            conn.close()

    @staticmethod
    def test_default_proxy(host: str, port: int) -> bool:
        logging.warning(f'{inspect.currentframe().f_code.co_name}("{host}", {port})')

        try:
            conn = http.client.HTTPConnection(host, port, timeout=0.1)
            conn.request("HEAD", "/")
            _ = conn.getresponse()
            return True
        except Exception as _:
            return False
        finally:
            conn.close()


class PrepareRequirements:
    def prepare_zig(self):
        logging.info(inspect.currentframe().f_code.co_name)

        # set zig env
        target = os.path.join(os.getcwd(), TARGET)
        os.makedirs(target, exist_ok=True)
        zig_dir_name = f"zig-{platform.system().lower()}-x86_64-0.13.0"
        zig_path = os.path.join(target, zig_dir_name)
        zig_lib_dir = os.path.join(zig_path, "lib")
        if (
            platform.system() == PLATFORM_SYSTEM_WINDOWS
            or is_on_windows_subsystem_linux()
        ):
            os.environ[ZIG] = zig_path
            os.environ[ZIG_LIB_DIR] = zig_lib_dir
            os.environ[PATH] = os.pathsep.join([zig_path, os.environ[PATH]])

        # download and extract zig
        if (
            platform.system() == PLATFORM_SYSTEM_WINDOWS
            or is_on_windows_subsystem_linux()
        ):
            file_name = f'{zig_dir_name}.{"zip" if platform.system() == PLATFORM_SYSTEM_WINDOWS else "tar.xz"}'
            dir_path = os.path.join(TARGET, zig_dir_name)
            file_path = os.path.join(TARGET, file_name)
            url = f"https://github.com/ascpkg/asc/releases/download/zig-0.13.0-cf90dfd-20240607/{file_name}"

            if os.path.exists(dir_path) and self.is_bin_exists("zig"):
                return
            if not os.path.exists(file_path):
                self.download_file(url=url, path=file_path)
            if os.path.exists(file_path):
                self.extract_file(path=file_path, dir=TARGET)

    def prepare_mac_os_sdk(self):
        logging.info(inspect.currentframe().f_code.co_name)

        # set macOS sdk env
        target = os.path.join(os.getcwd(), TARGET)
        os.makedirs(target, exist_ok=True)
        mac_os_sdk_path = os.path.join(target, MAC_OS_SDK_NAME)
        os.environ[SDK_ROOT] = os.environ.get(SDK_ROOT, mac_os_sdk_path)

        # download and extract macOS sdk
        file_name = f"{MAC_OS_SDK_NAME}.tar.xz"
        dir_path = os.path.join(TARGET, MAC_OS_SDK_NAME)
        file_path = os.path.join(TARGET, file_name)
        url = f"https://github.com/ascpkg/asc/releases/download/MacOSX11.3.sdk/{file_name}"

        if os.path.exists(dir_path):
            return
        if not os.path.exists(file_path):
            self.download_file(url=url, path=file_path)
        if os.path.exists(file_path):
            self.extract_file(path=file_path, dir=TARGET)

    def prepare_cargo_zig_build(self, use_python_pip: bool = False):
        logging.info(inspect.currentframe().f_code.co_name)

        if use_python_pip:
            shell(args=["python" if platform.system() == PLATFORM_SYSTEM_WINDOWS else "python3", "-m", "pip", "install", "cargo-zigbuild"])
        else:
            installed = [
                line.strip()
                for line in subprocess.run(
                    ["cargo", "--list"], stdout=subprocess.PIPE, universal_newlines=True
                )
                .stdout.strip()
                .splitlines()
            ]
            if "zigbuild" not in installed:
                shell(args=["cargo", "install", "cargo-zigbuild"])

    @staticmethod
    def is_bin_exists(command: str):
        try:
            subprocess.run([command], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        except FileNotFoundError as _:
            return False
        else:
            return True

    def download_file(self, url: str, path: str):
        logging.warning(f'{inspect.currentframe().f_code.co_name}("{url}", "{path}")')

        parsed_url = urllib.parse.urlparse(url)

        proxy = os.environ.get(HTTPS_PROXY)
        if not proxy:
            conn = http.client.HTTPSConnection(parsed_url.netloc, timeout=15)
        else:
            parsed_proxy = urllib.parse.urlparse(proxy)
            proxy_ip, _, proxy_port = parsed_proxy.netloc.partition(":")
            conn = http.client.HTTPSConnection(proxy_ip, int(proxy_port), timeout=15)
            conn.set_tunnel(parsed_url.netloc)

        try:
            conn.request(
                "GET",
                parsed_url.path + ("?" + parsed_url.query if parsed_url.query else ""),
            )
            response = conn.getresponse()

            check_size = 256 * 1024
            if response.status == 200:
                with open(path, "wb") as file:
                    while True:
                        chunk = response.read(check_size)
                        if not chunk:
                            break
                        file.write(chunk)
            elif response.status == 302:
                new_url = response.getheader("Location")
                self.download_file(url=new_url, path=path)

        except Exception as _:
            pass
        finally:
            conn.close()

    def extract_file(self, path: str, dir: str):
        logging.warning(f'{inspect.currentframe().f_code.co_name}("{path}", "{dir}")')

        cwd = os.getcwd()
        os.chdir(dir)

        if platform.system() == PLATFORM_SYSTEM_WINDOWS:
            shell(args=["tar", "-xf", os.path.basename(path)])
        else:
            if path.endswith(".zip"):
                shell(args=["unzip", os.path.basename(path)])
            if path.endswith(".tar.xz"):
                shell(args=["tar", "-Jxf", os.path.basename(path)])

        os.chdir(cwd)


class BuildRustTargets:
    def __init__(self, target):
        self.target = target
        self.name, self.version, self.description, self.keywords, self.license, self.repository = self.get_package_info()

    def add_rust_targets(self):
        logging.info(f"{inspect.currentframe().f_code.co_name}")

        installed = (
            subprocess.run(
                ["rustup", TARGET, "list", "--installed"],
                stdout=subprocess.PIPE,
                universal_newlines=True,
            )
            .stdout.strip()
            .splitlines()
        )

        for target in self.get_rust_targets(target_pattern=self.target):
            if target not in installed:
                shell(args=["rustup", TARGET, "add", target])

    def build_rust_targets(self):
        logging.info(f"{inspect.currentframe().f_code.co_name}")

        for target in self.get_rust_targets(
            target_pattern=self.target, glibc_version=".2.17"
        ):
            shell(
                args=[
                    "cargo",
                    (
                        "build"
                        if RUST_TARGET_WINDOWS_MSVC_PATTERN in target
                        else "zigbuild"
                    ),
                    "--release",
                    "--target",
                    target,
                ]
            )

    def check_build_results(self):
        not_exists = []
        for target in self.get_rust_targets(target_pattern=self.target):
            ext = "zip" if RUST_TARGET_WINDOWS_PATTERN in target else "tar.xz"
            path = os.path.join(
                TARGET,
                CROSS_BUILD_DIR_NAME,
                f"{target}-{self.version}.{ext}",
            )
            if not os.path.exists(path):
                not_exists.append(path)
        if not_exists:
            raise FileNotFoundError(f'not exists: {", ".join(not_exists)}')

    def package_rust_targets(self):
        logging.info(f"{inspect.currentframe().f_code.co_name}")

        for target in self.get_rust_targets(target_pattern=self.target):
            self.package(target=target)

    def build_python_dist(self):
        logging.info(f"{inspect.currentframe().f_code.co_name}")

        if platform.system() == PLATFORM_SYSTEM_WINDOWS:
            self.build_python_tar_gz()

        for target in self.get_rust_targets(target_pattern=self.target):
            self.build_python_wheel(target=target)

    @staticmethod
    def get_rust_targets(target_pattern: str = "", glibc_version: str = ""):
        targets = [
            (
                "x86_64-pc-windows-msvc"
                if platform.system() == PLATFORM_SYSTEM_WINDOWS
                else "x86_64-pc-windows-gnu"
            ),
            (
                "aarch64-pc-windows-msvc"
                if platform.system() == PLATFORM_SYSTEM_WINDOWS
                else "aarch64-pc-windows-gnullvm"
            ),
            "x86_64-apple-darwin",
            "aarch64-apple-darwin",
            f"x86_64-unknown-linux-gnu{glibc_version}",
            f"aarch64-unknown-linux-gnu{glibc_version}",
        ]

        return [t for t in targets if t.startswith(target_pattern)]
    
    def build_python_tar_gz(self):
        cross_build_dir = os.path.join(TARGET, CROSS_BUILD_DIR_NAME)
        source_name = f"{self.name}_bin-{self.version}"
        source_dir = os.path.join(cross_build_dir, source_name)
        tar_gz_file = f"{source_dir}.tar.gz"

        if os.path.exists(tar_gz_file):
            os.remove(tar_gz_file)
        shutil.rmtree(source_dir, ignore_errors=True)

        shutil.copytree(f".", source_dir, ignore=shutil.ignore_patterns(".git", ".github", ".gitignore", ".vscode", "runners", "target", "test_sources", "test_source_parser", "*.o", "clang_parse.py", "rfc.c"))
        
        with open(os.path.join(source_dir, "PKG-INFO"), mode="w", encoding="utf-8") as f:
            lines = self.format_meta_data()
            f.writelines(lines)

        shutil.make_archive(source_dir, "gztar", base_dir=source_name, root_dir=cross_build_dir)
        
        shutil.rmtree(source_dir)

    def build_python_wheel(self, target):
        dist_info = self.format_python_wheel_dist_info_dir()
        data_scripts = self.format_python_wheel_data_scripts_dirs()

        # make dirs
        cross_build_dir = os.path.join(TARGET, CROSS_BUILD_DIR_NAME)
        dir_name = f"{target}-{self.version}-wheel"
        dir_path = os.path.join(cross_build_dir, dir_name)
        if os.path.exists(dir_path):
            shutil.rmtree(dir_path, ignore_errors=True)
        os.makedirs(dir_path, exist_ok=True)
        logging.warning(f"makedirs {dir_path}")
        data_scripts_dir = os.path.join(dir_path, data_scripts)
        os.makedirs(data_scripts_dir, exist_ok=True)
        logging.warning(f"makedirs {data_scripts_dir}")
        dist_info_dir = os.path.join(dir_path, dist_info)
        os.makedirs(dist_info_dir, exist_ok=True)
        logging.warning(f"makedirs {dist_info_dir}")

        # copy files
        asc_file = f'asc{".exe" if RUST_TARGET_WINDOWS_PATTERN in target else ""}'
        src_asc_path = os.path.join(TARGET, target, RELEASE_DIR_NAME, asc_file)
        shutil.copy(src_asc_path, data_scripts_dir)
        logging.warning(f"copy {src_asc_path} to {data_scripts_dir}")

        # write METADATA
        meta_path = f"{dist_info_dir}/METADATA"
        with open(meta_path, mode="w", encoding="utf-8") as f:
            lines = self.format_meta_data()
            f.writelines(lines)

        # write WHEEL
        wheel_path = f"{dist_info_dir}/WHEEL"
        wheel_name = self.format_python_wheel_name(target)
        with open(wheel_path, mode="w", encoding="utf-8") as f:
            tag = wheel_name.replace(self.name + "_bin", "", 1).replace(self.version, "", 1).lstrip("-").rpartition(".")[0]
            lines = [
                "Wheel-Version: 1.0", "\n",
                "Generator: asc-cross-build (2025.1.6)", "\n",
                "Root-Is-Purelib: false", "\n",
                f"Tag: {tag}", "\n",
            ]
            f.writelines(lines)

        # write RECORD
        record_path = f"{dist_info_dir}/RECORD"
        asc_path = f"{data_scripts_dir}/{asc_file}"
        with open(record_path, mode="w", encoding="utf-8") as f:
            lines = [
                f"{dist_info}/METADATA,sha256={self.calculate_sha256(meta_path)},{os.path.getsize(meta_path)}", "\n",
                f"{dist_info}/WHEEL,sha256={self.calculate_sha256(wheel_path)},{os.path.getsize(wheel_path)}", "\n",
                f"{data_scripts}/{asc_file},sha256={self.calculate_sha256(asc_path)},{os.path.getsize(asc_path)}", "\n",
                f"{dist_info}/RECORD,,", "\n",
            ]
            f.writelines(lines)

        # compress
        cwd = os.getcwd()
        os.chdir(dir_path)
        with zipfile.ZipFile(f"../{wheel_name}", "w", compression=zipfile.ZIP_DEFLATED) as zip_f:
            for d in (os.path.dirname(data_scripts), dist_info):
                for folder_name, _, file_names in os.walk(d):
                    for file_name in file_names:
                        file_path = os.path.join(folder_name, file_name)
                        zip_f.write(file_path, os.path.relpath(file_path, os.path.dirname(d)))
        logging.warning(f"compress {wheel_name}")
        os.chdir(cwd)
        shutil.rmtree(dir_path)

    def format_meta_data(self) -> list:
        lines = [
            " ".join(PYTHON_WHEEL_METADATA_HEADER), "\n",
            f"{PYTHON_WHEEL_METADATA_NAME} {self.name}_bin", "\n",
            f"{PYTHON_WHEEL_METADATA_VERSION} {self.version}", "\n",
            f"{PYTHON_WHEEL_METADATA_SUMMARY} {self.description}", "\n",
            f"{PYTHON_WHEEL_METADATA_KEYWORDS} {self.keywords}", "\n",
            f"{PYTHON_WHEEL_METADATA_LICENSE} {self.license}", "\n",
            " ".join(PYTHON_WHEEL_METADATA_DESCRIPTION_CONTENT_TYPE), "\n",
            f"{PYTHON_WHEEL_METADATA_PROJECT_URL} {self.repository}" "\n",
        ]
        with open(README_MD_PATH, encoding="utf-8") as fp:
            lines.extend(["\n", fp.read()])
        return lines

    def calculate_sha256(self, file_path):
        sha256_hash = hashlib.sha256()
        with open(file_path, "rb") as f:
            for byte_block in iter(lambda: f.read(4096), b""):
                sha256_hash.update(byte_block)
        digest = sha256_hash.digest()
        return base64.urlsafe_b64encode(digest).decode("utf-8").rstrip("=")
    
    def format_python_wheel_data_scripts_dirs(self) -> str:
        return f'{self.name}_bin-{self.version}.data/scripts'
    
    def format_python_wheel_dist_info_dir(self) -> str:
        return f'{self.name}_bin-{self.version}.dist-info'
    
    def format_python_wheel_name(self, target) -> str:
        if RUST_TARGET_WINDOWS_PATTERN in target:
            if RUST_ARCH_AMD64 in target:
                return f'{self.name}_bin-{self.version}-py3-none-win_amd64.whl'
            elif RUST_ARCH_ARM64 in target:
                return f'{self.name}_bin-{self.version}-py3-none-win_arm64.whl'
        elif RUST_TARGET_LINUX_PATTERN in target:
            if RUST_ARCH_AMD64 in target:
                return f'{self.name}_bin-{self.version}-py3-none-manylinux_2_17_x86_64.manylinux2014_x86_64.whl'
            elif RUST_ARCH_ARM64 in target:
                return f'{self.name}_bin-{self.version}-py3-none-manylinux_2_17_aarch64.manylinux2014_aarch64.whl'
        elif RUST_TARGET_DARWIN_PATTERN in target:
            if RUST_ARCH_AMD64 in target:
                return f'{self.name}_bin-{self.version}-py3-none-macosx_10_9_x86_64.whl'
            elif RUST_ARCH_ARM64 in target:
                return f'{self.name}_bin-{self.version}-py3-none-macosx_11_0_arm64.whl'

    def package(self, target):
        # make dirs
        cross_build_dir = os.path.join(TARGET, CROSS_BUILD_DIR_NAME)
        dir_name = f"{target}-{self.version}"
        dir_path = os.path.join(cross_build_dir, dir_name)
        if os.path.exists(dir_path):
            shutil.rmtree(dir_path, ignore_errors=True)
        os.makedirs(dir_path, exist_ok=True)
        logging.warning(f"makedirs {dir_path}")

        # copy file
        src_file_path = os.path.join(
            TARGET,
            target,
            RELEASE_DIR_NAME,
            f'asc{".exe" if RUST_TARGET_WINDOWS_PATTERN in target else ""}',
        )
        shutil.copy(src_file_path, dir_path)
        logging.warning(f"copy {src_file_path} to {dir_path}")

        # compress
        cwd = os.getcwd()
        os.chdir(cross_build_dir)
        d = os.path.basename(dir_name)
        shutil.make_archive(
            base_name=d,
            base_dir=d,
            format="zip" if RUST_TARGET_WINDOWS_PATTERN in target else "xztar",
        )
        logging.warning(f"compress {dir_name}")
        shutil.rmtree(d)
        os.chdir(cwd)

    def get_package_info(self) -> list:
        logging.warning(inspect.currentframe().f_code.co_name)

        info = ["", "", "", "", "", ""]
        with open(CARGO_TOML_PATH, encoding="utf-8") as f:
            for line in f:
                if line.startswith(CARGO_TOML_PACKAGE_NAME):
                    info[0] = line.partition("=")[-1].strip().strip('"')
                elif line.startswith(CARGO_TOML_PACKAGE_VERSION):
                    info[1] = line.partition("=")[-1].strip().strip('"')
                elif line.startswith(CARGO_TOML_PACKAGE_DESCRIPTION):
                    info[2] = line.partition("=")[-1].strip().strip('"')
                elif line.startswith(CARGO_TOML_PACKAGE_KEYWORDS):
                    info[3] = line.partition("=")[-1].strip().strip('"[]').replace('", "', ",")
                elif line.startswith(CARGO_TOML_PACKAGE_LICENSE):
                    info[4] = line.partition("=")[-1].strip().strip('"')
                elif line.startswith(CARGO_TOML_PACKAGE_REPOSITORY):
                    info[5] = line.partition("=")[-1].strip().strip('"')
        return info


class PublishUtils:
    @staticmethod
    def publish():
        output_dir_path = os.path.join(TARGET, CROSS_BUILD_DIR_NAME)
        
        name = ""
        version = ""

        # build
        if platform.system() == PLATFORM_SYSTEM_WINDOWS:
            preparer = PrepareRequirements()
            preparer.prepare_zig()
            preparer.prepare_cargo_zig_build()

            for target in ("x86_64-pc-windows-msvc", "aarch64-pc-windows-msvc"):
                builder = BuildRustTargets(target)
                builder.add_rust_targets()
                builder.build_rust_targets()
                builder.build_python_tar_gz()
                builder.build_python_wheel(target)
                name, version = builder.name, builder.version
        else:
            # clean
            shutil.rmtree(output_dir_path, ignore_errors=True)
            os.makedirs(output_dir_path, exist_ok=True)

            preparer = PrepareRequirements()
            preparer.prepare_zig()
            preparer.prepare_cargo_zig_build()
            preparer.prepare_mac_os_sdk()

            for target in ("x86_64-apple-darwin", "aarch64-apple-darwin", "x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu"):
                builder = BuildRustTargets(target)
                builder.add_rust_targets()
                builder.build_rust_targets()
                builder.build_python_wheel(target)
                name, version = builder.name, builder.version

        # upload
        PublishUtils.publish_to_rust_crates()
        PublishUtils.publish_to_python_pypi(output_dir_path=output_dir_path, name=name, version=version)

    @staticmethod
    def publish_to_rust_crates():
        if platform.system() == PLATFORM_SYSTEM_WINDOWS:
            shell(args=["cargo", "publish", "--package=rs_container_ffi", "--no-verify"])
            shell(args=["cargo", "publish", "--package=c_source_parser_ffi", "--no-verify"])
            shell(args=["cargo", "publish", "--package=config_file_macros", "--no-verify"])
            shell(args=["cargo", "publish", "--package=config_file_types", "--no-verify"])
            shell(args=["cargo", "publish", "--package=config_file_derives", "--no-verify"])
            shell(args=["cargo", "publish", "--package=asc_bin", "--no-verify"])

    @staticmethod
    def publish_to_python_pypi(output_dir_path: str, name: str, version: str):
        if platform.system() == PLATFORM_SYSTEM_WINDOWS:
            shell(args=["maturin", "upload", "--username", "__token__", f"{output_dir_path}/{name}_bin-{version}*"])


class GitUtils:
    @staticmethod
    def get_latest_tag():
        proc = subprocess.run(["git", "describe", "--tags", "--abbrev=0"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
        print(proc.stdout.strip())

    @staticmethod
    def get_last_tag():
        proc = subprocess.run(["git", "describe", "--tags", "--abbrev=0"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
        current_tag = proc.stdout.strip()

        proc = subprocess.run(["git", "tag", "--sort=-creatordate"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)
        for line in proc.stdout.split("\n"):
            tag = line.strip()
            if tag != current_tag and re.search(pattern=r"^[0-9]{4}\.[0-9]{1,2}\.[0-9]{1,2}", string=line.strip()):
                print(tag)
                break


class FileUtils:
    @staticmethod
    def get_compressed_file(file_pattern: str):
        for path in glob.glob(os.path.join(TARGET, CROSS_BUILD_DIR_NAME, file_pattern)):
            print(path)


class ColoredStdoutLogger:
    class ColoredFormatter(logging.Formatter):
        def format(self, record):
            if record.levelno >= logging.ERROR:
                record.msg = f"\033[91m{record.msg}\033[0m"  # Red
            elif record.levelno >= logging.WARNING:
                record.msg = f"\033[93m{record.msg}\033[0m"  # Yellow
            else:
                record.msg = f"\033[94m{record.msg}\033[0m"  # Light Blue

            return super().format(record)

    def setup(self):
        logger = logging.getLogger()
        logger.setLevel(logging.INFO)

        handler = logging.StreamHandler()
        handler.setLevel(logging.INFO)

        formatter = self.ColoredFormatter(
            "%(asctime)s - %(lineno)d - %(levelname)s - %(message)s"
        )
        handler.setFormatter(formatter)

        logger.addHandler(handler)


class CommandLinesParser:
    # parse command line arguments
    command_lines = collections.namedtuple(
        "command_lines",
        (
            ALL,
            TARGET,
            "use_python_pip",
            GitUtils.get_latest_tag.__name__,
            GitUtils.get_last_tag.__name__,
            FileUtils.get_compressed_file.__name__,
            PrepareRequirements.prepare_zig.__name__,
            PrepareRequirements.prepare_mac_os_sdk.__name__,
            PrepareRequirements.prepare_cargo_zig_build.__name__,
            BuildRustTargets.add_rust_targets.__name__,
            BuildRustTargets.build_rust_targets.__name__,
            BuildRustTargets.package_rust_targets.__name__,
            BuildRustTargets.build_python_dist.__name__,
            BuildRustTargets.check_build_results.__name__,
            PublishUtils.publish.__name__,
        ),
    )

    def parse(self) -> command_lines:
        arg_parser = argparse.ArgumentParser(description="cross build")

        arg_parser.add_argument(
            f"--{ALL}",
            type=bool,
            default=False,
            help="run all tasks (default False)",
            choices=[True, False],
        )
        arg_parser.add_argument(
            f"--{TARGET}",
            type=str,
            default="",
            help='rust target name (default "")',
            choices=BuildRustTargets.get_rust_targets(),
        )
        arg_parser.add_argument(
            f"--use_python_pip",
            type=bool,
            default=False,
            help='use python pip to install cargo-zigbuild (default False)',
            choices=[True, False],
        )
        arg_parser.add_argument(
            f"--{GitUtils.get_latest_tag.__name__}",
            type=bool,
            default=False,
            help=f'{GitUtils.get_latest_tag.__name__.replace("_", " ")} (default False)',
            choices=[True, False],
        )
        arg_parser.add_argument(
            f"--{GitUtils.get_last_tag.__name__}",
            type=bool,
            default=False,
            help=f'{GitUtils.get_last_tag.__name__.replace("_", " ")} (default False)',
            choices=[True, False],
        )
        arg_parser.add_argument(
            f"--{FileUtils.get_compressed_file.__name__}",
            type=str,
            default="",
            help=f'{FileUtils.get_compressed_file.__name__.replace("_", " ")} (default "")',
        )
        arg_parser.add_argument(
            f"--{PrepareRequirements.prepare_zig.__name__}",
            type=bool,
            default=False,
            help=f'{PrepareRequirements.prepare_zig.__name__.replace("_", " ")} (default False)',
            choices=[True, False],
        )
        arg_parser.add_argument(
            f"--{PrepareRequirements.prepare_mac_os_sdk.__name__}",
            type=bool,
            default=False,
            help=f'{PrepareRequirements.prepare_mac_os_sdk.__name__.replace("_", " ")} (default False)',
            choices=[True, False],
        )
        arg_parser.add_argument(
            f"--{PrepareRequirements.prepare_cargo_zig_build.__name__}",
            type=bool,
            default=False,
            help=f'{PrepareRequirements.prepare_cargo_zig_build.__name__.replace("_", " ")} (default False)',
            choices=[True, False],
        )
        arg_parser.add_argument(
            f"--{BuildRustTargets.add_rust_targets.__name__}",
            type=bool,
            default=False,
            help=f'{BuildRustTargets.add_rust_targets.__name__.replace("_", " ")} (default False)',
            choices=[True, False],
        )
        arg_parser.add_argument(
            f"--{BuildRustTargets.build_rust_targets.__name__}",
            type=bool,
            default=False,
            help=f'{BuildRustTargets.build_rust_targets.__name__.replace("_", " ")} (default False)',
            choices=[True, False],
        )
        arg_parser.add_argument(
            f"--{BuildRustTargets.package_rust_targets.__name__}",
            type=bool,
            default=False,
            help=f'{BuildRustTargets.package_rust_targets.__name__.replace("_", " ")} (default False)',
            choices=[True, False],
        )
        arg_parser.add_argument(
            f"--{BuildRustTargets.build_python_dist.__name__}",
            type=bool,
            default=False,
            help=f'{BuildRustTargets.build_python_dist.__name__.replace("_", " ")} (default False)',
            choices=[True, False],
        )
        arg_parser.add_argument(
            f"--{BuildRustTargets.check_build_results.__name__}",
            type=bool,
            default=False,
            help=f'{BuildRustTargets.check_build_results.__name__.replace("_", " ")} (default False)',
            choices=[True, False],
        )
        arg_parser.add_argument(
            f"--{PublishUtils.publish.__name__}",
            type=bool,
            default=False,
            help=f'{PublishUtils.publish.__name__.replace("_", " ")} (default False)',
            choices=[True, False],
        )

        args = arg_parser.parse_args()

        return self.command_lines(
            all=args.all,
            target=args.target,
            use_python_pip=args.use_python_pip,
            get_latest_tag=args.get_latest_tag,
            get_last_tag=args.get_last_tag,
            get_compressed_file=args.get_compressed_file,
            prepare_zig=args.prepare_zig,
            prepare_mac_os_sdk=args.prepare_mac_os_sdk,
            prepare_cargo_zig_build=args.prepare_cargo_zig_build,
            add_rust_targets=args.add_rust_targets,
            build_rust_targets=args.build_rust_targets,
            package_rust_targets=args.package_rust_targets,
            build_python_dist=args.build_python_dist,
            check_build_results=args.check_build_results,
            publish=args.publish,
        )


if __name__ == "__main__":
    command_lines = CommandLinesParser().parse()

    if command_lines.all or command_lines.get_latest_tag:
        GitUtils.get_latest_tag()
        if not command_lines.all:
            exit(0)

    if command_lines.all or command_lines.get_last_tag:
        GitUtils.get_last_tag()
        if not command_lines.all:
            exit(0)

    if command_lines.all or command_lines.get_compressed_file:
        FileUtils.get_compressed_file(file_pattern=command_lines.get_compressed_file)
        if not command_lines.all:
            exit(0)

    ColoredStdoutLogger().setup()

    AutoProxy().set_default()
    
    preparer = PrepareRequirements()
    
    builder = BuildRustTargets(target=command_lines.target)

    if command_lines.all or command_lines.prepare_zig:
        if not command_lines.use_python_pip:
            preparer.prepare_zig()

    if command_lines.all or command_lines.prepare_mac_os_sdk:
        if not command_lines.use_python_pip:
            preparer.prepare_mac_os_sdk()

    if command_lines.all or command_lines.prepare_cargo_zig_build:
        preparer.prepare_cargo_zig_build(use_python_pip=command_lines.use_python_pip)

    if command_lines.all or command_lines.add_rust_targets:
        builder.add_rust_targets()

    if command_lines.all or command_lines.build_rust_targets:
        builder.build_rust_targets()

    if command_lines.all or command_lines.package_rust_targets:
        builder.package_rust_targets()

    if command_lines.all or command_lines.build_python_dist:
        builder.build_python_dist()

    if command_lines.all or command_lines.check_build_results:
        builder.check_build_results()

    if command_lines.all or command_lines.publish:
        PublishUtils.publish()
