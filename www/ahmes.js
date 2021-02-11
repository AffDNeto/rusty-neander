'use strict';
import { NeanderViewModel, RegisterController } from "./neander";


export class AhmesRegisterControler extends RegisterController {
    constructor(where){
        super(where);
        
        this.vFlag = this.div.querySelector('#overflowFlag');
        this.cFlag = this.div.querySelector('#carryFlag');
        this.bFlag = this.div.querySelector('#borrowFlag');
    }

    registerSet(acc, pc, nFlag, zFlag, vFlag, cFlag, bFlag, memAccess, instAccess){
        super.registerSet(acc, pc, nFlag, zFlag, memAccess, instAccess);
        this.vFlag.checked = vFlag;
        this.cFlag.checked = cFlag;
        this.bFlag.checked = bFlag;
    }

    init() {
        this.registerSet(0,0,1,1,1,1,1,0,0);
    }
}
export class AhmesView extends NeanderViewModel {
    constructor(node, model){
        super(node, model);
        this.setupExtraFlags()
    }

    setupRegistersView(){
        this.reg = this.node.querySelector("#registerContainer");
        this.reg = new AhmesRegisterControler(this.reg);
        this.reg.init();
        this.setupRegOnchangeCallbacks();
    }
    updateView(){
        var state = this.model.get_state();

        this.memMap.updateTable(state.mem);

        this.reg.registerSet(
            state.acc,
            state,pc,
            state.nf,
            state.zf,
            state.vf,
            state.cf,
            state.bf,
            state.mem_access_count,
            state.instruction_access
        )
    }
}