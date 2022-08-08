_$pi.define("app_a/widget/count_down/count_down.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
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
            direct_1.extendAttr(t.n2, w, 4016430791, true);
            t.n3 = direct_1.createSpan();
            direct_1.setStyle(t.n3, 30 /*color*/, w.color || '#a0abb1');
            direct_1.setStyle(t.n3, 42 /*fontSize*/, w.fontSize || 28 + 'px');
            direct_1.setClass(t.n3, 3437427013);
            direct_1.setText(t.n3, w.restTimeStr);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n3, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 4016430791);
            if (dirty0 & 1) direct_1.setStyle(t.n3, 30 /*color*/, w.color || '#a0abb1');
            if (dirty0 & 2) direct_1.setStyle(t.n3, 42 /*fontSize*/, w.fontSize || 28 + 'px');
            if (dirty0 & 4) direct_1.setText(t.n3, w.restTimeStr);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});