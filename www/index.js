// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
require('./style.css');
require('./neander.js');
require('./ahmes.js');
require('./nav-bar.html');
require('bootstrap');

import { AhmesView } from "./ahmes";
import { 
  ProgramTableView,
  NeanderViewModel, 
  RegisterController,
  readMemFile
} from "./neander";

const rust = import('../crate-wasm/pkg');

var rustModule;
rust
  .then(m => { m.greet('World!'); window.rustModule = m; load() } )
  .catch(console.error);

function load() {
  window.mem = ProgramTableView;
  window.reg = RegisterController;
  window.view = NeanderViewModel;

  window.NeanderView = new NeanderViewModel(
    document.getElementById('neanderUi'), 
    new window.rustModule.NeanderJS()
    );
  
  window.ahmes = new AhmesView(
    document.getElementById('ahmesUi'),
    new window.rustModule.AhmesJS()
    );
}