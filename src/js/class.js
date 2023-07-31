class Foo {}
class Foo extends Boo {
    foo() { this.a = 5; } 
    boo(c,d) {}
    boo(c,d,k) {
        this.boo();
        this.foo()
    }
}

class Foo {
    constructor() {
        super(this);
    }
}