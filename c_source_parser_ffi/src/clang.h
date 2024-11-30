#ifndef C_SOURCE_PARSER_FFI_CLANG_API
#define C_SOURCE_PARSER_FFI_CLANG_API

// libclang-13.0-d7b669b-20210915

/**
 * An "index" that consists of a set of translation units that would
 * typically be linked together into an executable or library.
 */
typedef void *CXIndex;

/**
 * A single translation unit, which resides in an index.
 */
typedef struct CXTranslationUnitImpl *CXTranslationUnit;

/**
 * Opaque pointer representing client data that will be passed through
 * to various callbacks and visitors.
 */
typedef void *CXClientData;

/**
 * A particular source file that is part of a translation unit.
 */
typedef void *CXFile;

/**
 * Flags that control the creation of translation units.
 *
 * The enumerators in this enumeration type are meant to be bitwise
 * ORed together to specify which options should be used when
 * constructing the translation unit.
 */
enum CXTranslationUnit_Flags {
    /**
     * Used to indicate that no special translation-unit options are
     * needed.
     */
    CXTranslationUnit_None = 0x0,
  
    /**
     * Used to indicate that the parser should construct a "detailed"
     * preprocessing record, including all macro definitions and instantiations.
     *
     * Constructing a detailed preprocessing record requires more memory
     * and time to parse, since the information contained in the record
     * is usually not retained. However, it can be useful for
     * applications that require more detailed information about the
     * behavior of the preprocessor.
     */
    CXTranslationUnit_DetailedPreprocessingRecord = 0x01,
  
    /**
     * Used to indicate that the translation unit is incomplete.
     *
     * When a translation unit is considered "incomplete", semantic
     * analysis that is typically performed at the end of the
     * translation unit will be suppressed. For example, this suppresses
     * the completion of tentative declarations in C and of
     * instantiation of implicitly-instantiation function templates in
     * C++. This option is typically used when parsing a header with the
     * intent of producing a precompiled header.
     */
    CXTranslationUnit_Incomplete = 0x02,
  
    /**
     * Used to indicate that the translation unit should be built with an
     * implicit precompiled header for the preamble.
     *
     * An implicit precompiled header is used as an optimization when a
     * particular translation unit is likely to be reparsed many times
     * when the sources aren't changing that often. In this case, an
     * implicit precompiled header will be built containing all of the
     * initial includes at the top of the main file (what we refer to as
     * the "preamble" of the file). In subsequent parses, if the
     * preamble or the files in it have not changed, \c
     * clang_reparseTranslationUnit() will re-use the implicit
     * precompiled header to improve parsing performance.
     */
    CXTranslationUnit_PrecompiledPreamble = 0x04,
  
    /**
     * Used to indicate that the translation unit should cache some
     * code-completion results with each reparse of the source file.
     *
     * Caching of code-completion results is a performance optimization that
     * introduces some overhead to reparsing but improves the performance of
     * code-completion operations.
     */
    CXTranslationUnit_CacheCompletionResults = 0x08,
  
    /**
     * Used to indicate that the translation unit will be serialized with
     * \c clang_saveTranslationUnit.
     *
     * This option is typically used when parsing a header with the intent of
     * producing a precompiled header.
     */
    CXTranslationUnit_ForSerialization = 0x10,
  
    /**
     * DEPRECATED: Enabled chained precompiled preambles in C++.
     *
     * Note: this is a *temporary* option that is available only while
     * we are testing C++ precompiled preamble support. It is deprecated.
     */
    CXTranslationUnit_CXXChainedPCH = 0x20,
  
    /**
     * Used to indicate that function/method bodies should be skipped while
     * parsing.
     *
     * This option can be used to search for declarations/definitions while
     * ignoring the usages.
     */
    CXTranslationUnit_SkipFunctionBodies = 0x40,
  
    /**
     * Used to indicate that brief documentation comments should be
     * included into the set of code completions returned from this translation
     * unit.
     */
    CXTranslationUnit_IncludeBriefCommentsInCodeCompletion = 0x80,
  
    /**
     * Used to indicate that the precompiled preamble should be created on
     * the first parse. Otherwise it will be created on the first reparse. This
     * trades runtime on the first parse (serializing the preamble takes time) for
     * reduced runtime on the second parse (can now reuse the preamble).
     */
    CXTranslationUnit_CreatePreambleOnFirstParse = 0x100,
  
    /**
     * Do not stop processing when fatal errors are encountered.
     *
     * When fatal errors are encountered while parsing a translation unit,
     * semantic analysis is typically stopped early when compiling code. A common
     * source for fatal errors are unresolvable include files. For the
     * purposes of an IDE, this is undesirable behavior and as much information
     * as possible should be reported. Use this flag to enable this behavior.
     */
    CXTranslationUnit_KeepGoing = 0x200,
  
    /**
     * Sets the preprocessor in a mode for parsing a single file only.
     */
    CXTranslationUnit_SingleFileParse = 0x400,
  
    /**
     * Used in combination with CXTranslationUnit_SkipFunctionBodies to
     * constrain the skipping of function bodies to the preamble.
     *
     * The function bodies of the main file are not skipped.
     */
    CXTranslationUnit_LimitSkipFunctionBodiesToPreamble = 0x800,
  
    /**
     * Used to indicate that attributed types should be included in CXType.
     */
    CXTranslationUnit_IncludeAttributedTypes = 0x1000,
  
    /**
     * Used to indicate that implicit attributes should be visited.
     */
    CXTranslationUnit_VisitImplicitAttributes = 0x2000,
  
    /**
     * Used to indicate that non-errors from included files should be ignored.
     *
     * If set, clang_getDiagnosticSetFromTU() will not report e.g. warnings from
     * included files anymore. This speeds up clang_getDiagnosticSetFromTU() for
     * the case where these warnings are not of interest, as for an IDE for
     * example, which typically shows only the diagnostics in the main file.
     */
    CXTranslationUnit_IgnoreNonErrorsFromIncludedFiles = 0x4000,
  
    /**
     * Tells the preprocessor not to skip excluded conditional blocks.
     */
    CXTranslationUnit_RetainExcludedConditionalBlocks = 0x8000
};

/**
 * Describes how the traversal of the children of a particular
 * cursor should proceed after visiting a particular child cursor.
 *
 * A value of this enumeration type should be returned by each
 * \c CXCursorVisitor to indicate how clang_visitChildren() proceed.
 */
