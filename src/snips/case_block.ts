// 1) Basic switch with a few case clauses and a default
function basicSwitch(x: number) {
    switch (x) {
        case 0:
            console.log("x is zero");
            break;
        case 1:
        case 2:
            console.log("x is one or two");
            break;
        default:
            console.log("x is something else");
    }
}

// 2) Switch with no default clause
function noDefault(y: string) {
    switch (y) {
        case "hello":
            return "greeting";
        case "bye":
            return "farewell";
    }
}

// 3) Switch with empty cases
function emptyCases(z: number) {
    switch (z) {
        case 10:
        // Intentional empty, will fall through
        case 11:
        // Another empty, fall through to next
        case 12:
            console.log("z is 10, 11, or 12");
            break;
        default:
            console.log("z is something else");
    }
}
