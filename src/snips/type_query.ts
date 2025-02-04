// 1) Basic 'typeof' in a type position
const someVar = { x: 10 };
type T1 = typeof someVar; // TypeQuery -> exprName = Identifier('someVar')

// 2) Class and static property
class MyClass {
    static prop = 123;
}
type T2 = typeof MyClass; // TypeQuery -> exprName = Identifier('MyClass')
type T3 = typeof MyClass.prop; // TypeQuery -> exprName = QualifiedName(MyClass, prop)

// 3) Namespace usage
namespace MyNS {
    export const data = 42;
}
type T4 = typeof MyNS.data; // TypeQuery -> exprName = QualifiedName(MyNS, data)

// 4) (Commented Example) Import type with 'typeof' (parsed as ImportTypeNode, isTypeOf = true)
// @ts-ignore
type T5 = typeof import("some-module");
