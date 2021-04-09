'use strict';

require('./components.js');
require('./style.css');

import {ProcessorViewModel, RegisterController, Flag, read65kMemFile} from "./components";
import('../crate-wasm/pkg/index_bg')
    .then( module => {
        window.wasm_bg = module;

        import('../crate-wasm/pkg')
            .then(m => {
                m.greet('World!'); window.rustModule = m; load()
            } )
            .catch(console.error);

    })



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
        this.memory_size = 1050;
        super.setupMemoryView(event);
    }

    readMemFile(file) {
        return read65kMemFile(file);
    }

    createRegisterController(node) {
        return new CesarRegisterController(node);
    }

    updateMemoryView(state) {
        let ptr = this.cpu.get_memory();
        let mem = new Uint8Array(wasm_bg.memory.buffer, ptr, this.memory_size);
        this.programView.updateTable(mem);
        this.dataView.updateTable(mem)
        this.programView.highlight_row(state.pc);
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