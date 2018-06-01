// Boolean
let isDone: boolean = false;

// Number
let decimal: number = 6;
let hex: number = 0xf00d;
let binary: number = 0b1010;
let octal: number = 0o744;

// String
let color: string = "blue";

let fullNme: string = `Bob Bobbington`;
let age: number = 37;
let sentence: string = `Hello, my name is ${fullNme}.
I'll be ${age + 1} years old next month.`;

// Array
let list: number[] = [1, 2, 3];
let array_list: Array<number> = [1, 2, 3];

// Tupl
let x: [string, number];
x = ["hello", 10]

console.log(x[0]); // hello
console.log(x[1]); // 10

// Enum
enum Color {Red = 1, Green, Blue}
let c: Color = Color.Green;

// Any
let notSure: any = 4;
notSure = "maybe a string instead";
notSure = false;

// void
function warnUser(): void {
    alert("This is my warning message");
}

// Undefined / null
let u: undefined = undefined;
let n: null = null;

// Never
function error(message: string): never {
    throw new Error(message);
}

// Object
declare function create(o: object | null): void;
create({ prop: 0 });
create(null);
// create(42); // Error

// Type assertions
let someValue: any = "This is a string".anchor
let strLength: number = (someValue as string).length;