class Foo {}
class Foo extends Boo {
    foo() { this.a = 5; } 
    boo(c,d) {}
}

class Foo {
    constructor() {
        super(this);
    }
}