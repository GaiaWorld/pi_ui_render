_$pi.define("app_c/demo/ui/show/base/borderimg.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 700], [0, 400], [0, 30], [0, 250], [0.2, 0.2, 0.4, 0.2, 1, 1], [null, [0, 50], [0, 50], [0, 120], [0, 50]]];
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
            direct_1.setStyle(t.n3, 7 /*top*/, staticObj[2]);
            direct_1.setStyle(t.n3, 10 /*left*/, staticObj[2]);
            direct_1.setStyle(t.n3, 6 /*position*/, 1);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[1]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n3, 54 /*borderImageSource*/, "app_c/demo/ui/images/03.png");
            direct_1.setStyle(t.n3, 55 /*borderImageSlice*/, staticObj[4]);
            direct_1.setStyle(t.n3, 51 /*borderWidth*/, staticObj[5]);
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