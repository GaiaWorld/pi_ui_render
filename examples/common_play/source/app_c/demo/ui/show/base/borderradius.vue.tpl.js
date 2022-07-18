_$pi.define("app_c/demo/ui/show/base/borderradius.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 700], [0, 400], [0, 100], [0.28, 0.95, 0.19, 1], [1, 0, 0, 1], [0, 30]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 61 /*display*/, 0);
            direct_1.setStyle(t.n2, 6 /*position*/, 1);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n2, 21 /*flexDirection*/, 0);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[3]);
            direct_1.setStyle(t.n3, 53 /*borderRadius*/, 20);
            t.n4 = direct_1.createDiv();
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n4, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n4, 53 /*borderRadius*/, 50);
            direct_1.setStyle(t.n4, 20 /*marginLeft*/, staticObj[5]);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n4, t.n2);
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