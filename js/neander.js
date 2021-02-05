'use strict';

export class MemTableControler {
  constructor(table, size) {
    this.memory_table = table;
    this.rowsLen = size
  }

  addCell(where, what) {
    var text_node = document.createTextNode(what);
    var cell = document.createElement("td");
  
    cell.appendChild(text_node);
    where.appendChild(cell);
  }

  init () {
    var tb = this.memory_table;
  
    for ( var i = 0, ii = this.rowsLen; i < ii; i++ ) {
      var row = tb.insertRow();
      row.classList.add("normalRow");
      
      this.addCell(row, i);    
      this.addCell(row, "0");    
      this.addCell(row, "00");    
    }
  
  };
  
  int2Hex(value) {
    return Number(value).toString(16).padStart(2, '0').toUpperCase();
  }

  updateRow(id, new_value) {
    var row = this.memory_table.children[id];
    
    if (row == undefined) {throw `Couldn't find row number ${id}`}
    
    row.children[1].textContent = new_value;
    row.children[2].textContent = this.int2Hex(new_value);
  
  }

  updateTable(newData) {
    if ( newData == undefined ) { throw "No data given." }
    if ( newData.length != this.rowsLen ) { 
      throw `New data size (${newData.length})doesn't match the size the table was created ${this.rowsLen}`
    }

    for ( var i = 0; i < this.rowsLen; i++){
      this.updateRow(i+1, newData[i]);
    }
  }
}

export class RegisterController {
  constructor(where){
    this.div = where

    this.accInput = document.querySelector(`#${this.div.id} #accInput`);
    this.pcInput = document.querySelector(`#${this.div.id} #pcInput`);
    this.nFlag = document.querySelector(`#${this.div.id} #negativeFlag`);
    this.zFlag = document.querySelector(`#${this.div.id} #zeroFlag`);
    this.access = document.querySelector(`#${this.div.id} #memAccess`);
    this.instructions = document.querySelector(`#${this.div.id} #instCount`);
  }

  registerSet(acc, pc, nFlag, zFlag, memAccess, instAccess){
    
    this.accInput.value = acc;
    this.pcInput.value = pc;
    this.nFlag.checked = nFlag;
    this.zFlag.checked = zFlag;
    this.access.value = memAccess;
    this.instructions.value = instAccess;

  }

  init(){
    this.registerSet(0,0,1,1,0,0,0);
  }

}

export class NeanderViewModel {
  constructor(node, model) {
    this.node = node;

    this.memMap = document.querySelector(`#${this.node.id} #memContainer`);
    this.memMap = new MemTableControler(this.memMap, 256);
    this.memMap.init();
    
    this.reg = document.querySelector(`#${this.node.id} #registerContainer`);
    this.reg = new RegisterController(this.reg);
    this.reg.init();
    
    this.cpu = model;
    this.updateView();

    this.stepsInput = document.querySelector(`#${this.node.id} #stepNum`);
    this.excuteSteps = 10;
    this.stepsInput.value = this.excuteSteps;

    this.btnStep = document.querySelector(`#${this.node.id} #btnStep`);
    this.btnStep.onclick = this.step.bind(this);
    
    this.btnRun = document.querySelector(`#${this.node.id} #btnRun`);
    this.btnRun.onclick = this.run.bind(this);
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

  run(){
    this.cpu.execute(this.excuteSteps);
    this.updateView();
  }

  updateView(){
    var state = this.cpu.get_state();

    this.memMap.updateTable(state.mem);

    this.reg.registerSet(state.acc, state.pc, state.nf, state.zf, state.mem_access_counter, state.instruction_counter);

  }



}
