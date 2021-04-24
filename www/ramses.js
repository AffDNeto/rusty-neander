'use strict';

import {Flag, RegisterController} from "./view_components/register";

require('./ramses.html');
require('./view_components/components.js');
require('./style.css');
require('bootstrap');
require('jquery');

import {ProcessorViewModel, NeanderMnemonicDecoder} from "./view_components/components";

const rust = import('../crate-wasm/pkg');

rust
    .then(m => { m.greet('World!'); window.rustModule = m; load() } )
    .catch(console.error);

function load() {
    window.RamsesView = new RamsesView(
        document.getElementById('ramsesUi'),
        new window.rustModule.RamsesJsInterface()
    );
}


export class RamsesRegisterControler extends RegisterController {
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
export class RamsesView extends ProcessorViewModel {
    constructor(node, model){
        super(node, model);
    }

    setupRiView() {
        super.setupRiView();
        this.decoder = new RamsesMnemonicDecoder();
    }

    createRegisterController(node) {
        return new RamsesRegisterControler(node);
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

class RamsesMnemonicDecoder extends NeanderMnemonicDecoder {
    constructor() {
        super();
        let ramsesTable = {
            16: [1, "STR reg end"],
            32: [1, "LDR reg end"],
            48: [1, "ADD reg end"],
            64: [1, "OR reg end"],
            80: [1, "AND reg end"],
            96: [0, "NOT reg"],
            112: [1, "SUB reg end"],
            128: [1, "JMP end"],
            144: [1, "JN end"],
            160: [1, "JZ end"],
            176: [1, "JC end"],
            192: [1, "JSR end"],
            208: [0, "NEG reg"],
            224: [0, "SHR reg"],
        }
        this.decodingTable = Object.assign({}, this.decodingTable, ramsesTable)

        this.modeTable = {
            0: "end",
            1: "end,I",
            2: "#end",
            3: "end,X",
        }

        this.regTable = {
            0: "A",
            1: "B",
            2: "X",
            3: "?",
        }
    }

    decodeInstruction(ri) {
        let code = ri & parseInt('11110000', 2);
        let mode = ri & parseInt('00000011', 2);
        let reg = (ri & parseInt('00001100', 2)) >> 2;

        var mnem = this.decodingTable[code];

        if(mnem === undefined) {
            return this.decodingTable[256];
        }

        mnem[1] = mnem[1].replace('reg', this.regTable[reg]);
        mnem[1] = mnem[1].replace('end', this.modeTable[mode]);

        return mnem
    }
}