'use strict';

// require('./ramses.html');
require('./components.js');
require('./style.css');
// require('bootstrap');
// require('jquery');

import {ProcessorViewModel, RegisterController, Flag, read65kMemFile} from "./components";

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
        
        this.rx = Array(8);
        for(const id of this.rx.keys()) {
            this.rx[id] = this.node.querySelector('#r'+id+'-input');
        }
        this.accInput = this.rx[0];
        this.pcInput = this.rx[7];
        this.btn_c = new Flag(this.node.querySelector('#btn-flag-c'));
        this.btn_v = new Flag(this.node.querySelector('#btn-flag-v'));
    }

    registerSet(rx, nFlag, zFlag, cFlag, vFlag, memAccess, instAccess){
        super.registerSet(rx[0], rx[7], nFlag, zFlag, memAccess, instAccess);

        for (const id of this.rx.keys()) {
            this.rx[id].value = rx[id];
        }
        this.btn_c.set_flag(cFlag);
        this.btn_v.set_flag(vFlag);
    }

    init() {
        this.registerSet(
            Array(8).fill(0),
            true,
            true,
            true,
            true,
            0,
            0
        );
    }
}
export class CesarView extends ProcessorViewModel {
    constructor(node, model){
        super(node, model);
    }

    setupMemoryView(event) {
        this.memory_size = 1024;
        super.setupMemoryView(event);
    }

    readMemFile(file) {
        return read65kMemFile(file);
    }

    createRegisterController(node) {
        return new CesarRegisterController(node);
    }

    updateRegisterView(state) {
        this.reg.registerSet(
            state.rx,
            state.nf,
            state.zf,
            state.cf,
            state.vf,
            state.mem_access_counter,
            state.instruction_counter
        )
    }
}