function int2Hex(value) {
    return Number(value)
        .toString(16)
        .toUpperCase();
}

export class ProgramTableView {
    constructor(
        table_node, selected_label_node, input_node, memory_change_callback, size) {
        this.table_node = table_node;
        this.selected_label_node = selected_label_node;
        this.input_node = input_node;
        this.size = size;
        this.selectedRow = null;
        this.memory_change_callback = memory_change_callback;
        this.hex_pad_size = 2;
        this.dec_pad_size = 3;
    }

    init() {
        this.init_table();
        this.init_input();
    }

    init_input() {
        this.input_node.onkeyup = (ev) => {
            console.info("Input for memory", ev, this);
            if (ev.key === 'Enter' || ev.keyCode === 13){
                this.change_value.bind(this)(ev);
            }
        };
        this.selected_row = 0;
        this.selected_label_node.textContent = this.selected_row;
    }

    change_value(ev) {
        this.memory_change_callback(this.selected_row, ev.target.value);
    }

    highlight_row(pos){
        let row = this.table_node.rows[pos];
        if (row === null || typeof row == "undefined") {
            console.error('Out of range index for table:', {pos});
        }
        console.debug(row)
        row.scrollIntoView(true);
        if (!row.classList.contains("table-active")) {
            row.classList.add('table-active');
            if (this.selectedRow != null) {
                this.selectedRow.classList.remove('table-active');
            }
            this.selectedRow = row;
        }
    }

    select_row(e) {
        var row = e.target.parentElement;
        var index = row.rowIndex;
        this.selected_row = index;
        this.selected_label_node.textContent = index;
        let value = row.cells[1].textContent.slice(0, this.dec_pad_size);
        this.input_node.value = value;
        this.input_node.select();
    }

    init_table() {
        this.currentData = Array.from({length:this.size},
            ()=>([0, "NOP"]));

        var tb = this.table_node.cloneNode();
        tb.onclick = this.select_row.bind(this);

        for (var i = 0, ii = this.size; i < ii; i++ ) {
            this.addRow(tb, i);
        }

        this.table_node.replaceWith(tb);
        this.table_node = tb;
    }

    addRow(where, row_id) {
        var row = document.createElement("tr")

        this.addCell(row, row_id.toString().padStart(this.dec_pad_size, '0'));
        this.addCell(row, '');
        this.addCell(row, '');
        this.updateRowValues(row, this.currentData[row_id]);

        where.appendChild(row);
    }
    addCell(where, what) {
        var text_node = document.createTextNode(what);
        var cell = document.createElement("td");
        cell.appendChild(text_node);
        where.appendChild(cell);
    }

    updateRow(id, new_value) {
        this.currentData[id] = new_value;
        var row = this.table_node.children[id];

        if (typeof row === 'undefined') {
            console.error("Couldn't find row number"+id)
            return
        }

        this.updateRowValues(row, new_value);
    }
    updateRowValues(where, new_value) {
        let dec_value = new_value[0].toString().padStart(this.dec_pad_size, '0')
        let hex_value = int2Hex(new_value[0]).padStart(this.hex_pad_size, '0')
        where.children[1].textContent = dec_value + " ["+ hex_value +"]";
        where.children[2].textContent = new_value[1];
    }
    updateTable(newData) {
        if ( newData === undefined ) { throw "No data given." }
        if ( newData.length < this.size ) {
            throw `New data size (${newData.length}) doesn't match the size the table was created ${this.size}`
        }

        //lazy update
        for (var i = 0; i < this.size; i++){
            if(this.currentData[i][0] !== newData[i][0] || this.currentData[i][1] !== newData[i][1]) {
                this.updateRow(i, newData[i])
                break;
            }
        }
    }
}

export class DataTableView extends ProgramTableView {
    constructor(table_node, selected_label_node, input_node, memory_change_callback, size) {
        super(table_node, selected_label_node, input_node, memory_change_callback, size);
    }

