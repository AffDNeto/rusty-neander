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
      this.addCell(row, "255");    
      this.addCell(row, "00");    
    }
  
  };
  
  updateRow(id, new_value) {
    var row = this.memory_table.children[id];
    
    if (row == undefined) {return;}
  
    var hex = Number(new_value).toString(16).padStart(2, '0').toUpperCase();
  
    row.children[1].textContent = new_value;
    row.children[2].textContent = hex;
  
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