enum CXChildVisitResult {
    /**
     * Terminates the cursor traversal.
     */
    CXChildVisit_Break,
    /**
     * Continues the cursor traversal with the next sibling of
     * the cursor just visited, without visiting its children.
     */
    CXChildVisit_Continue,
    /**
     * Recursively traverse the children of this cursor, using
     * the same visitor and client data.
     */
    CXChildVisit_Recurse
};

/**
 * Describes the kind of entity that a cursor refers to.
 */
enum CXCursorKind {
    /* Declarations */
    /**
     * A declaration whose specific kind is not exposed via this
     * interface.
     *
     * Unexposed declarations have the same operations as any other kind
     * of declaration; one can extract their location information,
     * spelling, find their definitions, etc. However, the specific kind
     * of the declaration is not reported.
     */
    CXCursor_UnexposedDecl = 1,
    /** A C or C++ struct. */
    CXCursor_StructDecl = 2,
    /** A C or C++ union. */
    CXCursor_UnionDecl = 3,
    /** A C++ class. */
    CXCursor_ClassDecl = 4,
    /** An enumeration. */
    CXCursor_EnumDecl = 5,
    /**
     * A field (in C) or non-static data member (in C++) in a
     * struct, union, or C++ class.
     */
    CXCursor_FieldDecl = 6,
    /** An enumerator constant. */
    CXCursor_EnumConstantDecl = 7,
    /** A function. */
    CXCursor_FunctionDecl = 8,
    /** A variable. */
    CXCursor_VarDecl = 9,
    /** A function or method parameter. */
    CXCursor_ParmDecl = 10,
    /** An Objective-C \@interface. */
    CXCursor_ObjCInterfaceDecl = 11,
    /** An Objective-C \@interface for a category. */
    CXCursor_ObjCCategoryDecl = 12,
    /** An Objective-C \@protocol declaration. */
    CXCursor_ObjCProtocolDecl = 13,
    /** An Objective-C \@property declaration. */
    CXCursor_ObjCPropertyDecl = 14,
    /** An Objective-C instance variable. */
    CXCursor_ObjCIvarDecl = 15,
    /** An Objective-C instance method. */
    CXCursor_ObjCInstanceMethodDecl = 16,
    /** An Objective-C class method. */
    CXCursor_ObjCClassMethodDecl = 17,
    /** An Objective-C \@implementation. */
    CXCursor_ObjCImplementationDecl = 18,
    /** An Objective-C \@implementation for a category. */
    CXCursor_ObjCCategoryImplDecl = 19,
    /** A typedef. */
    CXCursor_TypedefDecl = 20,
    /** A C++ class method. */
    CXCursor_CXXMethod = 21,
    /** A C++ namespace. */
    CXCursor_Namespace = 22,
    /** A linkage specification, e.g. 'extern "C"'. */
    CXCursor_LinkageSpec = 23,
    /** A C++ constructor. */
    CXCursor_Constructor = 24,
    /** A C++ destructor. */
    CXCursor_Destructor = 25,
    /** A C++ conversion function. */
    CXCursor_ConversionFunction = 26,
    /** A C++ template type parameter. */
    CXCursor_TemplateTypeParameter = 27,
    /** A C++ non-type template parameter. */
    CXCursor_NonTypeTemplateParameter = 28,
    /** A C++ template template parameter. */
    CXCursor_TemplateTemplateParameter = 29,
    /** A C++ function template. */
    CXCursor_FunctionTemplate = 30,
    /** A C++ class template. */
    CXCursor_ClassTemplate = 31,
    /** A C++ class template partial specialization. */
    CXCursor_ClassTemplatePartialSpecialization = 32,
    /** A C++ namespace alias declaration. */
    CXCursor_NamespaceAlias = 33,
    /** A C++ using directive. */
    CXCursor_UsingDirective = 34,
    /** A C++ using declaration. */
    CXCursor_UsingDeclaration = 35,
    /** A C++ alias declaration */
    CXCursor_TypeAliasDecl = 36,
    /** An Objective-C \@synthesize definition. */
    CXCursor_ObjCSynthesizeDecl = 37,
    /** An Objective-C \@dynamic definition. */
    CXCursor_ObjCDynamicDecl = 38,
    /** An access specifier. */
    CXCursor_CXXAccessSpecifier = 39,

    CXCursor_FirstDecl = CXCursor_UnexposedDecl,
    CXCursor_LastDecl = CXCursor_CXXAccessSpecifier,

    /* References */
    CXCursor_FirstRef = 40, /* Decl references */
    CXCursor_ObjCSuperClassRef = 40,
    CXCursor_ObjCProtocolRef = 41,
    CXCursor_ObjCClassRef = 42,
    /**
     * A reference to a type declaration.
     *
     * A type reference occurs anywhere where a type is named but not
     * declared. For example, given:
     *
     * \code
     * typedef unsigned size_type;
     * size_type size;
     * \endcode
     *
     * The typedef is a declaration of size_type (CXCursor_TypedefDecl),
     * while the type of the variable "size" is referenced. The cursor
     * referenced by the type of size is the typedef for size_type.
     */
    CXCursor_TypeRef = 43,
    CXCursor_CXXBaseSpecifier = 44,
    /**
     * A reference to a class template, function template, template
     * template parameter, or class template partial specialization.
     */
    CXCursor_TemplateRef = 45,
    /**
     * A reference to a namespace or namespace alias.
     */
    CXCursor_NamespaceRef = 46,
    /**
     * A reference to a member of a struct, union, or class that occurs in
     * some non-expression context, e.g., a designated initializer.
     */
    CXCursor_MemberRef = 47,
    /**
     * A reference to a labeled statement.
     *
     * This cursor kind is used to describe the jump to "start_over" in the
     * goto statement in the following example:
     *
     * \code
     *   start_over:
     *     ++counter;
     *
     *     goto start_over;
     * \endcode
     *
     * A label reference cursor refers to a label statement.
     */
    CXCursor_LabelRef = 48,

    /**
     * A reference to a set of overloaded functions or function templates
     * that has not yet been resolved to a specific function or function template.
     *
     * An overloaded declaration reference cursor occurs in C++ templates where
     * a dependent name refers to a function. For example:
     *
     * \code
     * template<typename T> void swap(T&, T&);
     *
     * struct X { ... };
     * void swap(X&, X&);
     *
     * template<typename T>
     * void reverse(T* first, T* last) {
     *   while (first < last - 1) {
     *     swap(*first, *--last);
     *     ++first;
     *   }
     * }
     *
     * struct Y { };
     * void swap(Y&, Y&);
     * \endcode
     *
     * Here, the identifier "swap" is associated with an overloaded declaration
     * reference. In the template definition, "swap" refers to either of the two
     * "swap" functions declared above, so both results will be available. At
     * instantiation time, "swap" may also refer to other functions found via
     * argument-dependent lookup (e.g., the "swap" function at the end of the
     * example).
     *
     * The functions \c clang_getNumOverloadedDecls() and
     * \c clang_getOverloadedDecl() can be used to retrieve the definitions
     * referenced by this cursor.
     */
    CXCursor_OverloadedDeclRef = 49,