    updateRowValues(where, new_value) {
        let dec_value = new_value[0].toString().padStart(this.dec_pad_size, '0')
        let hex_value = int2Hex(new_value[0]).padStart(this.hex_pad_size, '0')
        where.children[1].textContent = dec_value + " ["+ hex_value +"]";
    }

    addRow(where, row_id) {
        var row = document.createElement("tr")

        this.addCell(row, row_id.toString().padStart(this.dec_pad_size, '0'));
        this.addCell(row, '');
        this.updateRowValues(row, this.currentData[row_id])

        where.appendChild(row);
    }
}

export class RegisterController {
    constructor(where){
        this.node = where

        this.accInput = this.node.querySelector(`#accInput`);
        this.pcInput = this.node.querySelector(`#pcInput`);
        this.btn_n = new Flag(this.node.querySelector("#btn-flag-n"))
        this.btn_z = new Flag(this.node.querySelector("#btn-flag-z"))
        this.access = this.node.querySelector(`#memAccess`);
        this.instructions = this.node.querySelector(`#instCount`);
    }

    registerSet(acc, pc, nFlag, zFlag, memAccess, instAccess){

        this.accInput.value = acc;
        this.pcInput.value = pc;
        this.access.value = memAccess;
        this.instructions.value = instAccess;
        this.btn_n.set_flag(nFlag);
        this.btn_z.set_flag(zFlag);
    }

    init(){
        this.registerSet(0,0,1,1,0,0,0);
    }
}

export class Flag{
    constructor(node) {
        this.node = node
        this.set_flag(false)
    }

    set_flag(set){
        if(set) {
            this.node.classList.remove("btn-danger");
            this.node.classList.add("btn-success");
        }else {
            this.node.classList.remove("btn-success");
            this.node.classList.add("btn-danger");
        }

    }
}

export class ProcessorViewModel {
    constructor(node, model) {
        this.node = node;
        this.cpu = model;
        this.memory_size = 256;
        this.running = false;
        process.nextTick(() => {
            this.setupMemoryView()
            this.setupRegistersView()
            this.setupExecuteView();
            this.setupMemImporter();
            this.setupRiView();
            this.updateView();
        })

    }

    readMemFile(file) {
        return readMemFile(file)
    }

    setupMemImporter(){
        this.memFileInput = this.node.querySelector("#memFile");
        this.memFileInput.onchange = (e) => {
            console.debug("loading file")
            var reader = new FileReader();
            reader.onload = (e) => {
                console.debug("file loaded  ")
                var mem = this.readMemFile(e.target.result);
                this.cpu.load_mem(mem);
                this.cpu.clear_counters();
                this.cpu.set_pc(0);
                this.updateView();
            }
            reader.readAsArrayBuffer(e.target.files[0]);


        }
        this.fileReader = new FileReader();
    }
    setupMemoryView(event) {
        this.current_memory = Array.from({length:this.memory_size},
                                    ()=>([0, "NOP"]));

        var p_table = this.node.querySelector(`#memContainer`);
        var programViewInput = this.node.querySelector(`#memInput`);
        var programRowSelected = this.node.querySelector(`#selMem`);
        this.programView = new ProgramTableView(
            p_table, programRowSelected, programViewInput,
            this.changeMemoryValue.bind(this), this.memory_size);
        this.programView.init();

        var d_table = this.node.querySelector(`#dataContainer`);
        var dataViewInput = this.node.querySelector(`#dataInput`);
        var dataRowSelected = this.node.querySelector(`#selData`);
        this.dataView = new DataTableView(
            d_table, dataRowSelected, dataViewInput,
            this.changeMemoryValue.bind(this), this.memory_size);
        this.dataView.init()

        this.programView.updateTable(this.current_memory);
        this.dataView.updateTable(this.current_memory);
    }

    changeMemoryValue(position, new_value) {
        new_value = Number(new_value);
        if (!isNaN(new_value) && new_value >= 0 && new_value < 256) {
            this.cpu.set_mem(position, new_value);
            this.updateMemoryView(this.cpu.get_state());
        }
    }

