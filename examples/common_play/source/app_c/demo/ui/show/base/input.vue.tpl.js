_$pi.define("app_c/demo/ui/show/base/input.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [0.94, 0, 0, 1], [0, 200], [0, 40], [0.94, 0.56, 0.06, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setAttr(t.n2, "on-tap", "test20");
            direct_1.setStyle(t.n2, 61 /*display*/, 0);
            direct_1.setStyle(t.n2, 6 /*position*/, 1);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 65 /*opacity*/, 1);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createInput();
            direct_1.setAttr(t.n3, "on-change", "changeCall(e)");
            direct_1.setAttr(t.n3, "id", "debug");
            direct_1.setAttr(t.n3, "value", "第三方");
            direct_1.setAttr(t.n3, "type", "text");
            direct_1.setAttr(t.n3, "placeholder", "DDDD");
            direct_1.setStyle(t.n3, 42 /*fontSize*/, 18);
            direct_1.setStyle(t.n3, 30 /*color*/, staticObj[1]);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setClass(t.n3, 3421068014);
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
            let {} = w._$info;
            direct_1.extendAttr(t.n2, w);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});