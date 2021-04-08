'use strict';

// require('./ramses.html');
require('./components.js');
require('./style.css');
// require('bootstrap');
// require('jquery');

import { ProcessorViewModel, RegisterController, Flag } from "./components";

const rust = import('../crate-wasm/pkg');

rust
    .then(m => { m.greet('World!'); window.rustModule = m; load() } )
    .catch(console.error);

function load() {
    window.RamsesView = new CesarView(
        document.getElementById('cesarUi'),
        new window.rustModule.CesarJsInterface()
    );
}


export class CesarRegisterController extends RegisterController {
    constructor(where){
        super(where);
        
        this.accInput = this.node.querySelector('#raInput');
        this.rb = this.node.querySelector('#rbInput');
        this.rx = this.node.querySelector('#rxInput');
        this.btn_c = new Flag(this.node.querySelector('#btn-flag-c'));
    }

    registerSet(ra, rb, rx, pc, nFlag, zFlag, cFlag, memAccess, instAccess){
        super.registerSet(ra, pc, nFlag, zFlag, memAccess, instAccess);

        this.rb.value = rb;
        this.rx.value = rx;
        this.btn_c.set_flag(cFlag);
    }

    init() {
        this.registerSet(
            0,0,0,1,
            true,true,true,
            0,0
        );
    }
}
export class CesarView extends ProcessorViewModel {
    constructor(node, model){
        super(node, model);
    }

    setupMemoryView(event) {
        this.memory_size = 10024;
        super.setupMemoryView(event);
    }

    createRegisterController(node) {
        return new CesarRegisterController(node);
    }

    updateRegisterView(state) {
        this.reg.registerSet(
            state.ra,
            state.rb,
            state.rx,
            state.pc,
            state.nf,
            state.zf,
            state.cf,
            state.mem_access_counter,
            state.instruction_counter
        )
    }
}