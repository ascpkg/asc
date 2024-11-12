import clang.cindex;


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
    # CXTranslationUnit_SingleFileParse |  # don't parse include files recursively
    # CXTranslationUnit_RetainExcludedConditionalBlocks |  # keep includes inside ifdef blocks
    CXTranslationUnit_KeepGoing  # don't stop on errors
)


def get_cursor_info(cursor):
    if cursor.kind == clang.cindex.CursorKind.INCLUSION_DIRECTIVE:
        return f'Include: {cursor.spelling}'

    elif cursor.kind in (clang.cindex.CursorKind.FUNCTION_DECL, 
                          clang.cindex.CursorKind.CXX_METHOD,
                          clang.cindex.CursorKind.CONSTRUCTOR,
                          clang.cindex.CursorKind.DESTRUCTOR):
        func_name = cursor.spelling
        param_types = [arg.type.spelling for arg in cursor.get_arguments()]
        return f'Function: {func_name}({', '.join(param_types)})'

    elif cursor.kind == clang.cindex.CursorKind.CLASS_DECL:
        return f'Class: {cursor.spelling}'

    elif cursor.kind == clang.cindex.CursorKind.STRUCT_DECL:
        return f'Struct: {cursor.spelling}'

    elif cursor.kind == clang.cindex.CursorKind.ENUM_DECL:
        return f'Enum: {cursor.spelling}'

    elif cursor.kind == clang.cindex.CursorKind.UNION_DECL:
        return f'Union: {cursor.spelling}'

    elif cursor.kind == clang.cindex.CursorKind.VAR_DECL:
        return f'Variable: {cursor.spelling}'

    elif cursor.kind == clang.cindex.CursorKind.TYPEDEF_DECL:
        return f'Typedef: {cursor.spelling}'

    return None

def visit_cursor(cursor):
    info = get_cursor_info(cursor)
    if info:
        print(info)
    for child in cursor.get_children():
        visit_cursor(child)

def parse_source_file(source_file):
    index = clang.cindex.Index.create()
    translation_unit = index.parse(source_file, args=['-I', 'test_sources/test_package/src'], options=default_parser_options)
    
    if translation_unit is None:
        print(f'Error parsing: {source_file}')
        return

    print(f'Parsing: {source_file}')
    visit_cursor(translation_unit.cursor)


if __name__ == '__main__':
    # clang -I src -Xclang -ast-dump -fsyntax-only src/c/c.cpp
    clang.cindex.Config.set_library_path('D:/Program Files/LLVM/bin')
    parse_source_file('test_sources/test_package/src/c/c.cpp')