    setupExecuteView() {
        this.stepsInput = this.node.querySelector(`#stepNum`);
        this.excuteSteps = 100;
        this.stepsInput.value = this.excuteSteps;
        this.stepsInput.onchange = this.updateExecuteSteps.bind(this);

        this.btnStep = this.node.querySelector(`#btnStep`);
        this.btnStep.onclick = this.step.bind(this);

        this.btnRun = this.node.querySelector(`#btnRun`);
        this.btnRun.onclick = this.run.bind(this);

        this.btnStop = this.node.querySelector("#btnStop")
        this.btnStop.onclick = this.stop.bind(this);
    }

    stop(){
        this.running = false;
    }

    createRegisterController(node) {
        return new RegisterController(node);
    }
    setupRegistersView() {
        var node = this.node.querySelector(`#registerContainer`);
        this.reg = this.createRegisterController(node);
        this.reg.init();
        this.setupRegOnchangeCallbacks();
        this.clear_btn = this.node.querySelector("#clear-registers")
        this.clear_btn.onclick = (_) => {
            this.cpu.clear_counters();
            this.updateView();
        };
    }

    setupRiView() {
        console.log("Setting up decoder")
        this.riView = this.node.querySelector("#riInput");
        this.mnemView = this.node.querySelector("#mnemInput");
        this.decoder = new NeanderMnemonicDecoder();
    }
    setupRegOnchangeCallbacks(){
        //sets up the callbacks for updating the registers
        this.reg.pcInput.onchange = (e) => {
            var nv = Number(e.target.value);
            if ( isNaN(nv) || !(0 <= nv && nv <= 255) )  return false
            this.cpu.set_pc(nv)
            this.programView.highlight_row(nv);
            this.reg.pcInput.value = nv
            e.target.value = nv
            return true
        };

        this.reg.accInput.onchange = (e) => {
            var nv = Number(e.target.value);
            if ( isNaN(nv) || !(0 <= nv && nv <= 255) )  return false
            this.cpu.set_acc(nv)
            this.reg.accInput.value = nv
            return true
        };
    }

    updateExecuteSteps(event){
        var newValue = Number(event.target.value);
        if ( isNaN(newValue) || newValue < 1 ) {
            this.stepsInput.value = this.excuteSteps;
        }

        this.excuteSteps = newValue;
    }

    step(){
        this.cpu.execute(1);
        this.updateView();
    }

    run() {
        if (!this.running) {
            this.running = true
            setImmediate( () => {
                this.continue(this.excuteSteps);
            });
        }
    }

    continue( remainingSteps ){
        if (remainingSteps <= 0) {this.running = false;}

        if(this.running) {
            var result = this.cpu.execute(1);
            this.updateView();

            this.running = result;
            setTimeout(() => {
                this.continue(remainingSteps - 1);
            }, 1)
        }

    }

    updateMemoryView(state) {
        var delta = this.checkDataChanges(state.mem);
        var changed = []
        for(let i of delta) {
            changed = changed.concat(this.updateMnemonic(i));
        }
        delta = [...new Set(changed)].sort();

        for (let i of delta) {
            this.programView.updateRow(i, this.current_memory[i]);
            this.dataView.updateRow(i, this.current_memory[i]);
        }
        this.programView.highlight_row(state.pc);
    }

