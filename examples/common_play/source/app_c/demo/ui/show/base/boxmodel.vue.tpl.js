_$pi.define("app_c/demo/ui/show/base/boxmodel.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [0.47, 0.47, 0.47, 1], [0, 400], [[0, 20], null, null, null, null], [1, 0, 0, 1], [0, 300], [[0, 5], null, null, null, null], [0, 1, 0, 1], [[0, 30], null, null, null, null]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 29 /*justifyContent*/, 2);
            direct_1.setStyle(t.n2, 27 /*alignContent*/, 2);
            direct_1.setStyle(t.n2, 28 /*alignItems*/, 2);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[1]);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n3, 11 /*padding*/, staticObj[3]);
            t.n4 = direct_1.createDiv();
            direct_1.setStyle(t.n4, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[5]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[5]);
            direct_1.setStyle(t.n4, 11 /*padding*/, staticObj[3]);
            direct_1.setStyle(t.n4, 51 /*borderWidth*/, staticObj[6]);
            direct_1.setStyle(t.n4, 52 /*borderColor*/, staticObj[7]);
            direct_1.setStyle(t.n4, 16 /*margin*/, staticObj[8]);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let {} = w._$info;
            direct_1.extendAttr(t.n2, w);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});