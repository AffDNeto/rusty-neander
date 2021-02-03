// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
require('./neander.js');
require('./neander.css');

import "./neander";
import { MemTableControler } from "./neander";

const rust = import('../pkg');
var nn;
rust
  .then(m => { m.greet('World!'); nn = m } )
  .catch(console.error);

window.onload = function() {
  window.mem = MemTableControler;
  var memTb = new MemTableControler(document.getElementById("memContainer"), 10);
  window.mem_control = memTb
  memTb.init();
  memTb.updateTable([0, 1, 2, 4, 5, 6, 7, 12, 14, 15]);
}