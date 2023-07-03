function foo() {
    function boo() {
        var x = 5;
        while (true) {}
        function f() { 
            const x = 5+5;
            return 42;
        }
        const x = 5+5;
        let goo = [{}];
    }

    function f() { }
    function g() { }
    function moo() { var x = 42; }
}

function more() {}

function someFunction(a,b) { const x = a+b; return x in {y: 1}; } 

