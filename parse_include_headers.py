import argparse
import collections
import inspect
import logging
import os
import sys

from enum import Enum

from clang.cindex import Config, Index, CursorKind



# clang.cindex.TranslationUnit does not have all latest flags
# see: https://clang.llvm.org/doxygen/group__CINDEX__TRANSLATION__UNIT.html
CXTranslationUnit_DetailedPreprocessingRecord = 0x01
CXTranslationUnit_SkipFunctionBodies = 0x40
CXTranslationUnit_KeepGoing = 0x200
CXTranslationUnit_SingleFileParse = 0x400
CXTranslationUnit_RetainExcludedConditionalBlocks = 0x8000


default_parser_options = (
    CXTranslationUnit_DetailedPreprocessingRecord |  # needed for preprocessing parsing
    CXTranslationUnit_SkipFunctionBodies |  # for faster parsing
    CXTranslationUnit_SingleFileParse |  # don't parse include files recursively
    # CXTranslationUnit_RetainExcludedConditionalBlocks |  # keep includes inside ifdef blocks
    CXTranslationUnit_KeepGoing  # don't stop on errors
)


cli_args = collections.namedtuple(
    'cli_args',
    (
        'clang_lib_dir',
        'source',
        'args',
        'options',
    )
)


def parse_cli_args() -> cli_args:
    logging.info(inspect.currentframe().f_code.co_name)

    arg_parser = argparse.ArgumentParser(description='parse source include headers with clang ir')
    
    arg_parser.add_argument(
        '--clang-lib-dir', 
        type=str,
        help=f'set clang lib dir'
    )
    
    arg_parser.add_argument(
        '--source', 
        type=str,
        help=f'set source path to parse'
    )

    arg_parser.add_argument(
        '--args', 
        type=tuple,
        default=None,
        help=f'set clang parser args'
    )

    arg_parser.add_argument(
        '--options', 
        type=tuple,
        default=default_parser_options,
        help=f'set clang parser options'
    )

    args = arg_parser.parse_args()

    if not args.clang_lib_dir or not os.path.exists(args.clang_lib_dir) or not args.source or not os.path.exists(args.source):
        arg_parser.print_help()
        return None

    return cli_args(
        clang_lib_dir=args.clang_lib_dir,
        source=args.source,
        args=args.args,
        options=args.options
    )


class IncludeForm(Enum):
    Quoted = 0
    AngleBracket = 1


class IncludeInfo:
    def __init__(self, path, form, file=None):
        self.path = path
        self.form = form
        self.file = file

    def __str__(self):
        open_bracket, close_bracket = ('<', '>') if self.form == IncludeForm.AngleBracket else ('"', '"')
        return f'#include {open_bracket}{self.path}{close_bracket} // {self.file}'


def get_included_file(node):
    try:
        return node.get_included_file()
    except:
        return None


def parse_headers(source: str, args: tuple, options: tuple):
    index = Index.create()
    tu = index.parse(source, args=args, options=options)
    for node in tu.cursor.get_children():
        if node.kind == CursorKind.INCLUSION_DIRECTIVE:
            yield IncludeInfo(
                node.displayname,
                IncludeForm.AngleBracket if list(node.get_tokens())[-1].spelling == '>' else IncludeForm.Quoted,
                get_included_file(node)
            )

        
if __name__ == '__main__':
    args = parse_cli_args()
    if args:
        Config.set_library_path(args.clang_lib_dir)
        for include_info in parse_headers(args.source, args.args, args.options):
            print(include_info)

