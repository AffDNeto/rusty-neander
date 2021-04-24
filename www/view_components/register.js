export class RegisterController {
    constructor(where) {
        this.node = where

        this.accInput = this.node.querySelector(`#accInput`);
        this.pcInput = this.node.querySelector(`#pcInput`);
        this.btn_n = new Flag(this.node.querySelector("#btn-flag-n"))
        this.btn_z = new Flag(this.node.querySelector("#btn-flag-z"))
        this.access = this.node.querySelector(`#memAccess`);
        this.instructions = this.node.querySelector(`#instCount`);
    }

    registerSet(acc, pc, nFlag, zFlag, memAccess, instAccess) {

        this.accInput.value = acc;
        this.pcInput.value = pc;
        this.access.value = memAccess;
        this.instructions.value = instAccess;
        this.btn_n.set_flag(nFlag);
        this.btn_z.set_flag(zFlag);
    }

    init() {
        this.registerSet(0, 0, 1, 1, 0, 0, 0);
    }
}

export class Flag {
    constructor(node) {
        this.node = node
        this.set_flag(false)
    }

    set_flag(set) {
        if (set) {
            this.node.classList.remove("btn-danger");
            this.node.classList.add("btn-success");
        } else {
            this.node.classList.remove("btn-success");
            this.node.classList.add("btn-danger");
        }

    }
}