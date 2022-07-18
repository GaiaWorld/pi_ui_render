_$pi.define("app_c/fight/hp.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
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
            direct_1.extendAttr(t.n2, w, 3375223230, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 0 /*width*/, w.hp / w.maxHp * 100 + '%');
            direct_1.setClass(t.n3, 2499346767);
            t.n4 = direct_1.createDiv();
            direct_1.setClass(t.n4, 246535683);
            t.n5 = direct_1.createSpan();
            direct_1.setClass(t.n5, 3311419710);
            direct_1.setText(t.n5, w.hp + '/' + w.maxHp);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n5, t.n4);
            direct_1.append(t.n4, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 3375223230);
            if (dirty0 & 3) direct_1.setStyle(t.n3, 0 /*width*/, w.hp / w.maxHp * 100 + '%');
            if (dirty0 & 3) direct_1.setText(t.n5, w.hp + '/' + w.maxHp);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});