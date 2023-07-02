function tryFoo() {
    try {
        hello();
        world();
        x = 5 + 3;
        return x;
    } catch(e) {
        console.log("XXXX", e);
        return { err: 123 };
    } finally {
        setSocket(null);
        closeConnection();
    }

    try {
        return JSON.parse(x)
    } catch(e) {
        return {}
    }

    return 2 + 2;
} 