    /**
     * A reference to a variable that occurs in some non-expression
     * context, e.g., a C++ lambda capture list.
     */
    CXCursor_VariableRef = 50,

    CXCursor_LastRef = CXCursor_VariableRef,

    /* Error conditions */
    CXCursor_FirstInvalid = 70,
    CXCursor_InvalidFile = 70,
    CXCursor_NoDeclFound = 71,
    CXCursor_NotImplemented = 72,
    CXCursor_InvalidCode = 73,
    CXCursor_LastInvalid = CXCursor_InvalidCode,

    /* Expressions */
    CXCursor_FirstExpr = 100,

    /**
     * An expression whose specific kind is not exposed via this
     * interface.
     *
     * Unexposed expressions have the same operations as any other kind
     * of expression; one can extract their location information,
     * spelling, children, etc. However, the specific kind of the
     * expression is not reported.
     */
    CXCursor_UnexposedExpr = 100,

    /**
     * An expression that refers to some value declaration, such
     * as a function, variable, or enumerator.
     */
    CXCursor_DeclRefExpr = 101,

    /**
     * An expression that refers to a member of a struct, union,
     * class, Objective-C class, etc.
     */
    CXCursor_MemberRefExpr = 102,

    /** An expression that calls a function. */
    CXCursor_CallExpr = 103,

    /** An expression that sends a message to an Objective-C
     object or class. */
    CXCursor_ObjCMessageExpr = 104,

    /** An expression that represents a block literal. */
    CXCursor_BlockExpr = 105,

    /** An integer literal.
     */
    CXCursor_IntegerLiteral = 106,

    /** A floating point number literal.
     */
    CXCursor_FloatingLiteral = 107,

    /** An imaginary number literal.
     */
    CXCursor_ImaginaryLiteral = 108,

    /** A string literal.
     */
    CXCursor_StringLiteral = 109,

    /** A character literal.
     */
    CXCursor_CharacterLiteral = 110,

    /** A parenthesized expression, e.g. "(1)".
     *
     * This AST node is only formed if full location information is requested.
     */
    CXCursor_ParenExpr = 111,

    /** This represents the unary-expression's (except sizeof and
     * alignof).
     */
    CXCursor_UnaryOperator = 112,

    /** [C99 6.5.2.1] Array Subscripting.
     */
    CXCursor_ArraySubscriptExpr = 113,

    /** A builtin binary operation expression such as "x + y" or
     * "x <= y".
     */
    CXCursor_BinaryOperator = 114,

    /** Compound assignment such as "+=".
     */
    CXCursor_CompoundAssignOperator = 115,

    /** The ?: ternary operator.
     */
    CXCursor_ConditionalOperator = 116,

    /** An explicit cast in C (C99 6.5.4) or a C-style cast in C++
     * (C++ [expr.cast]), which uses the syntax (Type)expr.
     *
     * For example: (int)f.
     */
    CXCursor_CStyleCastExpr = 117,

    /** [C99 6.5.2.5]
     */
    CXCursor_CompoundLiteralExpr = 118,

    /** Describes an C or C++ initializer list.
     */
    CXCursor_InitListExpr = 119,

    /** The GNU address of label extension, representing &&label.
     */
    CXCursor_AddrLabelExpr = 120,

    /** This is the GNU Statement Expression extension: ({int X=4; X;})
     */
    CXCursor_StmtExpr = 121,

    /** Represents a C11 generic selection.
     */
    CXCursor_GenericSelectionExpr = 122,

    /** Implements the GNU __null extension, which is a name for a null
     * pointer constant that has integral type (e.g., int or long) and is the same
     * size and alignment as a pointer.
     *
     * The __null extension is typically only used by system headers, which define
     * NULL as __null in C++ rather than using 0 (which is an integer that may not
     * match the size of a pointer).
     */
    CXCursor_GNUNullExpr = 123,

    /** C++'s static_cast<> expression.
     */
    CXCursor_CXXStaticCastExpr = 124,

    /** C++'s dynamic_cast<> expression.
     */
    CXCursor_CXXDynamicCastExpr = 125,

    /** C++'s reinterpret_cast<> expression.
     */
    CXCursor_CXXReinterpretCastExpr = 126,

    /** C++'s const_cast<> expression.
     */
    CXCursor_CXXConstCastExpr = 127,

    /** Represents an explicit C++ type conversion that uses "functional"
     * notion (C++ [expr.type.conv]).
     *
     * Example:
     * \code
     *   x = int(0.5);
     * \endcode
     */
    CXCursor_CXXFunctionalCastExpr = 128,

    /** A C++ typeid expression (C++ [expr.typeid]).
     */
    CXCursor_CXXTypeidExpr = 129,

    /** [C++ 2.13.5] C++ Boolean Literal.
     */
    CXCursor_CXXBoolLiteralExpr = 130,

    /** [C++0x 2.14.7] C++ Pointer Literal.
     */
    CXCursor_CXXNullPtrLiteralExpr = 131,

    /** Represents the "this" expression in C++
     */
    CXCursor_CXXThisExpr = 132,

    /** [C++ 15] C++ Throw Expression.
     *
     * This handles 'throw' and 'throw' assignment-expression. When
     * assignment-expression isn't present, Op will be null.
     */
    CXCursor_CXXThrowExpr = 133,

    /** A new expression for memory allocation and constructor calls, e.g:
     * "new CXXNewExpr(foo)".
     */
    CXCursor_CXXNewExpr = 134,

    /** A delete expression for memory deallocation and destructor calls,
     * e.g. "delete[] pArray".
     */
    CXCursor_CXXDeleteExpr = 135,

    /** A unary expression. (noexcept, sizeof, or other traits)
     */
    CXCursor_UnaryExpr = 136,

    /** An Objective-C string literal i.e. @"foo".
     */
    CXCursor_ObjCStringLiteral = 137,

    /** An Objective-C \@encode expression.
     */
    CXCursor_ObjCEncodeExpr = 138,

    /** An Objective-C \@selector expression.
     */
    CXCursor_ObjCSelectorExpr = 139,

