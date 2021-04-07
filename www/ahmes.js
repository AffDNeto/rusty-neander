'use strict';

require('./ahmes.html');
require('.//components.js')
require('./style.css');
require('bootstrap');
require('jquery');

import { ProcessorViewModel, RegisterController, Flag } from "./components";

const rust = import('../crate-wasm/pkg');

rust
    .then(m => { m.greet('World!'); window.rustModule = m; load() } )
    .catch(console.error);

function load() {
    window.AhmesView = new AhmesView(
        document.getElementById('ahmesUi'),
        new window.rustModule.AhmesJS()
    );
}


export class AhmesRegisterControler extends RegisterController {
    constructor(where){
        super(where);
        
        this.btn_v = new Flag(this.div.querySelector('#btn-flag-v'));
        this.btn_c = new Flag(this.div.querySelector('#btn-flag-c'));
        this.btn_b = new Flag(this.div.querySelector('#btn-flag-b'));
    }

    registerSet(acc, pc, nFlag, zFlag, vFlag, cFlag, bFlag, memAccess, instAccess){
        super.registerSet(acc, pc, nFlag, zFlag, memAccess, instAccess);
        this.btn_v.set_flag(vFlag);
        this.btn_c.set_flag(cFlag);
        this.btn_b.set_flag(bFlag);
    }

    init() {
        this.registerSet(0,0,1,1,1,1,1,0,0);
    }
}
export class AhmesView extends ProcessorViewModel {
    constructor(node, model){
        super(node, model);
    }

    createRegisterController(node) {
        return new AhmesRegisterControler(node);
    }

    updateRegisterView(state) {
        this.reg.registerSet(
            state.acc,
            state.pc,
            state.nf,
            state.zf,
            state.vf,
            state.cf,
            state.bf,
            state.mem_access_counter,
            state.instruction_counter
        )
    }
}