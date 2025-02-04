namespace MyNamespace {
    export class MyClass {}
}

interface MyInterface {
    method(): void;
}

// 1) Namespace + Class reference in a variable type
let variable: MyNamespace.MyClass; // QualifiedName: MyNamespace.MyClass

// 2) QualifiedName in an 'extends' clause
interface SubInterface extends MyNamespace.MyClass {} // Another usage

// 3) Nested namespaces
namespace Outer {
    export namespace Inner {
        export interface NestedInterface {}
    }
}

let nestedVar: Outer.Inner.NestedInterface; // QualifiedName: Outer.Inner.NestedInterface

// 4) Type alias referencing a nested QualifiedName
type AliasType = Outer.Inner.NestedInterface; // Also a QualifiedName
