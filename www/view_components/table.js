import {int2Hex} from "./components";

export class ProgramTableView {
    constructor(
        table_node, selected_label_node, input_node, memory_change_callback, size, paddings) {
        this.table_node = table_node;
        this.selected_label_node = selected_label_node;
        this.input_node = input_node;
        this.size = size;
        this.selectedRow = null;
        this.memory_change_callback = memory_change_callback;
        this.hex_pad_size = 2;
        this.dec_pad_size = 3;
        this.rendered_size = 100;
        this.paddings = paddings;
    }

    init() {
        this.init_table();
        this.init_input();
        this.initObserver();
    }

    init_input() {
        this.input_node.onkeyup = (ev) => {
            console.info("Input for memory", ev, this);
            if (ev.key === 'Enter' || ev.keyCode === 13) {
                this.change_value.bind(this)(ev);
            }
        };
        this.selected_row = 0;
        this.selected_label_node.textContent = this.selected_row;
    }

    change_value(ev) {
        this.memory_change_callback(this.selected_row, ev.target.value);
    }

    highlight_row(pos) {
        if( ! this.isPosRendered(pos) ) {
            this.renderPos(pos);
        }

        this.highlihtRenderedRow(pos);
    }

    select_row(e) {
        var row = e.target.parentElement;
        var index = row.rowIndex+this.currentIndex;
        this.selected_row = index;
        this.selected_label_node.textContent = index;
        let value = row.cells[1].textContent.slice(0, this.dec_pad_size);
        this.input_node.value = value;
        this.input_node.select();
    }

    init_table() {
        this.currentData = Array.from({length: this.size},
            () => ([0, "NOP"]));

        var tb = this.table_node.cloneNode();
        tb.onclick = this.select_row.bind(this);

        for (var i = 0, ii = this.rendered_size; i < ii; i++) {
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
        this.updateVirtualRow(id, new_value, false);
    }

    updateRowValues(where, new_value) {
        let dec_value = new_value[0].toString().padStart(this.dec_pad_size, '0')
        let hex_value = int2Hex(new_value[0]).padStart(this.hex_pad_size, '0')
        where.children[1].textContent = dec_value + " [" + hex_value + "]";
        where.children[2].textContent = new_value[1];
    }

    updateTable(newData) {
        if (newData === undefined) {
            throw "No data given."
        }
        if (newData.length < this.size) {
            throw `New data size (${newData.length}) doesn't match the size the table was created ${this.size}`
        }

        //lazy update
        for (var i = 0; i < this.size; i++) {
            if (this.currentData[i][0] !== newData[i][0] || this.currentData[i][1] !== newData[i][1]) {
                this.updateRow(i, newData[i])
                break;
            }
        }
    }

    /// Methods related to the 'infinite' scrolling of the table
    initObserver(){
        const options = {
            root: this.table_node.parentNode.parentNode,
            threshold: 1.0
        }

        const callback = this.observerCallback.bind(this);

        this.trigger_observable = true;
        this.observer = new IntersectionObserver(callback, options);
        this.observer.observe(this.table_node.rows[0]);
        this.observer.observe(this.table_node.rows[this.rendered_size-1]);
        this.currentIndex = 0;

        this.adjustPadding(false);

    }

    observerCallback(entries) {
        entries.forEach(entry => {
            if(!this.trigger_observable){
                return
            }
            if(entry.target === this.table_node.rows[0]) {
                this.extendTopCallback(entry);
            } else if(entry.target === this.table_node.rows[this.rendered_size-1]){
                this.extentdBottomCallback(entry)
            }
        })
    }

    getSlidingWindow(isScrollDown) {
        const increment = this.rendered_size/2;
        let firstIndex;

        if(isScrollDown) {
            firstIndex = this.currentIndex+increment;
        }else{
            firstIndex = this.currentIndex-increment;
        }

        if ( firstIndex < 0 ){
            firstIndex = 0;
        }else if((firstIndex+this.rendered_size) > this.size ){
            firstIndex = this.size-this.rendered_size;
        }

        return firstIndex
    }

    updateVirtualRow(id, new_value, update_index) {
        let rendered_id = id - this.currentIndex;
        //Check if in range of the rendered elements
        if(rendered_id >= 0 && rendered_id < this.rendered_size){
            let row = this.table_node.children[rendered_id];

            if (typeof row === 'undefined') {
                console.error("Couldn't find row number" + rendered_id)
                return
            }

            if(update_index) {
                let index_text = Number(id).toString().padStart(this.dec_pad_size, '0');
                row.firstChild.textContent = index_text;
            }
            this.updateRowValues(row, new_value);
        }
    }

    recycleDOM() {
        for( let i = this.currentIndex; i < this.currentIndex+this.rendered_size; i++) {
            this.updateVirtualRow(i, this.currentData[i], true);
        }
    }

    extendTopCallback(entry) {
        console.warn("Top callback")
        if(this.currentIndex === 0) {
            // this.paddings[0].style.height = "0px";
            return
        }
        if(entry.isIntersecting) {
            this.currentIndex = this.getSlidingWindow(false);
            this.recycleDOM();
            this.adjustPadding();
        }
    }

    extentdBottomCallback(entry) {
        console.warn('Bottom callback');
        if(this.currentIndex === (this.size - this.rendered_size)) {
            return;
        }

        if (entry.isIntersecting){
            this.currentIndex = this.getSlidingWindow(true);
            this.recycleDOM();
            this.adjustPadding();
        }

    }

    adjustPadding() {
        let rowHeight = this.table_node.firstChild.getBoundingClientRect().height;
        let topPadding = this.currentIndex * rowHeight;
        let bottomPadding = (this.size - (this.currentIndex+this.rendered_size)) * rowHeight;
        let scrollContainer = this.table_node.parentNode.parentNode;

        let scrollY = scrollContainer.scrollTop;
        this.paddings[0].style.height = topPadding + 'px';
        this.paddings[1].style.height = bottomPadding + 'px';
        scrollContainer.scrollTo(0, scrollY)
    }

    isPosRendered(pos) {
        return (pos > this.currentIndex && pos < (this.currentIndex+this.rendered_size));
    }

    renderPos(pos) {
        let target = pos - (this.rendered_size/2);

        if ( target < 0 ) {
            target = 0;
        }else if(target > (this.size-this.rendered_size)) {
            target = this.size-this.rendered_size;
        }

        this.trigger_observable = false;

        this.currentIndex = target;
        this.recycleDOM();
        this.adjustPadding();

        this.trigger_observable = true;
    }

    highlihtRenderedRow(pos){
        pos = pos - this.currentIndex
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
}

export class DataTableView extends ProgramTableView {

    updateRowValues(where, new_value) {
        let dec_value = new_value[0].toString().padStart(this.dec_pad_size, '0')
        let hex_value = int2Hex(new_value[0]).padStart(this.hex_pad_size, '0')
        where.children[1].textContent = dec_value + " [" + hex_value + "]";
    }

    addRow(where, row_id) {
        var row = document.createElement("tr")

        this.addCell(row, row_id.toString().padStart(this.dec_pad_size, '0'));
        this.addCell(row, '');
        this.updateRowValues(row, this.currentData[row_id])

        where.appendChild(row);
    }
}