function foo(cd) {
    var that = _this;
    function boo(a,b) {
        var x = 5;
        var _x = 5;
        var snake_case = 5;
        var camelCase = 5;
        while (true) {}
        function f() { 
            const x = 5+5;
            return 42;
        }
        const x = 5+5;
        let goo = [{}];
    }

    function f(a,b,c) { }
    var x = function (a,b) { }
    var x = function () { }
}

function empty() {}
empty_call();
function foo(a,b,) {}
function foo(x, a = "foo",b = "bar") {}
goo(a,b)
goo(a,b,)

function someFunction(a,b) { const x = a+b; return x in {y: 1}; } 

function someFunction(a: string, b: int,) { } 
function someFunction(a: ?string, b: ?Map) { } 
function someFunction(a: int = 42) { } 

