import("./pkg")
  .then((module) => {
    module.run_app();
  })
  .catch(console.error);
