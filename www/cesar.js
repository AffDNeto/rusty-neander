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
    window.ModelView = new CesarView(
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
        this.setupVisor();
    }

    setupVisor(){
        this.visor = this.node.querySelector('#visor-area')
        document.onkeypress = (event) => {
            var key = event.which || event.keyCode;
            if (!this.running){
                console.info('Ignored visor input', event)
                return;
            }
            if (key <= 255){
                console.info("Sending keyboard input to cesar", event)
                this.cpu.input_keyboard(key);
            }
        }

    }

    setupRiView() {
        super.setupRiView();
        this.decoder = new CesarMnemonicDecoder();
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
        state.mem = mem;
        super.updateMemoryView(state);
        this.updateVisor();
    }

    changeMemoryValue(position, new_value) {
        super.changeMemoryValue(position, new_value);
        this.updateVisor();
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

    updateVisor() {
        var new_visor = this.cpu.get_visor();
        const space_placeholder = " "; //'·';
        new_visor = new_visor.replaceAll(' ', space_placeholder);

        this.visor.textContent = ">"+new_visor+"<";

    }

    updateRiView(state) {
        let ri = state.ri;
        let decodedRi = this.decoder.decodeInstruction(ri);
        this.riView.value = ri[0];

        if (parseInt(decodedRi[0]) >= 1 ){
            this.riView.value += " " + ri[1];
        }

        this.mnemView.value = decodedRi[1].replace('end', ri[1]);
    }

    decodeInstructionFromPosition(position) {
        let instruction_codes = [
            this.current_memory[position][0],
            ((position+1)<this.memory_size)?this.current_memory[position+1][0]:0
        ]
        var decoded_instruction = this.decoder.decodeInstruction(instruction_codes);

        let split_mnem = decoded_instruction[1].split(' ');

        if (split_mnem[1] !== undefined && split_mnem[1].split('end').length > 1) {
            decoded_instruction[1] = decoded_instruction[1].replace('end', this.getWord(position+2));
        }

        if (split_mnem[2] !== undefined && split_mnem[2].split('end').length > 1) {
            decoded_instruction[1] = decoded_instruction[1].replace('end', this.getWord(position+4));
        }

        return decoded_instruction;
    }

    getWord(position) {
        if (position+1 < this.memory_size) {
            // Check if both bytes are inside the memory
            return (this.current_memory[position][0] << 8)+ this.current_memory[position+1][0]
        }else{
            return '???'
        }
    }
}

class CesarMnemonicDecoder {
    constructor() {
        this.unknown = [0, '???'];
        this.decodingTable = {
            0: [0, "NOP"],
            1: this.decodeFlagOp,
            2: this.decodeFlagOp,
            3: this.decodeBranch,
            4: this.decodeJMP,
            5: this.decodeSOB,
            6: this.decodeJSR,
            7: this.decodeRTS,
            8: this.decodeSingleOp,
            9: this.decodeDoubleOp,
            10: this.decodeDoubleOp,
            11: this.decodeDoubleOp,
            12: this.decodeDoubleOp,
            13: this.decodeDoubleOp,
            14: this.decodeDoubleOp,
            15: [0, "HLT"],
        }

        this.branchTable = {
            0: "BR",
            1: "BNE",
            2: "BEQ",
            3: "BPL",
            4: "BMI",
            5: "BVC",
            6: "BVS",
            7: "BCC",
            8: "BCS",
            9: "BGE",
            10: "BLT",
            11: "BGT",
            12: "BLE",
            13: "BHI",
            14: "BLS",
        }

        this.modeTable = {
            0: "reg",
            1: "(reg)+",
            2: "-(reg)",
            3: "end(reg)",
            4: "(reg)",
            5: "((reg)+)",
            6: "(-(reg))",
            7: "(end(reg))",
        }

        this.singleOpTable = {
            0: "CLR",
            1: "NOT",
            2: "INC",
            3: "DEC",
            4: "NEG",
            5: "TST",
            6: "ROR",
            7: "ROL",
            8: "ASR",
            9: "ASL",
            10: "ADC",
            11: "SBC",
        }

        this.doubleOpTable = {
            1: "MOV",
            2: "ADD",
            3: "SUB",
            4: "CMP",
            5: "AND",
            6: "OR",
        }
    }

