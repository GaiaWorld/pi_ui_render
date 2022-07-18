_$pi.define("app_a/widget/pop/pop.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 600], [0, 800], [1, 0, 0, 1], [1, 0.5], [{
        t: "t",
        d: [[1, -0.5], [1, -0.5]]
    }], [1, 1], [1, 1, 1, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 6 /*position*/, 1);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n2, 64 /*zIndex*/, 1);
            direct_1.setStyle(t.n2, 46 /*backgroundColor*/, staticObj[2]);
            direct_1.setStyle(t.n2, 7 /*top*/, staticObj[3]);
            direct_1.setStyle(t.n2, 10 /*left*/, staticObj[3]);
            direct_1.setStyle(t.n2, 58 /*transform*/, staticObj[4]);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 29 /*justifyContent*/, 2);
            direct_1.setStyle(t.n3, 28 /*alignItems*/, 2);
            direct_1.setStyle(t.n3, 27 /*alignContent*/, 2);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[5]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[5]);
            t.n4 = direct_1.createSpan();
            direct_1.setStyle(t.n4, 42 /*fontSize*/, 35);
            direct_1.setStyle(t.n4, 30 /*color*/, staticObj[6]);
            direct_1.setText(t.n4, "对话框测试");
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