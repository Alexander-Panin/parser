export default function foo(a,b,c) {
    const x = xs.map((x) => x+1);
    return x;
}

export const foo = (a,b,c) => 42;

export default foo;

const x = () => 42;
export x;

export default class Foo {}

export * from "module-name";
export {a} from "b";
export {a as c} from "b";

export {a,b,c,}
export {a,b,c}
export {a,b,c} from "mega-module";