addEventListener("TrunkApplicationStarted", (event) => {
  console.log("WASM application started!");
  const canvas = document.getElementById("canvas");

  var [width, height] = [window.innerWidth, window.innerHeight];

  let context = window.wasmBindings.init(canvas, width, height);

  window.wasmBindings.draw(context);

  // window.wasmBindings.draw(window.innerWidth, window.innerHeight);

  addEventListener("resize", () => {
    [width, height] = [window.innerWidth, window.innerHeight];
    // console.log("Window resized: (", width, ", ", height, ")");
    window.wasmBindings.resize(context, width, height);
  });

  canvas.addEventListener("click", (e) => {
    // console.log(e);

    // [width, height] = [window.innerWidth, window.innerHeight];
    // console.log("Window resized: (", width, ", ", height, ")");
    window.wasmBindings.click(context, e.pageX, e.pageY);
  });
});
