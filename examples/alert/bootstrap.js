console.log("RUN11");
import("./pkg")
  .then((module) => {
    console.log("RUN");
    module.run_app();
  })
  .catch(console.error);
