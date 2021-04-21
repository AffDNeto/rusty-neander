'use strict';

require('./ahmes.html');
require('.//components.js')
require('./style.css');
require('bootstrap');
require('jquery');

import {ProcessorViewModel, RegisterController, Flag, NeanderMnemonicDecoder} from "./components";

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
        
        this.btn_v = new Flag(this.node.querySelector('#btn-flag-v'));
        this.btn_c = new Flag(this.node.querySelector('#btn-flag-c'));
        this.btn_b = new Flag(this.node.querySelector('#btn-flag-b'));
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

    setupRiView() {
        super.setupRiView();
        this.decoder = new AhmesMnemonicDecoder();
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

class AhmesMnemonicDecoder extends NeanderMnemonicDecoder {
    constructor() {
        super();
        let ahmesTable = {
            112: [1, "SUB end"],
            148: [1, "JP end"],
            152: [1, "JV end"],
            156: [1, "JNV end"],
            164: [1, "JNZ end"],
            176: [1, "JC end"],
            180: [1, "JNC end"],
            184: [1, "JB end"],
            188: [1, "JNB end"],
            224: [0, "SHR end"],
            225: [0, "SHL end"],
            226: [0, "ROR end"],
            227: [0, "ROL end"]
        };

        this.decodingTable = Object.assign({}, this.decodingTable, ahmesTable);
    }

    decodeRI(ri) {
        let code = ri & parseInt("11110000", 2); // Extract only the first 4 bits
        var mnem = this.decodingTable[code];

        if (code >= parseInt("90", 16) &&
            code <= parseInt("B0", 16)
        ) {
            // Check which jump it is
            let jump_code = ri & parseInt("11111100");
            mnem = this.decodingTable[jump_code];
        }
        if( parseInt(code) === parseInt("E0", 16) ){
            // Check which bit shift it is
            let shift_code = ri & parseInt("11110011", 2);
            mnem = this.decodingTable[shift_code];
        }

        if(mnem !== undefined) {
            return mnem;
        }else {
            return this.decodingTable[256];
        }
    }
}