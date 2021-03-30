'use strict';

require('./neander.html');
require('./style.css');
require('bootstrap');
require('jquery');

const rust = import('../crate-wasm/pkg');

rust
    .then(m => { m.greet('World!'); window.rustModule = m; load() } )
    .catch(console.error);

function load() {
  window.mem = MemTableControler;
  window.reg = RegisterController;
  window.view = NeanderViewModel;

  window.NeanderView = new NeanderViewModel(
      document.getElementById('neanderUi'),
      new window.rustModule.NeanderJS()
  );
}

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
    tb.onclick = (e) => {
      let parent = e.target.parentElement;
      if ( parent.tagName == 'TR' ) {
        this.onRowClick(parent.rowIndex);
      }
    };

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

    this.accInput = this.div.querySelector(`#accInput`);
    this.pcInput = this.div.querySelector(`#pcInput`);
    this.nFlag = this.div.querySelector(`#negativeFlag`);
    this.zFlag = this.div.querySelector(`#zeroFlag`);
    this.access = this.div.querySelector(`#memAccess`);
    this.instructions = this.div.querySelector(`#instCount`);
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
    this.cpu = model;

    this.setupMemoryView()
    this.setupRegistersView()
    this.setupExecuteView();
    this.updateView();
    this.setupMemImporter();
  }

  setupMemImporter(){
    this.memFileInput = this.node.querySelector("#memFile");
    this.memFileInput.onchange = (e) => {
      console.log("laoding file")
      var reader = new FileReader();
      reader.onload = (e) => {
        console.log("file loaded  ")
        var mem = readMemFile(e.target.result);
        this.cpu.load_mem(mem)
        this.updateView();
      }
      reader.readAsArrayBuffer(e.target.files[0]);
      

    }
    this.fileReader = new FileReader();


  }
  setupMemoryView(event) {
    this.memMap = document.querySelector(`#${this.node.id} #memContainer`);
    this.memMap = new MemTableControler(this.memMap, 256);
    this.memMap.init();
    this.memMap.onRowClick = (r) => {this.updateSelectedRow(r);}
    
    this.memInput = document.querySelector(`#${this.node.id} #memInput`);
    this.memInput.onchange = (e) =>
    {
      var nv = Number(e.target.value);
      if ( isNaN(nv) || !(0 <= nv && nv <= 255) )  return false
      this.cpu.set_mem(this.selectedMemPos, nv);
      this.memMap.updateRow(this.selectedMemPos+1, nv);
      e.target.value = nv;
      return true
    }
    this.selectedMemPos = 0;
    this.posLabel = document.querySelector(`#${this.node.id} #selMem`);
    this.posLabel.textContent = this.selectedMemPos;

  }

  updateSelectedRow(rowIndex) {
    this.selectedMemPos = rowIndex-1;
    this.posLabel.textContent = this.selectedMemPos;
    this.memInput.value = this.cpu.get_state().mem[this.selectedMemPos];
    this.memInput.focus();
  }

  setupExecuteView() {
    this.stepsInput = document.querySelector(`#${this.node.id} #stepNum`);
    this.excuteSteps = 10;
    this.stepsInput.value = this.excuteSteps;

    this.btnStep = document.querySelector(`#${this.node.id} #btnStep`);
    this.btnStep.onclick = this.step.bind(this);
    
    this.btnRun = document.querySelector(`#${this.node.id} #btnRun`);
    this.btnRun.onclick = this.run.bind(this);
  }

  setupRegistersView() {
    this.reg = document.querySelector(`#${this.node.id} #registerContainer`);
    this.reg = new RegisterController(this.reg);
    this.reg.init();
    this.setupRegOnchangeCallbacks();
  }
  
  setupRegOnchangeCallbacks(){
    //sets up the callbacks for updating the registers
    this.reg.pcInput.onchange = (e) => { 
      var nv = Number(e.target.value);
      if ( isNaN(nv) || !(0 <= nv && nv <= 255) )  return false
      this.cpu.set_pc(nv)
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

export function readMemFile(fileByteArray){
  const header = new Uint8Array(fileByteArray, 0, 4);
  const mem = new Uint8Array(fileByteArray, 4);
  var memArray = []
  for(var i=0; i < mem.length; i +=2){ // Skips every other byte because the file format is like this
    memArray[i/2] = mem[i];
  }

  return memArray

}

window.readMemFile = readMemFile;