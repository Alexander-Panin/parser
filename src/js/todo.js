function boo() {
    // x++; maybe not
    // this["foo"]()
    // this["foo"] = 5;
    // x?.a?.b
}

class Foo {
    constructor() {
        super(this);
    }
}