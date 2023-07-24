// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[cfg(feature = "slang_napi_interfaces")]
use napi::bindgen_prelude::*;
#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde :: Serialize,
    strum_macros :: AsRefStr,
    strum_macros :: Display,
    strum_macros :: EnumString,
)]
#[cfg_attr(
    feature = "slang_napi_interfaces",
    napi(string_enum, namespace = "syntax$nodes")
)]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum RuleKind {
    ABICoderPragma,
    AddressType,
    ArgumentsDeclaration,
    ArrayExpression,
    ArrayTypeName,
    ArrayValuesList,
    AsciiStringLiteralsList,
    AssemblyFlagsList,
    AssemblyStatement,
    BinaryExpression,
    Block,
    BreakStatement,
    CatchClause,
    CatchClauseError,
    CatchClausesList,
    ConditionalExpression,
    ConstantDefinition,
    ConstructorAttributesList,
    ConstructorDefinition,
    ContinueStatement,
    ContractDefinition,
    ContractMembersList,
    DeconstructionImport,
    DeconstructionImportSymbol,
    DeconstructionImportSymbolsList,
    DeleteStatement,
    DoWhileStatement,
    EmitStatement,
    EndOfFileTrivia,
    EnumDefinition,
    ErrorDefinition,
    ErrorParameter,
    ErrorParametersList,
    EventDefinition,
    EventParameter,
    EventParametersList,
    ExperimentalPragma,
    Expression,
    ExpressionStatement,
    FallbackFunctionAttributesList,
    FallbackFunctionDefinition,
    ForStatement,
    FunctionAttributesList,
    FunctionCallExpression,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionType,
    FunctionTypeAttributesList,
    HexStringLiteralsList,
    IdentifierPath,
    IdentifierPathsList,
    IdentifiersList,
    IfStatement,
    ImportDirective,
    IndexAccessExpression,
    InheritanceSpecifier,
    InheritanceType,
    InheritanceTypesList,
    InterfaceDefinition,
    InterfaceMembersList,
    LeadingTrivia,
    LibraryDefinition,
    LibraryMembersList,
    MappingKeyType,
    MappingType,
    MappingValueType,
    MemberAccessExpression,
    ModifierAttributesList,
    ModifierDefinition,
    ModifierInvocation,
    NamedArgument,
    NamedArgumentsDeclaration,
    NamedArgumentsList,
    NamedImport,
    NewExpression,
    NumericExpression,
    OverrideSpecifier,
    Parameter,
    ParametersDeclaration,
    ParametersList,
    PathImport,
    PositionalArgumentsList,
    PragmaDirective,
    ReceiveFunctionAttributesList,
    ReceiveFunctionDefinition,
    ReturnStatement,
    ReturnsDeclaration,
    RevertStatement,
    SourceUnit,
    SourceUnitMembersList,
    StateVariableAttributesList,
    StateVariableDefinition,
    Statement,
    StatementsList,
    StructDefinition,
    StructMember,
    StructMembersList,
    ThrowStatement,
    TrailingTrivia,
    TryStatement,
    TupleDeconstructionStatement,
    TupleExpression,
    TupleMember,
    TupleMembersList,
    TupleValuesList,
    TypeExpression,
    TypeName,
    UnaryPostfixExpression,
    UnaryPrefixExpression,
    UncheckedBlock,
    UnicodeStringLiteralsList,
    UnnamedFunctionAttributesList,
    UnnamedFunctionDefinition,
    UserDefinedValueTypeDefinition,
    UsingDirective,
    UsingDirectiveDeconstruction,
    UsingDirectivePath,
    UsingDirectiveSymbol,
    UsingDirectiveSymbolsList,
    VariableDeclaration,
    VariableDeclarationStatement,
    VersionPragma,
    VersionPragmaBinaryExpression,
    VersionPragmaExpression,
    VersionPragmaExpressionsList,
    VersionPragmaSpecifier,
    VersionPragmaUnaryExpression,
    WhileStatement,
    YulAssignmentStatement,
    YulBlock,
    YulBreakStatement,
    YulContinueStatement,
    YulDeclarationStatement,
    YulExpression,
    YulExpressionsList,
    YulForStatement,
    YulFunctionCallExpression,
    YulFunctionDefinition,
    YulIdentifierPath,
    YulIdentifierPathsList,
    YulIdentifiersList,
    YulIfStatement,
    YulLeaveStatement,
    YulParametersDeclaration,
    YulReturnsDeclaration,
    YulStatement,
    YulStatementsList,
    YulSwitchCase,
    YulSwitchCasesList,
    YulSwitchStatement,
}
