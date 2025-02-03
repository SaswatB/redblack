// Variable declaration
let myVariable = 10; // <-- 'myVariable' is an Identifier

// Function name
function doSomething() { /* ... */ } // <-- 'doSomething' is an Identifier

// Class name
class Person { // <-- 'Person' is an Identifier
  name: string;
  constructor(name: string) {
    this.name = name; // 'this' is a keyword, 'name' is an Identifier
  }
}

// Object property shorthand
const user = { myVariable }; // <-- 'myVariable' here is also an Identifier

// Other typical examples:
const _hidden = 42;
const $element = document.getElementById("some-id");