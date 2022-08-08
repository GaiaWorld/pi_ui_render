_$pi.define("app_c/demo/ui/menu/state", ["require", "exports", "module", "pi_gui/widget/forelet"], function (require, exports, module, forelet_1) {
    "use strict";

    exports.forelet = void 0;
    class Forelet1 extends forelet_1.Forelet {
        notify(eventType, args) {
            if (this.widgets && this.widgets.length > 0) {
                for (let w of this.widgets) {
                    w.notify(eventType, args);
                }
            }
            return true;
        }
    }
    exports.forelet = new Forelet1();
});