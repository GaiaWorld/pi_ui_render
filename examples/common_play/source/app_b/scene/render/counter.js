_$pi.define("app_b/scene/render/counter", ["require", "exports", "module"], function (require, exports, module) {
    "use strict";

    exports.Counter = void 0;
    class Counter {
        constructor() {
            this.id = 0;
            this.list = [];
        }
        recycle(index) {
            this.list.push();
        }
        get() {
            let temp = this.list.pop();
            if (temp == undefined) {
                temp = this.id++;
            }
            return temp;
        }
    }
    exports.Counter = Counter;
});