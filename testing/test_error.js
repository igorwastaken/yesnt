(function() {
    try {
        let result = nonExistentFunction();
        printout(`Result: ${result}`);
    } catch (error) {
        printout(`Caught an error: ${error.message}`);
    }
})();
