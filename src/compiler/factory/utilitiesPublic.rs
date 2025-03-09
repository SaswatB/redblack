use oxc_ast::AstKind;

// pub fn setTextRange<'a, T: TextRange<'a>>(range: T, location: Option<&impl TextRange<'a>>) -> T {
//     if let Some(location) = location {
//         range.setTextRangePosEnd(location.pos(), location.end())
//     } else {
//         range
//     }
// }

pub fn canHaveModifiers(node: &AstKind) -> bool {
    match node {
        AstKind::TSTypeParameter(_) // TypeParameter
        | AstKind::FormalParameter(_) // Parameter
        | AstKind::TSPropertySignature(_) // PropertySignature
        | AstKind::PropertyDefinition(_) // PropertyDeclaration
        | AstKind::TSMethodSignature(_) // MethodSignature
        | AstKind::MethodDefinition(_) // MethodDeclaration
        // | SyntaxKind::Constructor // No direct mapping, part of MethodDefinition with kind Constructor
        // | SyntaxKind::GetAccessor // No direct mapping, part of MethodDefinition with kind Get
        // | SyntaxKind::SetAccessor // No direct mapping, part of MethodDefinition with kind Set
        | AstKind::TSIndexSignature(_) // IndexSignature
        | AstKind::TSConstructorType(_) // ConstructorTypeNode
        | AstKind::Function(_) // FunctionExpression
        | AstKind::ArrowFunctionExpression(_) // ArrowFunction
        | AstKind::Class(_) // ClassExpression
        | AstKind::VariableDeclarationList(_) // VariableStatement
        // | SyntaxKind::FunctionDeclaration // Part of Function
        // | SyntaxKind::ClassDeclaration // Part of Class
        | AstKind::TSInterfaceDeclaration(_) // InterfaceDeclaration
        | AstKind::TSTypeAliasDeclaration(_) // TypeAliasDeclaration
        | AstKind::TSEnumDeclaration(_) // EnumDeclaration
        | AstKind::TSModuleDeclaration(_) // ModuleDeclaration
        | AstKind::TSImportEqualsDeclaration(_) // ImportEqualsDeclaration
        | AstKind::ImportDeclaration(_) // ImportDeclaration
        | AstKind::TSExportAssignment(_) // ExportAssignment (isExportEquals: true)
        | AstKind::ExportDefaultDeclaration(_) // ExportAssignment (isExportEquals: false)
        | AstKind::ExportAllDeclaration(_) // ExportDeclaration
        | AstKind::ExportNamedDeclaration(_) // ExportDeclaration
        | AstKind::AccessorProperty(_) // !rb added this, but ts doesn't have it
        => true,
        _ => false,
    }
}

pub fn canHaveDecorators(node: &AstKind) -> bool {
    match node {
        AstKind::FormalParameter(_) // Parameter
        | AstKind::PropertyDefinition(_) // PropertyDeclaration
        | AstKind::MethodDefinition(_) // MethodDeclaration, GetAccessor, SetAccessor
        | AstKind::Class(_) // ClassExpression, ClassDeclaration
        => true,
        _ => false,
    }
}
