// 1) Basic call signature in an interface
interface MyCallable {
    (x: number): string;
}

// 2) Type literal with a call signature
type MyFormatter = {
    (value: string): string;
};

// 3) Generic call signature
interface GenericCallable {
    <T>(arg: T): T;
}

// 4) Overloaded call signatures
interface OverloadedCallable {
    (value: string): string;
    (value: number): number;
}

// 5) Optional parameters, rest parameters, return type
interface AdvancedCallable {
    <T>(arg: T, optionalArg?: number, ...rest: string[]): T[];
}

// 6) Putting it all together in a single interface
interface MixedCallable {
    // Overload #1
    (value: number): number[];
    // Overload #2
    (value: string): string[];
    // Generic form
    <T>(value: T): T[];
}
