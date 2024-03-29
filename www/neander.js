'use strict';

require('./neander.html');
require('./style.css');
require('bootstrap');
require('jquery');

import { ProcessorViewModel} from "./view_components/components";

const rust = import('../crate-wasm/pkg');

rust
    .then(m => { m.greet('World!'); window.rustModule = m; load() } )
    .catch(console.error);

function load() {
  window.NeanderView = new NeanderViewModel(
      document.getElementById('neanderUi'),
      new window.rustModule.NeanderJS()
  );
}

class NeanderViewModel extends ProcessorViewModel {
    constructor(where, model) {
      super(where, model);
    }
}