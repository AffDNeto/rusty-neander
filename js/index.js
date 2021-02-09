// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
require('./neander.js');
require('./neander.css');

import { 
  MemTableControler, 
  NeanderViewModel, 
  RegisterController,
  readMemFile
} from "./neander";

const rust = import('../pkg');

var rustModule;
rust
  .then(m => { m.greet('World!'); window.rustModule = m; load() } )
  .catch(console.error);

function load() {
  window.mem = MemTableControler;
  window.reg = RegisterController;
  window.view = NeanderViewModel;

  window.NeanderView = new NeanderViewModel(
    document.getElementById('neanderUi'), 
    new window.rustModule.NeanderJS()
    );

}