    /** An Objective-C \@protocol expression.
     */
    CXCursor_ObjCProtocolExpr = 140,

    /** An Objective-C "bridged" cast expression, which casts between
     * Objective-C pointers and C pointers, transferring ownership in the process.
     *
     * \code
     *   NSString *str = (__bridge_transfer NSString *)CFCreateString();
     * \endcode
     */
    CXCursor_ObjCBridgedCastExpr = 141,

    /** Represents a C++0x pack expansion that produces a sequence of
     * expressions.
     *
     * A pack expansion expression contains a pattern (which itself is an
     * expression) followed by an ellipsis. For example:
     *
     * \code
     * template<typename F, typename ...Types>
     * void forward(F f, Types &&...args) {
     *  f(static_cast<Types&&>(args)...);
     * }
     * \endcode
     */
    CXCursor_PackExpansionExpr = 142,

    /** Represents an expression that computes the length of a parameter
     * pack.
     *
     * \code
     * template<typename ...Types>
     * struct count {
     *   static const unsigned value = sizeof...(Types);
     * };
     * \endcode
     */
    CXCursor_SizeOfPackExpr = 143,

    /* Represents a C++ lambda expression that produces a local function
     * object.
     *
     * \code
     * void abssort(float *x, unsigned N) {
     *   std::sort(x, x + N,
     *             [](float a, float b) {
     *               return std::abs(a) < std::abs(b);
     *             });
     * }
     * \endcode
     */
    CXCursor_LambdaExpr = 144,

    /** Objective-c Boolean Literal.
     */
    CXCursor_ObjCBoolLiteralExpr = 145,

    /** Represents the "self" expression in an Objective-C method.
     */
    CXCursor_ObjCSelfExpr = 146,

    /** OpenMP 5.0 [2.1.5, Array Section].
     */
    CXCursor_OMPArraySectionExpr = 147,

    /** Represents an @available(...) check.
     */
    CXCursor_ObjCAvailabilityCheckExpr = 148,

    /**
     * Fixed point literal
     */
    CXCursor_FixedPointLiteral = 149,

    /** OpenMP 5.0 [2.1.4, Array Shaping].
     */
    CXCursor_OMPArrayShapingExpr = 150,

    /**
     * OpenMP 5.0 [2.1.6 Iterators]
     */
    CXCursor_OMPIteratorExpr = 151,

    /** OpenCL's addrspace_cast<> expression.
     */
    CXCursor_CXXAddrspaceCastExpr = 152,

    CXCursor_LastExpr = CXCursor_CXXAddrspaceCastExpr,

    /* Statements */
    CXCursor_FirstStmt = 200,
    /**
     * A statement whose specific kind is not exposed via this
     * interface.
     *
     * Unexposed statements have the same operations as any other kind of
     * statement; one can extract their location information, spelling,
     * children, etc. However, the specific kind of the statement is not
     * reported.
     */
    CXCursor_UnexposedStmt = 200,

    /** A labelled statement in a function.
     *
     * This cursor kind is used to describe the "start_over:" label statement in
     * the following example:
     *
     * \code
     *   start_over:
     *     ++counter;
     * \endcode
     *
     */
    CXCursor_LabelStmt = 201,

    /** A group of statements like { stmt stmt }.
     *
     * This cursor kind is used to describe compound statements, e.g. function
     * bodies.
     */
    CXCursor_CompoundStmt = 202,

    /** A case statement.
     */
    CXCursor_CaseStmt = 203,

    /** A default statement.
     */
    CXCursor_DefaultStmt = 204,

    /** An if statement
     */
    CXCursor_IfStmt = 205,

    /** A switch statement.
     */
    CXCursor_SwitchStmt = 206,

    /** A while statement.
     */
    CXCursor_WhileStmt = 207,

    /** A do statement.
     */
    CXCursor_DoStmt = 208,

    /** A for statement.
     */
    CXCursor_ForStmt = 209,

    /** A goto statement.
     */
    CXCursor_GotoStmt = 210,

    /** An indirect goto statement.
     */
    CXCursor_IndirectGotoStmt = 211,

    /** A continue statement.
     */
    CXCursor_ContinueStmt = 212,

    /** A break statement.
     */
    CXCursor_BreakStmt = 213,

    /** A return statement.
     */
    CXCursor_ReturnStmt = 214,

    /** A GCC inline assembly statement extension.
     */
    CXCursor_GCCAsmStmt = 215,
    CXCursor_AsmStmt = CXCursor_GCCAsmStmt,

    /** Objective-C's overall \@try-\@catch-\@finally statement.
     */
    CXCursor_ObjCAtTryStmt = 216,

    /** Objective-C's \@catch statement.
     */
    CXCursor_ObjCAtCatchStmt = 217,

    /** Objective-C's \@finally statement.
     */
    CXCursor_ObjCAtFinallyStmt = 218,

    /** Objective-C's \@throw statement.
     */
    CXCursor_ObjCAtThrowStmt = 219,

    /** Objective-C's \@synchronized statement.
     */
    CXCursor_ObjCAtSynchronizedStmt = 220,

    /** Objective-C's autorelease pool statement.
     */
    CXCursor_ObjCAutoreleasePoolStmt = 221,

    /** Objective-C's collection statement.
     */
    CXCursor_ObjCForCollectionStmt = 222,

    /** C++'s catch statement.
     */
    CXCursor_CXXCatchStmt = 223,

    /** C++'s try statement.
     */
    CXCursor_CXXTryStmt = 224,

    /** C++'s for (* : *) statement.
     */
    CXCursor_CXXForRangeStmt = 225,

    /** Windows Structured Exception Handling's try statement.
     */
    CXCursor_SEHTryStmt = 226,

    /** Windows Structured Exception Handling's except statement.
     */
    CXCursor_SEHExceptStmt = 227,

    /** Windows Structured Exception Handling's finally statement.
     */
    CXCursor_SEHFinallyStmt = 228,

    /** A MS inline assembly statement extension.
     */
    CXCursor_MSAsmStmt = 229,

    /** The null statement ";": C99 6.8.3p3.
     *
     * This cursor kind is used to describe the null statement.
     */
    CXCursor_NullStmt = 230,

    /** Adaptor class for mixing declarations with statements and
     * expressions.
     */
    CXCursor_DeclStmt = 231,

    /** OpenMP parallel directive.
     */
    CXCursor_OMPParallelDirective = 232,

    /** OpenMP SIMD directive.
     */
    CXCursor_OMPSimdDirective = 233,

    /** OpenMP for directive.
     */
    CXCursor_OMPForDirective = 234,

