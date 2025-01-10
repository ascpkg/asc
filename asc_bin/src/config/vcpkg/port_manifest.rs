use std::collections::BTreeMap;

use regex::Regex;
use serde::{Deserialize, Serialize};

use config_file_derives::ConfigFile;
use config_file_types;

use crate::util;

use super::versions_baseline::VcpkgPortVersion;

static SOURCE_PREFIX: &str = "Source:";
static VERSION_PREFIX: &str = "Version:";
static VERSION_DATE_PREFIX: &str = "Version-Date:";
static VERSION_SEMVER_PREFIX: &str = "Version-Semver:";
static VERSION_STRING_PREFIX: &str = "Version-String:";
static PORT_VERSION_PREFIX: &str = "Port-Version:";
static BUILD_DEPENDS_PREFIX: &str = "Build-Depends:";

const REGEX_PORT_NAME_MULTIPLE_DASHES: &str = r"-+";
const REGEX_PORT_NAME_INVALID_CHARS: &str = r"[^a-zA-Z0-9\-]";

/// https://learn.microsoft.com/en-us/vcpkg/reference/vcpkg-json
#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("json")]
#[serde(rename_all = "kebab-case")]
pub struct VcpkgPortManifest {
    #[serde(skip)]
    path: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "$comment")]
    comment: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "$schema")]
    schema: Option<String>,

    name: String,

    /// https://learn.microsoft.com/en-us/vcpkg/reference/vcpkg-json#version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_semver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_string: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_version: Option<VcpkgU32OrStrField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    maintainers: Option<VcpkgStrOrVecStrField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<VcpkgStrOrVecStrField>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    homepage: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    documentation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    license: Option<String>,

    /// https://learn.microsoft.com/en-us/vcpkg/reference/vcpkg-json#platform-expression
    #[serde(default, skip_serializing_if = "String::is_empty")]
    supports: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    dependencies: Vec<VcpkgPortDependency>,

    /// https://learn.microsoft.com/en-us/vcpkg/reference/vcpkg-json#dependency
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    default_features: Vec<VcpkgPortDefaultFeature>,

    /// https://learn.microsoft.com/en-us/vcpkg/reference/vcpkg-json#feature-object
    #[serde(skip_serializing_if = "Option::is_none")]
    features: Option<VcpkgPortFeatures>,

    #[serde(skip_serializing_if = "Option::is_none")]
    builtin_baseline: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<VcpkgVersionOverrides>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum VcpkgU32OrStrField {
    U32(u32),
    Str(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum VcpkgStrOrVecStrField {
    Str(String),
    VecStr(Vec<String>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum VcpkgPortFeatures {
    Vec(Vec<VcpkgPortFeature>),
    Map(BTreeMap<String, VcpkgPortFeature>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum VcpkgPortDependency {
    Str(String),
    Map(DependencyDetails),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum VcpkgPortDefaultFeature {
    Str(String),
    Map(VcpkgFeatureSummary),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct VcpkgFeatureSummary {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct VcpkgPortFeature {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<VcpkgStrOrVecStrField>,

    /// https://learn.microsoft.com/en-us/vcpkg/reference/vcpkg-json#platform-expression
    #[serde(default, skip_serializing_if = "String::is_empty")]
    supports: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    dependencies: Vec<VcpkgPortDependency>,
}

/// https://learn.microsoft.com/en-us/vcpkg/reference/vcpkg-json#dependency
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct DependencyDetails {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_features: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    features: Vec<VcpkgPortDefaultFeature>,
    /// https://learn.microsoft.com/en-us/vcpkg/reference/vcpkg-json#platform-expression
    #[serde(skip_serializing_if = "Option::is_none")]
    platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "version>=")]
    version_ge: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
struct VcpkgVersionOverrides {
    name: String,

    /// https://learn.microsoft.com/en-us/vcpkg/reference/vcpkg-json#version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_semver: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_string: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_version: Option<VcpkgU32OrStrField>,
}

impl VcpkgPortManifest {
    pub fn update_vcpkg_json_file(
        path: &str,
        all_port_versions: &BTreeMap<String, String>,
    ) -> String {
        if !util::fs::is_file_exists(&path) {
            return String::new();
        }

        let mut data = Self::load(path, false).unwrap();

        let port_version = Self::parse_port_version(&data.port_version);
        let (name, version) = Self::build_version_suffix_name(
            &data.name,
            &data.version,
            &data.version_date,
            &data.version_semver,
            &data.version_string,
            &port_version,
        );
        data.name = name;

        // update dependencies
        let mut deps: Vec<VcpkgPortDependency> = data.dependencies.clone();
        Self::add_version_suffix_to_deps(&mut deps, all_port_versions);
        data.dependencies = deps;

        // update features' dependencies
        if let Some(features) = &mut data.features {
            match features {
                VcpkgPortFeatures::Vec(vec_features) => {
                    for feature_desc in vec_features {
                        let mut deps = feature_desc.dependencies.clone();
                        Self::add_version_suffix_to_deps(&mut deps, all_port_versions);
                        feature_desc.dependencies = deps;
                    }
                }
                VcpkgPortFeatures::Map(map_features) => {
                    for (_feature_name, feature_desc) in map_features {
                        let mut deps = feature_desc.dependencies.clone();
                        Self::add_version_suffix_to_deps(&mut deps, all_port_versions);
                        feature_desc.dependencies = deps;
                    }
                }
            }
        }

        std::fs::rename(path, format!("{path}.bak")).unwrap();
        data.dump(true, false);

        return version;
    }

    fn add_version_suffix_to_deps(
        deps: &mut Vec<VcpkgPortDependency>,
        all_port_versions: &BTreeMap<String, String>,
    ) {
        for index in 0..deps.len() {
            match deps[index].clone() {
                VcpkgPortDependency::Str(str_dep) => {
                    if let Some(version) = all_port_versions.get(&str_dep) {
                        deps[index] = VcpkgPortDependency::Str(format!("{str_dep}-{version}"));
                    }
                }
                VcpkgPortDependency::Map(mut map_dep) => {
                    if let Some(version) = all_port_versions.get(&map_dep.name) {
                        map_dep.name = format!("{}-{version}", map_dep.name);
                        deps[index] = VcpkgPortDependency::Map(map_dep);
                    }
                }
            }
        }
    }

    fn parse_port_version(value: &Option<VcpkgU32OrStrField>) -> Option<u32> {
        let mut port_version = Some(0);
        if let Some(v) = value {
            match v {
                VcpkgU32OrStrField::U32(u32_v) => {
                    port_version = Some(u32_v.clone());
                }
                VcpkgU32OrStrField::Str(str_v) => {
                    port_version = Some(str_v.parse::<u32>().unwrap_or(0));
                }
            }
        }
        return port_version;
    }

    pub fn update_control_file(path: &str, all_port_versions: &BTreeMap<String, String>) -> String {
        if !util::fs::is_file_exists(&path) {
            return String::new();
        }

        let text = std::fs::read_to_string(path).unwrap();
        let version = Self::get_version_from_control_file(&text);

        let mut lines = text
            .lines()
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        for no in 0..lines.len() {
            let line = &lines[no];
            if line.starts_with(SOURCE_PREFIX) {
                lines[no] = format!("{line}-{version}");
            } else if line.starts_with(BUILD_DEPENDS_PREFIX) {
                let mut deps = line
                    .split_at(BUILD_DEPENDS_PREFIX.len())
                    .1
                    .trim()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                for index in 0..deps.len() {
                    let dep = &deps[index];
                    if dep.contains("[") && dep.contains("]") {
                        let (mut name, features_and_platform) = dep.split_once("[").unwrap();
                        name = name.trim();
                        if let Some(version) = all_port_versions.get(name) {
                            deps[index] = format!("{name}-{version}[{features_and_platform}");
                        }
                    } else if dep.contains("(") && dep.contains(")") {
                        let (mut name, platform) = dep.split_once("(").unwrap();
                        name = name.trim();
                        if let Some(version) = all_port_versions.get(name) {
                            deps[index] = format!("{name}-{version} ({platform}");
                        }
                    } else {
                        if let Some(version) = all_port_versions.get(dep) {
                            deps[index] = format!("{dep}-{version}");
                        }
                    }
                }
                lines[no] = format!("{BUILD_DEPENDS_PREFIX} {}", deps.join(", "));
            }
        }

        std::fs::rename(path, format!("{path}.bak")).unwrap();
        std::fs::write(path, (lines.join("\n") + "\n").as_bytes()).unwrap();

        return version;
    }

    pub fn get_version_from_vcpkg_json_file(text: &str) -> String {
        let data = VcpkgPortManifest::loads(text, false).unwrap();

        let port_version = Self::parse_port_version(&data.port_version);
        let (_name, version) = Self::build_version_suffix_name(
            &data.name,
            &data.version,
            &data.version_date,
            &data.version_semver,
            &data.version_string,
            &port_version,
        );
        return version;
    }

    pub fn get_versions_from_vcpkg_json_file(
        text: &str,
    ) -> (
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        u32,
    ) {
        let data = VcpkgPortManifest::loads(text, false).unwrap();
        let port_version = Self::parse_port_version(&data.port_version);
        return (
            data.version,
            data.version_date,
            data.version_semver,
            data.version_string,
            port_version.unwrap_or(0),
        );
    }

    fn build_version_suffix_name(
        name: &str,
        version: &Option<String>,
        version_date: &Option<String>,
        version_semver: &Option<String>,
        version_string: &Option<String>,
        port_version: &Option<u32>,
    ) -> (String, String) {
        let mut versions = vec![];
        if let Some(v) = version {
            versions.push(v.clone());
        }
        if let Some(v) = version_date {
            versions.push(v.clone());
        }
        if let Some(v) = version_semver {
            versions.push(v.clone());
        }
        if let Some(v) = version_string {
            versions.push(v.clone());
        }
        if let Some(v) = port_version {
            versions.push(format!("{v}"));
        } else {
            versions.push(String::from("0"));
        }

        let v = Self::normalize_port_name(versions.join("-"));
        return (format!("{name}-{v}"), v);
    }

    pub fn normalize_port_name(name: String) -> String {
        let re_invalid_chars = Regex::new(REGEX_PORT_NAME_INVALID_CHARS).unwrap();
        let s = re_invalid_chars.replace_all(&name, "-");
        let re_multiple_dashes = Regex::new(REGEX_PORT_NAME_MULTIPLE_DASHES).unwrap();
        return re_multiple_dashes.replace_all(&s, "-").to_string();
    }

    pub fn remove_version_suffix(
        name: &String,
        version_info: &VcpkgPortVersion,
    ) -> (String, String) {
        let v = format!("{}#{}", version_info.baseline, version_info.port_version);
        let suffix = VcpkgPortManifest::normalize_port_name(format!("-{v}"));
        let re = Regex::new(&suffix).unwrap();
        return (re.replacen(&name, 1, "").to_string(), v);
    }

    pub fn get_version_from_control_file(text: &str) -> String {
        let (version, version_date, version_semver, version_string, port_version) =
            Self::get_versions_from_control_file(text);

        return Self::build_version_suffix_name(
            "",
            &version,
            &version_date,
            &version_semver,
            &version_string,
            &Some(port_version),
        )
        .1;
    }

    pub fn get_versions_from_control_file(
        text: &str,
    ) -> (
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        u32,
    ) {
        let mut version = None;
        let mut version_date = None;
        let mut version_semver = None;
        let mut version_string = None;
        let mut port_version = 0u32;

        let mut versions = 0;
        for line in text.lines() {
            if line.starts_with(VERSION_PREFIX) {
                versions += 1;
                version = Some(line.split_at(VERSION_PREFIX.len()).1.trim().to_string());
            } else if line.starts_with(VERSION_DATE_PREFIX) {
                versions += 1;
                version_date = Some(
                    line.split_at(VERSION_DATE_PREFIX.len())
                        .1
                        .trim()
                        .to_string(),
                );
            } else if line.starts_with(VERSION_SEMVER_PREFIX) {
                versions += 1;
                version_semver = Some(
                    line.split_at(VERSION_SEMVER_PREFIX.len())
                        .1
                        .trim()
                        .to_string(),
                );
            } else if line.starts_with(VERSION_STRING_PREFIX) {
                versions += 1;
                version_string = Some(
                    line.split_at(VERSION_STRING_PREFIX.len())
                        .1
                        .trim()
                        .to_string(),
                );
            } else if line.starts_with(PORT_VERSION_PREFIX) {
                versions += 1;
                port_version = line
                    .split_at(PORT_VERSION_PREFIX.len())
                    .1
                    .trim()
                    .parse::<u32>()
                    .unwrap_or(0);
            }
            if versions == 2 {
                break;
            }
        }

        return (
            version,
            version_date,
            version_semver,
            version_string,
            port_version,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::relative_paths::{VCPKG_BASELINE_JSON_FILE_NAME, VCPKG_VERSIONS_DIR_NAME};

    const FFMPEG_CONTROL_COMMIT_ID: &str = "373915929eac1d0219474c18a6e8a3134783dfc5";
    const FFMPEG_VCPKG_JSON_COMMIT_ID: &str = "44e8841e065a1b14340c6c0bb90210b11d7c3d4d";

    const FFMPEG_CONTROL_CONTENT_VERSIONED: &str = r#"Source: ffmpeg-4-3-2-11
Version: 4.3.2
Port-Version: 11
Homepage: https://ffmpeg.org
Description: a library to decode, encode, transcode, mux, demux, stream, filter and play pretty much anything that humans and machines have created.
  FFmpeg is the leading multimedia framework, able to decode, encode, transcode, mux, demux, stream, filter and play pretty much anything that humans and machines have created. It supports the most obscure ancient formats up to the cutting edge. No matter if they were designed by some standards committee, the community or a corporation. It is also highly portable: FFmpeg compiles, runs, and passes our testing infrastructure FATE across Linux, Mac OS X, Microsoft Windows, the BSDs, Solaris, etc. under a wide variety of build environments, machine architectures, and configurations.
Default-Features: avresample, avcodec, avformat, avdevice, avfilter, postproc, swresample, swscale

Feature: ffmpeg
Build-Depends: ffmpeg-4-3-2-11[core,avcodec,avfilter,avformat]
Description: Build the ffmpeg application

Feature: ffplay
Build-Depends: ffmpeg-4-3-2-11[core,avcodec,avfilter,avformat,swscale,swresample,sdl2]
Description: Build the ffplay application

Feature: ffprobe
Build-Depends: ffmpeg-4-3-2-11[core,avcodec,avformat]
Description: Build the ffprobe application

Feature: avcodec
Description: Build the avcodec library

Feature: avformat
Build-Depends: ffmpeg-4-3-2-11[core,avcodec]
Description: Build the avformat library

Feature: avdevice
Build-Depends: ffmpeg-4-3-2-11[core,avcodec,avformat]
Description: Build the avdevice library

Feature: avfilter
Description: Build the avfilter library

Feature: postproc
Build-Depends: ffmpeg-4-3-2-11[core,gpl]
Description: Build the postproc library

Feature: swresample
Description: Build the swresample library

Feature: swscale
Description: Build the swscale library

Feature: avresample
Description: Build the avresample library

Feature: nonfree
Description: Allow use of nonfree code, the resulting libs and binaries will be unredistributable

Feature: gpl
Description: Allow use of GPL code, the resulting libs and binaries will be under GPL

Feature: version3
Description: Upgrade (L)GPL to version 3

Feature: all
Build-Depends: ffmpeg-4-3-2-11[bzip2,iconv,freetype,lzma,mp3lame,openh264,openjpeg,opus,snappy,soxr,speex,theora,vorbis,vpx,webp,zlib], ffmpeg-4-3-2-11[ass] (!(uwp | arm)), ffmpeg-4-3-2-11[dav1d] (!(uwp | arm | x86 | osx)), ffmpeg-4-3-2-11[fontconfig] (!(windows & static) & !(uwp | arm)), ffmpeg-4-3-2-11[fribidi] (!(uwp | arm)), ffmpeg-4-3-2-11[ilbc] (!(arm & uwp)), ffmpeg-4-3-2-11[modplug] (!(windows & static) & !uwp), ffmpeg-4-3-2-11[nvcodec] ((windows | linux) & !uwp & !arm), ffmpeg-4-3-2-11[opencl] (!uwp), ffmpeg-4-3-2-11[ssh] (!(uwp | arm) & !static), ffmpeg-4-3-2-11[opengl] (!uwp & !(windows & arm) & !osx), ffmpeg-4-3-2-11[sdl2] (!osx), ffmpeg-4-3-2-11[tensorflow] (!(x86 | arm | uwp) & !static), ffmpeg-4-3-2-11[tesseract] (!uwp & !(windows & arm) & !static), ffmpeg-4-3-2-11[wavpack] (!arm), ffmpeg-4-3-2-11[xml2] (!static)
Description: Build with all allowed dependencies selected that are compatible with the lgpl license

Feature: all-gpl
Build-Depends: ffmpeg-4-3-2-11[gpl,all], ffmpeg-4-3-2-11[avisynthplus] (windows & !arm & !uwp & !static), ffmpeg-4-3-2-11[x264] (!arm), ffmpeg-4-3-2-11[x265] (!arm & !uwp)
Description: Build with all allowed dependencies selected that are compatible with the gpl license

Feature: all-nonfree
Build-Depends: ffmpeg-4-3-2-11[nonfree,all-gpl,openssl], ffmpeg-4-3-2-11[fdk-aac] (!arm & !uwp)
Description: Build with all allowed dependencies selected with a non-redistributable license

Feature: ass
Build-Depends: libass-0-15-0-1
Description: Libass subtitles rendering, needed for subtitles and ass filter support in ffmpeg

Feature: avisynthplus
Build-Depends: avisynthplus-3-7-0-0, ffmpeg-4-3-2-11[core,gpl]
Description: Reading of AviSynth script files

Feature: bzip2
Build-Depends: bzip2-1-0-8-1
Description: Bzip2 support

Feature: dav1d
Build-Depends: dav1d-0-8-2-0
Description: AV1 decoding via libdav1d

Feature: iconv
Build-Depends: libiconv-1-16-8
Description: Iconv support

Feature: ilbc
Build-Depends: libilbc-3-0-3-0
Description: iLBC de/encoding via libilbc

Feature: fdk-aac
Build-Depends: fdk-aac-2018-07-08-3, ffmpeg-4-3-2-11[core,nonfree]
Description: AAC de/encoding via libfdk-aac

Feature: fontconfig
Build-Depends: fontconfig-2-13-1-7
Description: Useful for drawtext filter

Feature: freetype
Build-Depends: freetype-2-10-4-0
Description: Needed for drawtext filter

Feature: fribidi
Build-Depends: fribidi-1-0-10-2
Description: Improves drawtext filter

Feature: lzma
Build-Depends: liblzma-5-2-5-2
Description: lzma support

Feature: modplug
Build-Depends: libmodplug-0-8-9-0-7
Description: ModPlug via libmodplug

Feature: mp3lame
Build-Depends: mp3lame-3-100-6
Description: MP3 encoding via libmp3lame

Feature: nvcodec
Build-Depends: ffnvcodec-10-0-26-0-1
Description: Nvidia video decoding/encoding acceleration

Feature: opencl
Build-Depends: opencl-2-2-7
Description: OpenCL processing

Feature: opengl
Build-Depends: opengl-0-0-8, opengl-registry-2020-03-25-0
Description: OpenGL rendering

Feature: openh264
Build-Depends: openh264-2021-03-16-0
Description: H.264 de/encoding via openh264

Feature: openjpeg
Build-Depends: openjpeg-2-3-1-4
Description: JPEG 2000 de/encoding via OpenJPEG

Feature: openssl
Build-Depends: openssl-1-1-1k-0, ffmpeg-4-3-2-11[core,nonfree]
Description: Needed for https support if gnutls, libtls or mbedtls is not used

Feature: opus
Build-Depends: opus-1-3-1-5
Description: Opus de/encoding via libopus

Feature: sdl2
Build-Depends: sdl2-2-0-14-4
Description: Sdl2 support

Feature: snappy
Build-Depends: snappy-1-1-8-0
Description: Snappy compression, needed for hap encoding

Feature: soxr
Build-Depends: soxr-0-1-3-3, ffmpeg-4-3-2-11[core,swresample]
Description: Include libsoxr resampling

Feature: speex
Build-Depends: speex-1-2-0-8
Description: Speex de/encoding via libspeex

Feature: ssh
Build-Depends: libssh-0-9-5-3
Description: SFTP protocol via libssh

Feature: tensorflow
Build-Depends: tensorflow-2-4-1-0
Description: TensorFlow as a DNN module backend for DNN based filters like sr

Feature: tesseract
Build-Depends: tesseract-4-1-1-8
Description: Tesseract, needed for ocr filter

Feature: theora
Build-Depends: libtheora-1-2-0alpha1-20170719-2
Description: Theora encoding via libtheora

Feature: vorbis
Build-Depends: libvorbis-1-3-7-1
Description: Vorbis en/decoding via libvorbis, native implementation exists

Feature: vpx
Build-Depends: libvpx-1-9-0-9
Description: VP8 and VP9 de/encoding via libvpx

Feature: wavpack
Build-Depends: wavpack-5-3-0-1
Description: Wavpack encoding via libwavpack

Feature: webp
Build-Depends: libwebp-1-1-0-3
Description: WebP encoding via libwebp

Feature: x264
Build-Depends: x264-157-303c484ec828ed0-15, ffmpeg-4-3-2-11[core,gpl]
Description: H.264 encoding via x264

Feature: x265
Build-Depends: x265-3-4-4, ffmpeg-4-3-2-11[core,gpl]
Description: HEVC encoding via x265

Feature: xml2
Build-Depends: libxml2-2-9-10-6
Description: XML parsing using the C library libxml2, needed for dash demuxing support

Feature: zlib
Build-Depends: zlib-1-2-11-10
Description: zlib support
"#;

    const FFMPEG_VCPKG_JSON_CONTENT_VERSIONED: &str = r#"{
  "name": "ffmpeg-4-4-0",
  "version-string": "4.4",
  "description": [
    "a library to decode, encode, transcode, mux, demux, stream, filter and play pretty much anything that humans and machines have created.",
    "FFmpeg is the leading multimedia framework, able to decode, encode, transcode, mux, demux, stream, filter and play pretty much anything that humans and machines have created. It supports the most obscure ancient formats up to the cutting edge. No matter if they were designed by some standards committee, the community or a corporation. It is also highly portable: FFmpeg compiles, runs, and passes our testing infrastructure FATE across Linux, Mac OS X, Microsoft Windows, the BSDs, Solaris, etc. under a wide variety of build environments, machine architectures, and configurations."
  ],
  "homepage": "https://ffmpeg.org",
  "default-features": [
    "avcodec",
    "avdevice",
    "avfilter",
    "avformat",
    "postproc",
    "swresample",
    "swscale"
  ],
  "features": {
    "all": {
      "description": "Build with all allowed dependencies selected that are compatible with the lgpl license",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "avresample",
            "bzip2",
            "freetype",
            "iconv",
            "lzma",
            "mp3lame",
            "openh264",
            "openjpeg",
            "opus",
            "snappy",
            "soxr",
            "speex",
            "theora",
            "vorbis",
            "vpx",
            "webp",
            "zlib"
          ]
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "sdl2"
          ],
          "platform": "!osx"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "opencl"
          ],
          "platform": "!uwp"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "xml2"
          ],
          "platform": "!static"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "ilbc"
          ],
          "platform": "!(arm & uwp)"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "ass"
          ],
          "platform": "!(uwp | arm)"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "fribidi"
          ],
          "platform": "!(uwp | arm)"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "ssh"
          ],
          "platform": "!(uwp | arm) & !static"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "dav1d"
          ],
          "platform": "!(uwp | arm | x86 | osx)"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "modplug"
          ],
          "platform": "!(windows & static) & !uwp"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "tensorflow"
          ],
          "platform": "!(x86 | arm | uwp) & !static"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "opengl"
          ],
          "platform": "!uwp & !(windows & arm) & !osx"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "nvcodec"
          ],
          "platform": "(windows | linux) & !uwp & !arm"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "tesseract"
          ],
          "platform": "!uwp & !(windows & arm) & !static"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "fontconfig"
          ],
          "platform": "!(windows & static) & !(uwp | arm)"
        }
      ]
    },
    "all-gpl": {
      "description": "Build with all allowed dependencies selected that are compatible with the gpl license",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "all",
            "gpl"
          ]
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "x264"
          ],
          "platform": "!arm"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "x265"
          ],
          "platform": "!arm & !uwp"
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "avisynthplus"
          ],
          "platform": "windows & !arm & !uwp & !static"
        }
      ]
    },
    "all-nonfree": {
      "description": "Build with all allowed dependencies selected with a non-redistributable license",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "all-gpl",
            "nonfree",
            "openssl"
          ]
        },
        {
          "name": "ffmpeg-4-4-0",
          "features": [
            "fdk-aac"
          ],
          "platform": "!arm & !uwp"
        }
      ]
    },
    "ass": {
      "description": "Libass subtitles rendering, needed for subtitles and ass filter support in ffmpeg",
      "dependencies": [
        "libass-0-15-1-0"
      ]
    },
    "avcodec": {
      "description": "Build the avcodec library"
    },
    "avdevice": {
      "description": "Build the avdevice library",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "default-features": false,
          "features": [
            "avcodec",
            "avformat"
          ]
        }
      ]
    },
    "avfilter": {
      "description": "Build the avfilter library"
    },
    "avformat": {
      "description": "Build the avformat library",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "default-features": false,
          "features": [
            "avcodec"
          ]
        }
      ]
    },
    "avisynthplus": {
      "description": "Reading of AviSynth script files",
      "dependencies": [
        "avisynthplus-3-7-0-0",
        {
          "name": "ffmpeg-4-4-0",
          "default-features": false,
          "features": [
            "gpl"
          ]
        }
      ]
    },
    "avresample": {
      "description": "Build the avresample library"
    },
    "bzip2": {
      "description": "Bzip2 support",
      "dependencies": [
        "bzip2-1-0-8-1"
      ]
    },
    "dav1d": {
      "description": "AV1 decoding via libdav1d",
      "dependencies": [
        "dav1d-0-8-2-0"
      ]
    },
    "fdk-aac": {
      "description": "AAC de/encoding via libfdk-aac",
      "dependencies": [
        "fdk-aac-2018-07-08-3",
        {
          "name": "ffmpeg-4-4-0",
          "default-features": false,
          "features": [
            "nonfree"
          ]
        }
      ]
    },
    "ffmpeg": {
      "description": "Build the ffmpeg application",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "default-features": false,
          "features": [
            "avcodec",
            "avfilter",
            "avformat"
          ]
        }
      ]
    },
    "ffplay": {
      "description": "Build the ffplay application",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "default-features": false,
          "features": [
            "avcodec",
            "avfilter",
            "avformat",
            "sdl2",
            "swresample",
            "swscale"
          ]
        }
      ]
    },
    "ffprobe": {
      "description": "Build the ffprobe application",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "default-features": false,
          "features": [
            "avcodec",
            "avformat"
          ]
        }
      ]
    },
    "fontconfig": {
      "description": "Useful for drawtext filter",
      "dependencies": [
        "fontconfig-2-13-1-7"
      ]
    },
    "freetype": {
      "description": "Needed for drawtext filter",
      "dependencies": [
        "freetype-2-10-4-0"
      ]
    },
    "fribidi": {
      "description": "Improves drawtext filter",
      "dependencies": [
        "fribidi-1-0-10-2"
      ]
    },
    "gpl": {
      "description": "Allow use of GPL code, the resulting libs and binaries will be under GPL"
    },
    "iconv": {
      "description": "Iconv support",
      "dependencies": [
        "libiconv-1-16-8"
      ]
    },
    "ilbc": {
      "description": "iLBC de/encoding via libilbc",
      "dependencies": [
        "libilbc-3-0-3-0"
      ]
    },
    "lzma": {
      "description": "lzma support",
      "dependencies": [
        "liblzma-5-2-5-2"
      ]
    },
    "modplug": {
      "description": "ModPlug via libmodplug",
      "dependencies": [
        "libmodplug-0-8-9-0-7"
      ]
    },
    "mp3lame": {
      "description": "MP3 encoding via libmp3lame",
      "dependencies": [
        "mp3lame-3-100-6"
      ]
    },
    "nonfree": {
      "description": "Allow use of nonfree code, the resulting libs and binaries will be unredistributable"
    },
    "nvcodec": {
      "description": "Nvidia video decoding/encoding acceleration",
      "dependencies": [
        "ffnvcodec-10-0-26-0-1"
      ]
    },
    "opencl": {
      "description": "OpenCL processing",
      "dependencies": [
        "opencl-2-2-7"
      ]
    },
    "opengl": {
      "description": "OpenGL rendering",
      "dependencies": [
        "opengl-0-0-8",
        "opengl-registry-2020-03-25-0"
      ]
    },
    "openh264": {
      "description": "H.264 de/encoding via openh264",
      "dependencies": [
        "openh264-2021-03-16-0"
      ]
    },
    "openjpeg": {
      "description": "JPEG 2000 de/encoding via OpenJPEG",
      "dependencies": [
        "openjpeg-2-3-1-4"
      ]
    },
    "openssl": {
      "description": "Needed for https support if gnutls, libtls or mbedtls is not used",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "default-features": false,
          "features": [
            "nonfree"
          ]
        },
        "openssl-1-1-1k-1"
      ]
    },
    "opus": {
      "description": "Opus de/encoding via libopus",
      "dependencies": [
        "opus-1-3-1-5"
      ]
    },
    "postproc": {
      "description": "Build the postproc library",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "default-features": false,
          "features": [
            "gpl"
          ]
        }
      ]
    },
    "sdl2": {
      "description": "Sdl2 support",
      "dependencies": [
        "sdl2-2-0-14-4"
      ]
    },
    "snappy": {
      "description": "Snappy compression, needed for hap encoding",
      "dependencies": [
        "snappy-1-1-8-0"
      ]
    },
    "soxr": {
      "description": "Include libsoxr resampling",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "default-features": false,
          "features": [
            "swresample"
          ]
        },
        "soxr-0-1-3-3"
      ]
    },
    "speex": {
      "description": "Speex de/encoding via libspeex",
      "dependencies": [
        "speex-1-2-0-8"
      ]
    },
    "ssh": {
      "description": "SFTP protocol via libssh",
      "dependencies": [
        "libssh-0-9-5-3"
      ]
    },
    "swresample": {
      "description": "Build the swresample library"
    },
    "swscale": {
      "description": "Build the swscale library"
    },
    "tensorflow": {
      "description": "TensorFlow as a DNN module backend for DNN based filters like sr",
      "dependencies": [
        "tensorflow-2-4-1-0"
      ]
    },
    "tesseract": {
      "description": "Tesseract, needed for ocr filter",
      "dependencies": [
        "tesseract-4-1-1-8"
      ]
    },
    "theora": {
      "description": "Theora encoding via libtheora",
      "dependencies": [
        "libtheora-1-2-0alpha1-20170719-2"
      ]
    },
    "version3": {
      "description": "Upgrade (L)GPL to version 3"
    },
    "vorbis": {
      "description": "Vorbis en/decoding via libvorbis, native implementation exists",
      "dependencies": [
        "libvorbis-1-3-7-1"
      ]
    },
    "vpx": {
      "description": "VP8 and VP9 de/encoding via libvpx",
      "dependencies": [
        "libvpx-1-9-0-9"
      ]
    },
    "webp": {
      "description": "WebP encoding via libwebp",
      "dependencies": [
        "libwebp-1-1-0-3"
      ]
    },
    "x264": {
      "description": "H.264 encoding via x264",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "default-features": false,
          "features": [
            "gpl"
          ]
        },
        "x264-157-303c484ec828ed0-15"
      ]
    },
    "x265": {
      "description": "HEVC encoding via x265",
      "dependencies": [
        {
          "name": "ffmpeg-4-4-0",
          "default-features": false,
          "features": [
            "gpl"
          ]
        },
        "x265-3-4-4"
      ]
    },
    "xml2": {
      "description": "XML parsing using the C library libxml2, needed for dash demuxing support",
      "dependencies": [
        "libxml2-2-9-10-6"
      ]
    },
    "zlib": {
      "description": "zlib support",
      "dependencies": [
        "zlib-1-2-11-10"
      ]
    }
  }
}"#;

    pub fn get_vcpkg_root_dir() -> String {
        let vcpkg_conf = crate::cli::commands::VcpkgArgs::load_or_default();
        for (name, _url, _branch, directory) in vcpkg_conf.flatten_registry() {
            if name == crate::config::relative_paths::VCPKG_DIR_NAME {
                return directory;
            }
        }
        return String::new();
    }

    fn get_all_port_versions(commit_id: &str) -> BTreeMap<String, String> {
        let vcpkg_root_dir = get_vcpkg_root_dir();
        let baseline_json_text = crate::git::show::commit_file_content(
            &vcpkg_root_dir,
            commit_id,
            &format!("{VCPKG_VERSIONS_DIR_NAME}/{VCPKG_BASELINE_JSON_FILE_NAME}"),
        );
        if let Some(baseline_data) =
            crate::config::vcpkg::versions_baseline::VcpkgBaseline::loads(&baseline_json_text, true)
        {
            let mut all_port_versions = BTreeMap::new();
            for (k, v) in baseline_data.default {
                all_port_versions.insert(
                    k,
                    VcpkgPortManifest::normalize_port_name(v.format_version_text()),
                );
            }
            return all_port_versions;
        }

        let mut all_port_versions = BTreeMap::new();
        for (port, (control_file_text, vcpkg_json_file_text)) in
            crate::git::ls_tree::list_ports(commit_id, &vcpkg_root_dir, true)
        {
            if !control_file_text.is_empty() {
                all_port_versions.insert(
                    port,
                    VcpkgPortManifest::get_version_from_control_file(&control_file_text),
                );
            } else if !vcpkg_json_file_text.is_empty() {
                all_port_versions.insert(
                    port,
                    VcpkgPortManifest::get_version_from_vcpkg_json_file(&vcpkg_json_file_text),
                );
            }
        }
        return all_port_versions;
    }

    fn get_ffmpeg_control() -> String {
        let vcpkg_root_dir = get_vcpkg_root_dir();
        crate::git::show::commit_file_content(
            &vcpkg_root_dir,
            FFMPEG_CONTROL_COMMIT_ID,
            "ports/ffmpeg/CONTROL",
        )
    }

    fn get_ffmpeg_vcpkg_json() -> String {
        let vcpkg_root_dir = get_vcpkg_root_dir();
        crate::git::show::commit_file_content(
            &vcpkg_root_dir,
            FFMPEG_VCPKG_JSON_COMMIT_ID,
            "ports/ffmpeg/vcpkg.json",
        )
    }

    #[test]
    fn test_get_version_from_control_file() {
        assert_eq!(
            String::from("4-3-2-11"),
            VcpkgPortManifest::get_version_from_control_file(&get_ffmpeg_control())
        );
    }

    #[test]
    fn test_get_version_from_vcpkg_json_file() {
        assert_eq!(
            String::from("4-4-0"),
            VcpkgPortManifest::get_version_from_vcpkg_json_file(&get_ffmpeg_vcpkg_json())
        );
    }

    #[test]
    fn test_update_control_file() {
        let path = "ffmpeg.CONTROL";
        std::fs::write(path, get_ffmpeg_control().as_bytes()).unwrap();

        let all_port_versions = get_all_port_versions(FFMPEG_CONTROL_COMMIT_ID);
        VcpkgPortManifest::update_control_file(&path, &all_port_versions);

        let is_same = is_file_text_equals(&path, FFMPEG_CONTROL_CONTENT_VERSIONED);

        std::fs::remove_file(path).unwrap();

        assert!(is_same);
    }

    #[test]
    fn test_update_vcpkg_json_file() {
        let path = "ffmpeg.vcpkg.json";
        std::fs::write(path, get_ffmpeg_vcpkg_json().as_bytes()).unwrap();

        let all_port_versions = get_all_port_versions(FFMPEG_VCPKG_JSON_COMMIT_ID);
        VcpkgPortManifest::update_vcpkg_json_file(path, &all_port_versions);

        let is_same = is_file_text_equals(&path, FFMPEG_VCPKG_JSON_CONTENT_VERSIONED);

        std::fs::remove_file(path).unwrap();

        assert!(is_same);
    }

    fn test_load_vcpkg_json_file_qt() {
        let d = VcpkgPortManifest::load("qt.json", false);
        assert!(d.is_some());
    }

    fn test_load_vcpkg_json_file_cpprestsdk() {
        let d = VcpkgPortManifest::load("cpprestsdk.json", false);
        assert!(d.is_some())
    }

    #[test]
    fn test_load_vcpkg_json_file_openvino() {
        let path: &str = "openvino.json";
        let commit_hash = "2ad5618";
        let vcpkg_root_dir = get_vcpkg_root_dir();
        let text = crate::git::show::commit_file_content(
            &vcpkg_root_dir,
            commit_hash,
            "ports/openvino/vcpkg.json",
        );
        std::fs::write(path, text.as_bytes()).unwrap();
        std::fs::write("openvino.old.json", text.as_bytes()).unwrap();

        let d = VcpkgPortManifest::loads(&text, false);
        assert!(d.is_some());

        let all_port_versions = get_all_port_versions(commit_hash);
        VcpkgPortManifest::update_vcpkg_json_file(path, &all_port_versions);
    }

    fn is_file_text_equals(path: &str, content: &str) -> bool {
        std::fs::read_to_string(path).unwrap() == content
    }
}
