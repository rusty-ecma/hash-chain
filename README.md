# Hash-Chain

![hex characters in the shape of a chain](icon.svg)

A tiered hashmap and hashset implementation that allows for easily representing lexically scoped variables.


```js

const x = 0;
const y = 2;
function me() {
    let x = 1;
    console.log(x); //prints 1
    console.log(y); //prints 2
}

```