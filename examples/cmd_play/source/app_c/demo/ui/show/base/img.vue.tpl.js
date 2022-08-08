_$pi.define("app_c/demo/ui/show/base/img.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 400], [0, 600], [0, 100], [0.84, 0.11, 0.06, 1], [0, 0], [0.06, 0.84, 0.38, 1], [0, 200], [0.84, 0.06, 0.73, 1], [0, 90]];
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
            direct_1.setStyle(t.n2, 66 /*overflow*/, true);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 61 /*display*/, 0);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[3]);
            direct_1.setStyle(t.n3, 65 /*opacity*/, 0.6);
            t.n4 = direct_1.createDiv();
            direct_1.setStyle(t.n4, 61 /*display*/, 0);
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n4, 46 /*backgroundColor*/, staticObj[3]);
            direct_1.setStyle(t.n4, 65 /*opacity*/, 0.6);
            t.n5 = direct_1.createImg();
            direct_1.setAttr(t.n5, "src", "app_c/demo/ui/images/01.png");
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[2]);
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 6 /*position*/, 1);
            direct_1.setStyle(t.n6, 10 /*left*/, staticObj[4]);
            direct_1.setStyle(t.n6, 61 /*display*/, 0);
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n6, 46 /*backgroundColor*/, staticObj[5]);
            direct_1.setStyle(t.n6, 65 /*opacity*/, 0.6);
            t.n7 = direct_1.createImg();
            direct_1.setAttr(t.n7, "src", "app_c/demo/ui/images/01.png");
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[2]);
            t.n8 = direct_1.createDiv();
            direct_1.setStyle(t.n8, 6 /*position*/, 1);
            direct_1.setStyle(t.n8, 10 /*left*/, staticObj[2]);
            direct_1.setStyle(t.n8, 61 /*display*/, 0);
            direct_1.setStyle(t.n8, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n8, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n8, 65 /*opacity*/, 0.6);
            t.n9 = direct_1.createImg();
            direct_1.setAttr(t.n9, "src", "app_c/demo/ui/images/01.png");
            direct_1.setStyle(t.n9, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n9, 1 /*height*/, staticObj[2]);
            t.n10 = direct_1.createDiv();
            direct_1.setStyle(t.n10, 6 /*position*/, 1);
            direct_1.setStyle(t.n10, 10 /*left*/, staticObj[6]);
            direct_1.setStyle(t.n10, 61 /*display*/, 0);
            direct_1.setStyle(t.n10, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n10, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n10, 46 /*backgroundColor*/, staticObj[7]);
            direct_1.setStyle(t.n10, 65 /*opacity*/, 0.6);
            t.n11 = direct_1.createImg();
            direct_1.setAttr(t.n11, "src", "app_c/demo/ui/images/01.png");
            direct_1.setStyle(t.n11, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n11, 6 /*position*/, 1);
            direct_1.setStyle(t.n11, 1 /*height*/, staticObj[2]);
            t.n12 = direct_1.createDiv();
            direct_1.setStyle(t.n12, 61 /*display*/, 0);
            direct_1.setStyle(t.n12, 6 /*position*/, 1);
            direct_1.setStyle(t.n12, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n12, 1 /*height*/, staticObj[2]);
            t.n13 = direct_1.createDiv();
            direct_1.setStyle(t.n13, 61 /*display*/, 0);
            direct_1.setStyle(t.n13, 6 /*position*/, 1);
            direct_1.setStyle(t.n13, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n13, 1 /*height*/, staticObj[2]);
            t.n14 = direct_1.createImg();
            direct_1.setAttr(t.n14, "src", "app_c/demo/ui/images/01.png");
            direct_1.setStyle(t.n14, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n14, 1 /*height*/, staticObj[8]);
            t.n15 = direct_1.createImg();
            direct_1.setAttr(t.n15, "src", "app_c/demo/ui/images/01.png");
            direct_1.setStyle(t.n15, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n15, 1 /*height*/, staticObj[8]);
            t.n16 = direct_1.createImg();
            direct_1.setAttr(t.n16, "src", "app_c/demo/ui/images/01.png");
            direct_1.setStyle(t.n16, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n16, 1 /*height*/, staticObj[8]);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n5, t.n4);
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n7, t.n6);
            direct_1.append(t.n6, t.n2);
            direct_1.append(t.n9, t.n8);
            direct_1.append(t.n8, t.n2);
            direct_1.append(t.n11, t.n10);
            direct_1.append(t.n12, t.n10);
            direct_1.append(t.n13, t.n10);
            direct_1.append(t.n10, t.n2);
            direct_1.append(t.n14, t.n2);
            direct_1.append(t.n15, t.n2);
            direct_1.append(t.n16, t.n2);
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