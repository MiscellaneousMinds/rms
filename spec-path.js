const exec = require("child_process");

const getSpecPaths = async () => {
  const paths = await new Promise((res, rej) =>
    exec.exec("find . -type f -name '*-spec.yaml'", (error, stdout, stderr) => {
      if (error || stderr) return rej(error || stderr);
      res(stdout);
    })
  );

  return paths;
};

module.exports = { getSpecPaths };
