var fs = require("fs");
const { createAppAuth } = require("@octokit/auth-app");
const main = async () => {
    const argv = require("yargs").option("private-key", {
        alias: "p",
        description: "Path to the private key used to generate the jwt",
        global: true,
        demandOption: true,
    }).argv;
    var privateKey = fs.readFileSync(argv.privateKey, "utf8");
    const auth = createAppAuth({ appId: 115727, privateKey: privateKey });
    const jwt = await auth({ type: "app" });
    console.log(jwt.token);
};
main();
