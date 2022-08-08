_$pi.define("app_a/download/download.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 577378026, true);
            t.n3 = direct_1.createDiv();
            direct_1.setClass(t.n3, 589147975);
            t.n4 = direct_1.createSpan();
            direct_1.setClass(t.n4, 2801188448);
            direct_1.setText(t.n4, "加载中 请稍后...");
            t.n5 = direct_1.createDiv();
            direct_1.setClass(t.n5, 2725163555);
            t.n6 = direct_1.createSpan();
            direct_1.setClass(t.n6, 2801188448);
            direct_1.setText(t.n6, w.processTxt);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n6, t.n5);
            direct_1.append(t.n5, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 577378026);
            if (dirty0 & 1) direct_1.setText(t.n6, w.processTxt);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});