    /** OpenMP sections directive.
     */
    CXCursor_OMPSectionsDirective = 235,

    /** OpenMP section directive.
     */
    CXCursor_OMPSectionDirective = 236,

    /** OpenMP single directive.
     */
    CXCursor_OMPSingleDirective = 237,

    /** OpenMP parallel for directive.
     */
    CXCursor_OMPParallelForDirective = 238,

    /** OpenMP parallel sections directive.
     */
    CXCursor_OMPParallelSectionsDirective = 239,

    /** OpenMP task directive.
     */
    CXCursor_OMPTaskDirective = 240,

    /** OpenMP master directive.
     */
    CXCursor_OMPMasterDirective = 241,

    /** OpenMP critical directive.
     */
    CXCursor_OMPCriticalDirective = 242,

    /** OpenMP taskyield directive.
     */
    CXCursor_OMPTaskyieldDirective = 243,

    /** OpenMP barrier directive.
     */
    CXCursor_OMPBarrierDirective = 244,

    /** OpenMP taskwait directive.
     */
    CXCursor_OMPTaskwaitDirective = 245,

    /** OpenMP flush directive.
     */
    CXCursor_OMPFlushDirective = 246,

    /** Windows Structured Exception Handling's leave statement.
     */
    CXCursor_SEHLeaveStmt = 247,

    /** OpenMP ordered directive.
     */
    CXCursor_OMPOrderedDirective = 248,

    /** OpenMP atomic directive.
     */
    CXCursor_OMPAtomicDirective = 249,

    /** OpenMP for SIMD directive.
     */
    CXCursor_OMPForSimdDirective = 250,

    /** OpenMP parallel for SIMD directive.
     */
    CXCursor_OMPParallelForSimdDirective = 251,

    /** OpenMP target directive.
     */
    CXCursor_OMPTargetDirective = 252,

    /** OpenMP teams directive.
     */
    CXCursor_OMPTeamsDirective = 253,

    /** OpenMP taskgroup directive.
     */
    CXCursor_OMPTaskgroupDirective = 254,

    /** OpenMP cancellation point directive.
     */
    CXCursor_OMPCancellationPointDirective = 255,

    /** OpenMP cancel directive.
     */
    CXCursor_OMPCancelDirective = 256,

    /** OpenMP target data directive.
     */
    CXCursor_OMPTargetDataDirective = 257,

    /** OpenMP taskloop directive.
     */
    CXCursor_OMPTaskLoopDirective = 258,

    /** OpenMP taskloop simd directive.
     */
    CXCursor_OMPTaskLoopSimdDirective = 259,

    /** OpenMP distribute directive.
     */
    CXCursor_OMPDistributeDirective = 260,

    /** OpenMP target enter data directive.
     */
    CXCursor_OMPTargetEnterDataDirective = 261,

    /** OpenMP target exit data directive.
     */
    CXCursor_OMPTargetExitDataDirective = 262,

    /** OpenMP target parallel directive.
     */
    CXCursor_OMPTargetParallelDirective = 263,

    /** OpenMP target parallel for directive.
     */
    CXCursor_OMPTargetParallelForDirective = 264,

    /** OpenMP target update directive.
     */
    CXCursor_OMPTargetUpdateDirective = 265,

    /** OpenMP distribute parallel for directive.
     */
    CXCursor_OMPDistributeParallelForDirective = 266,

    /** OpenMP distribute parallel for simd directive.
     */
    CXCursor_OMPDistributeParallelForSimdDirective = 267,

    /** OpenMP distribute simd directive.
     */
    CXCursor_OMPDistributeSimdDirective = 268,

    /** OpenMP target parallel for simd directive.
     */
    CXCursor_OMPTargetParallelForSimdDirective = 269,

    /** OpenMP target simd directive.
     */
    CXCursor_OMPTargetSimdDirective = 270,

    /** OpenMP teams distribute directive.
     */
    CXCursor_OMPTeamsDistributeDirective = 271,

    /** OpenMP teams distribute simd directive.
     */
    CXCursor_OMPTeamsDistributeSimdDirective = 272,

    /** OpenMP teams distribute parallel for simd directive.
     */
    CXCursor_OMPTeamsDistributeParallelForSimdDirective = 273,

    /** OpenMP teams distribute parallel for directive.
     */
    CXCursor_OMPTeamsDistributeParallelForDirective = 274,

    /** OpenMP target teams directive.
     */
    CXCursor_OMPTargetTeamsDirective = 275,

    /** OpenMP target teams distribute directive.
     */
    CXCursor_OMPTargetTeamsDistributeDirective = 276,

    /** OpenMP target teams distribute parallel for directive.
     */
    CXCursor_OMPTargetTeamsDistributeParallelForDirective = 277,

    /** OpenMP target teams distribute parallel for simd directive.
     */
    CXCursor_OMPTargetTeamsDistributeParallelForSimdDirective = 278,

    /** OpenMP target teams distribute simd directive.
     */
    CXCursor_OMPTargetTeamsDistributeSimdDirective = 279,

    /** C++2a std::bit_cast expression.
     */
    CXCursor_BuiltinBitCastExpr = 280,

    /** OpenMP master taskloop directive.
     */
    CXCursor_OMPMasterTaskLoopDirective = 281,

    /** OpenMP parallel master taskloop directive.
     */
    CXCursor_OMPParallelMasterTaskLoopDirective = 282,

    /** OpenMP master taskloop simd directive.
     */
    CXCursor_OMPMasterTaskLoopSimdDirective = 283,

    /** OpenMP parallel master taskloop simd directive.
     */
    CXCursor_OMPParallelMasterTaskLoopSimdDirective = 284,

    /** OpenMP parallel master directive.
     */
    CXCursor_OMPParallelMasterDirective = 285,

    /** OpenMP depobj directive.
     */
    CXCursor_OMPDepobjDirective = 286,

    /** OpenMP scan directive.
     */
    CXCursor_OMPScanDirective = 287,

    /** OpenMP tile directive.
     */
    CXCursor_OMPTileDirective = 288,

    /** OpenMP canonical loop.
     */
    CXCursor_OMPCanonicalLoop = 289,

    /** OpenMP interop directive.
     */
    CXCursor_OMPInteropDirective = 290,

    /** OpenMP dispatch directive.
     */
    CXCursor_OMPDispatchDirective = 291,

    /** OpenMP masked directive.
     */
    CXCursor_OMPMaskedDirective = 292,

    /** OpenMP unroll directive.
     */
    CXCursor_OMPUnrollDirective = 293,

    CXCursor_LastStmt = CXCursor_OMPUnrollDirective,

