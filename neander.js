import Clusterize from "clusterize.js";

const data1 = [
  ["id", "value"],
  ['0', 125],
  ['0', 124],
  ['0', 123],
  ['0', 122]
];

mem_row_template = 
function(id, value){
  return `
  <td>${id}</td>
  <td><input type="text" id="mem_${id}" value="${value}"></td>
  <td></td>`;
}

var data = [];

for ( var i = 0, ii = 256; i < ii; i++ ) {
  data.push(mem_row_template(i, '0'));
}

var clust = new Clusterize({
  rows: data,
  scrollId: 'scrollArea',
  contentId: 'memContent' 
})