    updateMnemonic(delta_pos){
        var instruction_start_pos = delta_pos;
        var changed_pos = [];
        while(instruction_start_pos !== 0 && this.current_memory[instruction_start_pos][1] === ""){
            /// Empty mnemonic position mean that pos is part of a instruction
            instruction_start_pos--;
        }

        // Line changed is already the begining of a instruction
        var decoded_instruction = this.decodeInstructionFromPosition(instruction_start_pos);

        let address_pos = instruction_start_pos+1;
        let instr_arity = decoded_instruction[0];

        this.current_memory[instruction_start_pos][1] = decoded_instruction[1];
        changed_pos.push(instruction_start_pos);
        if(instr_arity > 1){
            console.log('here');
        }
        for (var i = 1; i< this.memory_size && i <= instr_arity; i++){
            // Empty position with no mnemonic
            this.current_memory[instruction_start_pos+i][1] = "";
            changed_pos.push(instruction_start_pos+i);
        }

        let after_instr = instruction_start_pos+instr_arity+1;
        if(after_instr < this.memory_size && this.current_memory[after_instr][1] === ""){
            // If mnen is empty, this position was part of the previous instruction
            this.current_memory[after_instr][1] = "??";
            changed_pos = changed_pos.concat(this.updateMnemonic(after_instr));
        }

        return changed_pos;
    }

    decodeInstructionFromPosition(position) {
        var decoded_instruction = this.decoder.decodeInstruction(
            this.current_memory[position][0]
        );
        if (position+1 < this.memory_size ) {
            decoded_instruction[1] = decoded_instruction[1].replace(
                    'end', this.current_memory[position+1][0]
            );
        }
        return decoded_instruction;
    }
    checkDataChanges(data){
        var position_delta = [];
        for ( var i = 0; i < this.memory_size; i++){
            if(data[i] !== this.current_memory[i][0]) {
                /// Updates value on current memory
                this.current_memory[i][0] = data[i]
                position_delta.push(i);
            }
        }

        return position_delta;
    }

    updateRegisterView(state) {
        this.reg.registerSet(state.acc, state.pc, state.nf, state.zf, state.mem_access_counter, state.instruction_counter);
    }

    updateView(){
        var state = this.cpu.get_state();

        this.updateMemoryView(state);
        this.updateRegisterView(state);
        this.updateRiView(state);
    }

    updateRiView(state) {
        let [line, i] = state.ri
        let decodedRI = this.decoder.decodeInstruction(i);
        this.riView.value = i;
        var mnem_value = decodedRI[1];
        if(parseInt(decodedRI[0])===1) {
            this.riView.value += " " + state.mem[line+1];
            mnem_value = mnem_value.replace('end', state.mem[line+1]);
        }

        this.mnemView.value = mnem_value;
    }

}

export class InstructionMnemonic {
    constructor(arity, mnem) {
        this.arity = arity;
        this.mnem = mnem;
    }

    toString() { return this.mnem }
}

export class NeanderMnemonicDecoder {
    constructor() {
        this.decodingTable = {
            256: [0, " "],
            0: [0, "NOP"],
            16: [1, "STA end"],
            32: [1, "LDA end"],
            48: [1, "ADD end"],
            64: [1, "OR end"],
            80: [1, "AND end"],
            96: [0, "NOT"],
            128: [1, "JMP end"],
            144: [1, "JN end"],
            160: [1, "JZ end"],
            240: [0, "HLT"],
        }
    }

    // Decodes instruction based on it's value
    // and return it's arity and mnemonic
    decodeInstruction(ri){
        // Extract instruction code
        let code = ri & 240;
        let mnem = this.decodingTable[code];

        if (mnem === undefined) {
            return this.decodingTable[256];
        } else {
            return mnem
        }
    }
}

export function readMemFile(fileByteArray){
    const header = new Uint8Array(fileByteArray, 0, 4);
    const mem = new Uint8Array(fileByteArray, 4);
    var memArray = []
    for(var i=0; i < mem.length; i +=2){ // Skips every other byte because the file format is like this
        memArray[i/2] = mem[i];
    }

    return memArray

}

export function read65kMemFile(fileByteArray){
    const header = new Uint8Array(fileByteArray, 0, 4);
    const mem = new Uint8Array(fileByteArray, 4);
    var memArray = []
    for(var i=0; i < mem.length; i ++){ // Skips every other byte because the file format is like this
        memArray[i] = mem[i];
    }

    return memArray

}