    /**
     * Cursor that represents the translation unit itself.
     *
     * The translation unit cursor exists primarily to act as the root
     * cursor for traversing the contents of a translation unit.
     */
    CXCursor_TranslationUnit = 300,

    /* Attributes */
    CXCursor_FirstAttr = 400,
    /**
     * An attribute whose specific kind is not exposed via this
     * interface.
     */
    CXCursor_UnexposedAttr = 400,

    CXCursor_IBActionAttr = 401,
    CXCursor_IBOutletAttr = 402,
    CXCursor_IBOutletCollectionAttr = 403,
    CXCursor_CXXFinalAttr = 404,
    CXCursor_CXXOverrideAttr = 405,
    CXCursor_AnnotateAttr = 406,
    CXCursor_AsmLabelAttr = 407,
    CXCursor_PackedAttr = 408,
    CXCursor_PureAttr = 409,
    CXCursor_ConstAttr = 410,
    CXCursor_NoDuplicateAttr = 411,
    CXCursor_CUDAConstantAttr = 412,
    CXCursor_CUDADeviceAttr = 413,
    CXCursor_CUDAGlobalAttr = 414,
    CXCursor_CUDAHostAttr = 415,
    CXCursor_CUDASharedAttr = 416,
    CXCursor_VisibilityAttr = 417,
    CXCursor_DLLExport = 418,
    CXCursor_DLLImport = 419,
    CXCursor_NSReturnsRetained = 420,
    CXCursor_NSReturnsNotRetained = 421,
    CXCursor_NSReturnsAutoreleased = 422,
    CXCursor_NSConsumesSelf = 423,
    CXCursor_NSConsumed = 424,
    CXCursor_ObjCException = 425,
    CXCursor_ObjCNSObject = 426,
    CXCursor_ObjCIndependentClass = 427,
    CXCursor_ObjCPreciseLifetime = 428,
    CXCursor_ObjCReturnsInnerPointer = 429,
    CXCursor_ObjCRequiresSuper = 430,
    CXCursor_ObjCRootClass = 431,
    CXCursor_ObjCSubclassingRestricted = 432,
    CXCursor_ObjCExplicitProtocolImpl = 433,
    CXCursor_ObjCDesignatedInitializer = 434,
    CXCursor_ObjCRuntimeVisible = 435,
    CXCursor_ObjCBoxable = 436,
    CXCursor_FlagEnum = 437,
    CXCursor_ConvergentAttr = 438,
    CXCursor_WarnUnusedAttr = 439,
    CXCursor_WarnUnusedResultAttr = 440,
    CXCursor_AlignedAttr = 441,
    CXCursor_LastAttr = CXCursor_AlignedAttr,

    /* Preprocessing */
    CXCursor_PreprocessingDirective = 500,
    CXCursor_MacroDefinition = 501,
    CXCursor_MacroExpansion = 502,
    CXCursor_MacroInstantiation = CXCursor_MacroExpansion,
    CXCursor_InclusionDirective = 503,
    CXCursor_FirstPreprocessing = CXCursor_PreprocessingDirective,
    CXCursor_LastPreprocessing = CXCursor_InclusionDirective,

    /* Extra Declarations */
    /**
     * A module import declaration.
     */
    CXCursor_ModuleImportDecl = 600,
    CXCursor_TypeAliasTemplateDecl = 601,
    /**
     * A static_assert or _Static_assert node
     */
    CXCursor_StaticAssert = 602,
    /**
     * a friend declaration.
     */
    CXCursor_FriendDecl = 603,
    CXCursor_FirstExtraDecl = CXCursor_ModuleImportDecl,
    CXCursor_LastExtraDecl = CXCursor_FriendDecl,

    /**
     * A code completion overload candidate.
     */
    CXCursor_OverloadCandidate = 700
};

/**
 * A cursor representing some element in the abstract syntax tree for
 * a translation unit.
 *
 * The cursor abstraction unifies the different kinds of entities in a
 * program--declaration, statements, expressions, references to declarations,
 * etc.--under a single "cursor" abstraction with a common set of operations.
 * Common operation for a cursor include: getting the physical location in
 * a source file where the cursor points, getting the name associated with a
 * cursor, and retrieving cursors for any child nodes of a particular cursor.
 *
 * Cursors can be produced in two specific ways.
 * clang_getTranslationUnitCursor() produces a cursor for a translation unit,
 * from which one can use clang_visitChildren() to explore the rest of the
 * translation unit. clang_getCursor() maps from a physical source location
 * to the entity that resides at that location, allowing one to map from the
 * source code into the AST.
 */
typedef struct {
    enum CXCursorKind kind;
    int xdata;
    const void *data[3];
} CXCursor;

/**
 * Identifies a specific source location within a translation
 * unit.
 *
 * Use clang_getExpansionLocation() or clang_getSpellingLocation()
 * to map a source location to a particular file, line, and column.
 */
typedef struct {
    const void *ptr_data[2];
    unsigned int_data;
} CXSourceLocation;

/**
 * A character string.
 *
 * The \c CXString type is used to return strings from the interface when
 * the ownership of that string might differ from one call to the next.
 * Use \c clang_getCString() to retrieve the string data and, once finished
 * with the string data, call \c clang_disposeString() to free the string.
 */
typedef struct {
    const void *data;
    unsigned private_flags;
} CXString;

/**
 * The type of an element in the abstract syntax tree.
 *
 */
typedef struct {
    enum CXTypeKind kind;
    void *data[2];
} CXType;

/**
 * Provides a shared context for creating translation units.
 *
 * It provides two options:
 *
 * - excludeDeclarationsFromPCH: When non-zero, allows enumeration of "local"
 * declarations (when loading any new translation units). A "local" declaration
 * is one that belongs in the translation unit itself and not in a precompiled
 * header that was used by the translation unit. If zero, all declarations
 * will be enumerated.
 *
 * Here is an example:
 *
 * \code
 *   // excludeDeclsFromPCH = 1, displayDiagnostics=1
 *   Idx = clang_createIndex(1, 1);
 *
 *   // IndexTest.pch was produced with the following command:
 *   // "clang -x c IndexTest.h -emit-ast -o IndexTest.pch"
 *   TU = clang_createTranslationUnit(Idx, "IndexTest.pch");
 *
 *   // This will load all the symbols from 'IndexTest.pch'
 *   clang_visitChildren(clang_getTranslationUnitCursor(TU),
 *                       TranslationUnitVisitor, 0);
 *   clang_disposeTranslationUnit(TU);
 *
 *   // This will load all the symbols from 'IndexTest.c', excluding symbols
 *   // from 'IndexTest.pch'.
 *   char *args[] = { "-Xclang", "-include-pch=IndexTest.pch" };
 *   TU = clang_createTranslationUnitFromSourceFile(Idx, "IndexTest.c", 2, args,
 *                                                  0, 0);
 *   clang_visitChildren(clang_getTranslationUnitCursor(TU),
 *                       TranslationUnitVisitor, 0);
 *   clang_disposeTranslationUnit(TU);
 * \endcode
 *
 * This process of creating the 'pch', loading it separately, and using it (via
 * -include-pch) allows 'excludeDeclsFromPCH' to remove redundant callbacks
 * (which gives the indexer the same performance benefit as the compiler).
 */
