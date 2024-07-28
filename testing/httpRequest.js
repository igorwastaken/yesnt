const githubApi = httpGet("https://api.github.com");
printout(JSON.parse(githubApi));
printout(httpPost("https://jsonplaceholder.typicode.com/posts", JSON.stringify({ title: "foo", body: "bar", userId: 1 })));