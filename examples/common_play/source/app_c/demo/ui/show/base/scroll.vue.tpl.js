_$pi.define("app_c/demo/ui/show/base/scroll.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 700], [0, 400], [0, 500], [0, 300], [0.66, 0.97, 0.95, 1], [1, 1], [2, 0], [0.92, 0.66, 0.97, 1], [0, 200], [0, 120], [0.97, 0.76, 0.08, 1], [0.78, 0.97, 0.08, 1], [0.08, 0.97, 0.08, 1], [0.08, 0.97, 0.82, 1], [0.08, 0.36, 0.97, 1], [0.54, 0.08, 0.97, 1], [0.97, 0.14, 0.08, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[1]);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setAttr(t.n3, "scroll_path", "y");
            direct_1.setAttr(t.n3, "scroll_type", "none");
            direct_1.setStyle(t.n3, 6 /*position*/, 1);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n3, 66 /*overflow*/, true);
            t.n4 = direct_1.createDiv();
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[5]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[6]);
            direct_1.setStyle(t.n4, 46 /*backgroundColor*/, staticObj[7]);
            direct_1.setStyle(t.n4, 21 /*flexDirection*/, 0);
            direct_1.setStyle(t.n4, 22 /*flexWrap*/, 1);
            t.n5 = direct_1.createDiv();
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n5, 46 /*backgroundColor*/, staticObj[10]);
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n6, 46 /*backgroundColor*/, staticObj[11]);
            t.n7 = direct_1.createDiv();
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n7, 46 /*backgroundColor*/, staticObj[12]);
            t.n8 = direct_1.createDiv();
            direct_1.setStyle(t.n8, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n8, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n8, 46 /*backgroundColor*/, staticObj[13]);
            t.n9 = direct_1.createDiv();
            direct_1.setStyle(t.n9, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n9, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n9, 46 /*backgroundColor*/, staticObj[14]);
            t.n10 = direct_1.createDiv();
            direct_1.setStyle(t.n10, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n10, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n10, 46 /*backgroundColor*/, staticObj[15]);
            t.n11 = direct_1.createDiv();
            direct_1.setStyle(t.n11, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n11, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n11, 46 /*backgroundColor*/, staticObj[16]);
            t.n12 = direct_1.createDiv();
            direct_1.setStyle(t.n12, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n12, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n12, 46 /*backgroundColor*/, staticObj[10]);
            t.n13 = direct_1.createDiv();
            direct_1.setStyle(t.n13, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n13, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n13, 46 /*backgroundColor*/, staticObj[11]);
            t.n14 = direct_1.createDiv();
            direct_1.setStyle(t.n14, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n14, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n14, 46 /*backgroundColor*/, staticObj[12]);
            t.n15 = direct_1.createDiv();
            direct_1.setStyle(t.n15, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n15, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n15, 46 /*backgroundColor*/, staticObj[13]);
            t.n16 = direct_1.createDiv();
            direct_1.setStyle(t.n16, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n16, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n16, 46 /*backgroundColor*/, staticObj[14]);
            t.n17 = direct_1.createDiv();
            direct_1.setStyle(t.n17, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n17, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n17, 46 /*backgroundColor*/, staticObj[15]);
            t.n18 = direct_1.createDiv();
            direct_1.setStyle(t.n18, 0 /*width*/, staticObj[8]);
            direct_1.setStyle(t.n18, 1 /*height*/, staticObj[9]);
            direct_1.setStyle(t.n18, 46 /*backgroundColor*/, staticObj[16]);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n5, t.n4);
            direct_1.append(t.n6, t.n4);
            direct_1.append(t.n7, t.n4);
            direct_1.append(t.n8, t.n4);
            direct_1.append(t.n9, t.n4);
            direct_1.append(t.n10, t.n4);
            direct_1.append(t.n11, t.n4);
            direct_1.append(t.n12, t.n4);
            direct_1.append(t.n13, t.n4);
            direct_1.append(t.n14, t.n4);
            direct_1.append(t.n15, t.n4);
            direct_1.append(t.n16, t.n4);
            direct_1.append(t.n17, t.n4);
            direct_1.append(t.n18, t.n4);
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