// CINDEX_LINKAGE CXIndex clang_createIndex(int excludeDeclarationsFromPCH,
//                                          int displayDiagnostics);
typedef CXIndex (*func_ptr_clang_createIndex)(int excludeDeclarationsFromPCH, int displayDiagnostics);
static const char *$clang_createIndex$ = "clang_createIndex";

/**
 * Destroy the given index.
 *
 * The index must not be destroyed until all of the translation units created
 * within that index have been destroyed.
 */
// CINDEX_LINKAGE void clang_disposeIndex(CXIndex index);
typedef void (*func_ptr_clang_disposeIndex)(CXIndex index);
static const char *$clang_disposeIndex$ = "clang_disposeIndex";

/**
 * Same as \c clang_parseTranslationUnit2, but returns
 * the \c CXTranslationUnit instead of an error code.  In case of an error this
 * routine returns a \c NULL \c CXTranslationUnit, without further detailed
 * error codes.
 */
// CINDEX_LINKAGE CXTranslationUnit clang_parseTranslationUnit(
//     CXIndex CIdx, const char *source_filename,
//     const char *const *command_line_args, int num_command_line_args,
//     struct CXUnsavedFile *unsaved_files, unsigned num_unsaved_files,
//     unsigned options);
typedef CXTranslationUnit (*func_ptr_clang_parseTranslationUnit)(
    CXIndex CIdx, const char *source_filename,
    const char *const *command_line_args, int num_command_line_args,
    struct CXUnsavedFile *unsaved_files, unsigned num_unsaved_files,
    unsigned options);
static const char *$clang_parseTranslationUnit$ = "clang_parseTranslationUnit";

/**
 * Destroy the specified CXTranslationUnit object.
 */
// CINDEX_LINKAGE void clang_disposeTranslationUnit(CXTranslationUnit);
typedef void (*func_ptr_clang_disposeTranslationUnit)(CXTranslationUnit);
static const char *$clang_disposeTranslationUnit$ = "clang_disposeTranslationUnit";

/**
 * Visitor invoked for each cursor found by a traversal.
 *
 * This visitor function will be invoked for each cursor found by
 * clang_visitCursorChildren(). Its first argument is the cursor being
 * visited, its second argument is the parent visitor for that cursor,
 * and its third argument is the client data provided to
 * clang_visitCursorChildren().
 *
 * The visitor should return one of the \c CXChildVisitResult values
 * to direct clang_visitCursorChildren().
 */
typedef enum CXChildVisitResult (*CXCursorVisitor)(CXCursor cursor,
                                                   CXCursor parent,
                                                   CXClientData client_data);

/**
 * Visit the children of a particular cursor.
 *
 * This function visits all the direct children of the given cursor,
 * invoking the given \p visitor function with the cursors of each
 * visited child. The traversal may be recursive, if the visitor returns
 * \c CXChildVisit_Recurse. The traversal may also be ended prematurely, if
 * the visitor returns \c CXChildVisit_Break.
 *
 * \param parent the cursor whose child may be visited. All kinds of
 * cursors can be visited, including invalid cursors (which, by
 * definition, have no children).
 *
 * \param visitor the visitor function that will be invoked for each
 * child of \p parent.
 *
 * \param client_data pointer data supplied by the client, which will
 * be passed to the visitor each time it is invoked.
 *
 * \returns a non-zero value if the traversal was terminated
 * prematurely by the visitor returning \c CXChildVisit_Break.
 */
// CINDEX_LINKAGE unsigned clang_visitChildren(CXCursor parent,
//                                             CXCursorVisitor visitor,
//                                             CXClientData client_data);
typedef unsigned (*func_ptr_clang_visitChildren)(CXCursor parent,
                                        CXCursorVisitor visitor,
                                        CXClientData client_data);
static const char *$clang_visitChildren$ = "clang_visitChildren";

/**
 * Retrieve the cursor that represents the given translation unit.
 *
 * The translation unit cursor can be used to start traversing the
 * various declarations within the given translation unit.
 */
// CINDEX_LINKAGE CXCursor clang_getTranslationUnitCursor(CXTranslationUnit);
typedef CXCursor (*func_ptr_clang_getTranslationUnitCursor)(CXTranslationUnit);
static const char *$clang_getTranslationUnitCursor$ = "clang_getTranslationUnitCursor";

/**
 * Retrieve the physical location of the source constructor referenced
 * by the given cursor.
 *
 * The location of a declaration is typically the location of the name of that
 * declaration, where the name of that declaration would occur if it is
 * unnamed, or some keyword that introduces that particular declaration.
 * The location of a reference is where that reference occurs within the
 * source code.
 */
// CINDEX_LINKAGE CXSourceLocation clang_getCursorLocation(CXCursor);
typedef CXSourceLocation (*func_ptr_clang_getCursorLocation)(CXCursor);
static const char *$clang_getCursorLocation$ = "clang_getCursorLocation";

/**
 * Retrieve the file, line, column, and offset represented by
 * the given source location.
 *
 * If the location refers into a macro expansion, return where the macro was
 * expanded or where the macro argument was written, if the location points at
 * a macro argument.
 *
 * \param location the location within a source file that will be decomposed
 * into its parts.
 *
 * \param file [out] if non-NULL, will be set to the file to which the given
 * source location points.
 *
 * \param line [out] if non-NULL, will be set to the line to which the given
 * source location points.
 *
 * \param column [out] if non-NULL, will be set to the column to which the given
 * source location points.
 *
 * \param offset [out] if non-NULL, will be set to the offset into the
 * buffer to which the given source location points.
 */
// CINDEX_LINKAGE void clang_getFileLocation(CXSourceLocation location,
//                                           CXFile *file, unsigned *line,
//                                           unsigned *column, unsigned *offset);
typedef void (*func_ptr_clang_getFileLocation)(CXSourceLocation location,
                                      CXFile *file, unsigned *line,
                                      unsigned *column, unsigned *offset);
