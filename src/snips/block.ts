// 1) Function block
function greet(name: string) {
    console.log("Hello, " + name);
}

// 2) If/Else block
const x = 15;
if (x > 10) {
    console.log("x is large");
} else {
    console.log("x is small or medium");
}

// 3) Free-standing block
{
    let temp = 10;
    console.log("Temp is:", temp);
}

// 4) Lambda block
const numbers = [1, 2, 3, 4, 5];
numbers.forEach((num) => {
    console.log(num * 2);
});

// 5) Class block
class Person {
    name: string;
    constructor(name: string) {
        this.name = name;
    }
}
