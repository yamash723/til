class Student {
    fullNmae: string;

    constructor(public firstName: string, public middleInitial: string, public lastName: string) {
        this.fullNmae = firstName + ' ' + middleInitial + ' ' + lastName;
    }
}

interface Person {
    firstName: string;
    lastName: string;
}

function greeter(person: Person) {
    return 'Hello, ' + person.firstName + ' ' + person.lastName;
}

let user = new Student('Jane', 'M', 'User');

document.body.innerHTML = greeter(user);

// // webpack tutorial
// import * as _ from 'lodash';

// function component() {
//     var element = document.createElement('div');

//     // Lodash, now imported by this script.
//     element.innerHTML = _.join(['Hello', 'webpack'], ' ');

//     return element;
// }

// document.body.appendChild(component());