static const char *$clang_getFileLocation$ = "clang_getFileLocation";

/**
 * Retrieve the kind of the given cursor.
 */
// CINDEX_LINKAGE enum CXCursorKind clang_getCursorKind(CXCursor);
typedef enum CXCursorKind (*func_ptr_clang_getCursorKind)(CXCursor);
static const char *$clang_getCursorKind$ = "clang_getCursorKind";

/**
 * Retrieve the file that is included by the given inclusion directive
 * cursor.
 */
// CINDEX_LINKAGE CXFile clang_getIncludedFile(CXCursor cursor);
typedef CXFile (*func_ptr_clang_getIncludedFile)(CXCursor cursor);
static const char *$clang_getIncludedFile$ = "clang_getIncludedFile";

/**
 * Retrieve the complete file and path name of the given file.
 */
// CINDEX_LINKAGE CXString clang_getFileName(CXFile SFile);
typedef CXString (*func_ptr_clang_getFileName)(CXFile SFile);
static const char *$clang_getFileName$ = "clang_getFileName";

/**
 * Retrieve the character data associated with the given string.
 */
// CINDEX_LINKAGE const char *clang_getCString(CXString string);
typedef const char *(*func_ptr_clang_getCString)(CXString string);
static const char *$clang_getCString$ = "clang_getCString";

/**
 * Free the given string.
 */
// CINDEX_LINKAGE void clang_disposeString(CXString string);
typedef void (*func_ptr_clang_disposeString)(CXString string);
static const char *$clang_disposeString$ = "clang_disposeString";

/**
 * Retrieve a name for the entity referenced by this cursor.
 */
// CINDEX_LINKAGE CXString clang_getCursorSpelling(CXCursor);
typedef CXString (*func_ptr_clang_getCursorSpelling)(CXCursor);
static const char *$clang_getCursorSpelling$ = "clang_getCursorSpelling";

/**
 * Retrieve the number of non-variadic arguments associated with a given
 * cursor.
 *
 * The number of arguments can be determined for calls as well as for
 * declarations of functions or methods. For other cursors -1 is returned.
 */
// CINDEX_LINKAGE int clang_Cursor_getNumArguments(CXCursor C);
typedef int (*func_ptr_clang_Cursor_getNumArguments)(CXCursor C);
static const char *$clang_Cursor_getNumArguments$ = "clang_Cursor_getNumArguments";

/**
 * Retrieve the argument cursor of a function or method.
 *
 * The argument cursor can be determined for calls as well as for declarations
 * of functions or methods. For other cursors and for invalid indices, an
 * invalid cursor is returned.
 */
// CINDEX_LINKAGE CXCursor clang_Cursor_getArgument(CXCursor C, unsigned i);
typedef CXCursor (*func_ptr_clang_Cursor_getArgument)(CXCursor C, unsigned i);
static const char *$clang_Cursor_getArgument$ = "clang_Cursor_getArgument";

/**
 * Retrieve the type of a CXCursor (if any).
 */
// CINDEX_LINKAGE CXType clang_getCursorType(CXCursor C);
typedef CXType (*func_ptr_clang_getCursorType)(CXCursor C);
static const char *$clang_getCursorType$ = "clang_getCursorType";

/**
 * Return the canonical type for a CXType.
 *
 * Clang's type system explicitly models typedefs and all the ways
 * a specific type can be represented.  The canonical type is the underlying
 * type with all the "sugar" removed.  For example, if 'T' is a typedef
 * for 'int', the canonical type for 'T' would be 'int'.
 */
// CINDEX_LINKAGE CXType clang_getCanonicalType(CXType T);
typedef CXType (*func_ptr_clang_getCanonicalType)(CXType T);
static const char *$clang_getCanonicalType$ = "clang_getCanonicalType";

/**
 * Retrieve the return type associated with a function type.
 *
 * If a non-function type is passed in, an invalid type is returned.
 */
// CINDEX_LINKAGE CXType clang_getResultType(CXType T);
typedef CXType (*func_ptr_clang_getResultType)(CXType T);
static const char *$clang_getResultType$ = "clang_getResultType";

/**
 * Pretty-print the underlying type using the rules of the
 * language of the translation unit from which it came.
 *
 * If the type is invalid, an empty string is returned.
 */
// CINDEX_LINKAGE CXString clang_getTypeSpelling(CXType CT);
typedef CXString (*func_ptr_clang_getTypeSpelling)(CXType CT);
static const char *$clang_getTypeSpelling$ = "clang_getTypeSpelling";

/**
 * Determine the semantic parent of the given cursor.
 *
 * The semantic parent of a cursor is the cursor that semantically contains
 * the given \p cursor. For many declarations, the lexical and semantic parents
 * are equivalent (the lexical parent is returned by
 * \c clang_getCursorLexicalParent()). They diverge when declarations or
 * definitions are provided out-of-line. For example:
 *
 * \code
 * class C {
 *  void f();
 * };
 *
 * void C::f() { }
 * \endcode
 *
 * In the out-of-line definition of \c C::f, the semantic parent is
 * the class \c C, of which this function is a member. The lexical parent is
 * the place where the declaration actually occurs in the source code; in this
 * case, the definition occurs in the translation unit. In general, the
 * lexical parent for a given entity can change without affecting the semantics
 * of the program, and the lexical parent of different declarations of the
 * same entity may be different. Changing the semantic parent of a declaration,
 * on the other hand, can have a major impact on semantics, and redeclarations
 * of a particular entity should all have the same semantic context.
 *
 * In the example above, both declarations of \c C::f have \c C as their
 * semantic context, while the lexical context of the first \c C::f is \c C
 * and the lexical context of the second \c C::f is the translation unit.
 *
 * For global declarations, the semantic parent is the translation unit.
 */
// CINDEX_LINKAGE CXCursor clang_getCursorSemanticParent(CXCursor cursor);
typedef CXCursor (*func_ptr_clang_getCursorSemanticParent)(CXCursor cursor);
static const char *$clang_getCursorSemanticParent$ = "clang_getCursorSemanticParent";

/**
 * Returns non-zero if \p cursor is null.
 */
// CINDEX_LINKAGE int clang_Cursor_isNull(CXCursor cursor);
typedef int (*func_ptr_clang_Cursor_isNull)(CXCursor cursor);
static const char *$clang_Cursor_isNull$ = "clang_Cursor_isNull";

#endif // C_SOURCE_PARSER_FFI_CLANG_API