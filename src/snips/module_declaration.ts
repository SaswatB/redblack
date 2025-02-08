// 1) Basic namespace
namespace MyNamespace {
    export function greet() {
        console.log("Hello from MyNamespace");
    }
}

// 2) Nested namespaces
namespace Outer.Inner {
    export const value = 42;
}

// 3) Ambient external module declaration (usually in .d.ts)
declare module "my-external-module" {
    export function doSomething(): void;
}

// 4) Global augmentation
declare global {
    interface String {
        customMethod(): void;
    }
}

// 5) Another form of ambient module with no body (rare, but valid)
declare module "empty-module";