    extractCode(value, mask, radix=2) {
        return value & parseInt(mask, radix)
    }

    decodeRegMode(reg, mode){
        var arity = 0;
        mode = Number(mode);
        if ((mode === 3 || mode === 7)
            || (Number(reg) === 7 && (mode === 1 || mode === 5))
        ) {
            arity += 2; // Instruction uses the next 2 positions to execute
        }

        return [this.modeTable[mode].replace('reg', "R"+reg), arity];
    }

    decodeFlagOp(ri) {
        var code = (ri[0] & parseInt("f0", 16)) >> 4;
        var mnem;
        if (parseInt(code) === 1) {
            mnem = "CCC "
        }else {
            mnem = "SCC "
        }

        if ( (ri[0] & parseInt("1000", 2)) !== 0) {
            mnem += "N"
        }
        if ( (ri[0] & parseInt("0100", 2)) !== 0) {
            mnem += "Z"
        }
        if ( (ri[0] & parseInt("0010", 2)) !== 0) {
            mnem += "V"
        }
        if ( (ri[0] & parseInt("0001", 2)) !== 0) {
            mnem += "C"
        }

        return [0, mnem]
    }

    decodeBranch(ri) {
        let branch = this.extractCode(ri[0], '0f', 16);
        var mnem = this.branchTable[branch];

        if (mnem === undefined) { return  this.unknown }

        mnem += " "+ri[1];
        return [1, mnem]
    }

    decodeJMP(ri) {
        console.debug("jump", ri)
        let reg = this.extractCode(ri[1], '111', 2);
        let mode = this.extractCode(ri[1], '111000', 2) >> 3;
        let regMode = this.decodeRegMode(reg, mode)
        let mnem = "JMP "+regMode[0];
        return [1+regMode[1], mnem]
    }

    decodeSOB(ri) {
        console.debug("sob", ti);
        let r = this.extractCode(ri[0],'111', 2);

        return [1, "SOB R"+r+",end"]
    }

    decodeJSR(ri) {
        console.debug("jsr", ri)
        let r1 = this.extractCode(ri[0], '111', 2);
        let r2 = this.extractCode(ri[1], '111', 2);
        let mode = this.extractCode(ri[1], '111000', 2) >> 3;
        let regMode = this.decodeRegMode(r2, mode)

        return [1+regMode[1], "JSR R"+r1+","+regMode[0]];
    }

    decodeRTS(ri) {
        console.debug('rts', ri);
        let r = this.extractCode(ri[0], '111', 2);
        return [0, "RTS R"+r]
    }

    decodeSingleOp(ri) {
        let code = this.extractCode(ri[0], '1111', 2);
        let reg = this.extractCode(ri[1], '111', 2);
        let mode = this.extractCode(ri[1], '111000', 2) >> 3;
        let regMode = this.decodeRegMode(reg,mode);

        var mnem = this.singleOpTable[code]+" "+regMode[0];

        return [1+regMode[1], mnem]
    }

    decodeDoubleOp(ri) {
        console.debug("double op ", ri);
        let code = this.extractCode(ri[0], '1110000', 2) >> 4;
        let r_src = this.extractCode(ri[0], '1', 2) << 2 +
                    this.extractCode(ri[1], '11000000', 2) >> 6;
        let m_src = this.extractCode(ri[0], '1110', 2) >> 1;
        let r_dst = this.extractCode(ri[1], '111', 2) ;
        let m_dst = this.extractCode(ri[1], '111000', 2) >> 3;

        let reg_mode_src = this.decodeRegMode(r_src, m_src);
        let reg_mode_dst = this.decodeRegMode(r_dst, m_dst);

        let mnem = [this.doubleOpTable[code], reg_mode_src[0], reg_mode_dst[0]].join(" ");
        let arity = 1 + reg_mode_src[1] + reg_mode_dst[1];

        return [arity, mnem];
    }

    decodeInstruction(ri) {
        console.debug('Decoding '+ri)
        let code = (ri[0] & parseInt('11110000', 2)) >> 4;
        let mnem = this.decodingTable[code];

        if( mnem === undefined) {return this.unknown}

        if ( (typeof mnem) === "function" ) {
            try {
                mnem = mnem.bind(this)(ri);
            } catch (e) {
                console.error(e)
                mnem = this.unknown;
            }
        }

        return mnem;
    }
}