_$pi.define("app_c/demo/ui/show/base/transform.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 700], [0, 400], [0, 200], [0, 80], [0, 0.51, 0.58, 1], [{
        t: "r",
        d: 60
    }], [[1, 0], [1, 1]], [0, 100], [0.27, 0.96, 0.09, 1], [{
        t: "s",
        d: [0.6, 0.6]
    }], [0, 0.23, 0.58, 1], [0.95, 0.25, 0.08, 1], [[1, 1], [1, 1]], [0, 50], [{
        t: "r",
        d: 10
    }], [0.74, 0.2, 0.95, 1], [{
        t: "t",
        d: [[0, 30], [0, 30]]
    }], [{
        t: "t",
        d: [[0, 20], [0, 20]]
    }], [0.95, 0.2, 0.51, 1], [{
        t: "t",
        d: [[1, 0.5], [1, 0.5]]
    }], [[0, 30], null, null, null, null], [{
        t: "r",
        d: 30
    }], [{
        t: "rx",
        d: 30
    }], [{
        t: "ry",
        d: 30
    }], [{
        t: "sx",
        d: 30
    }], [{
        t: "sy",
        d: 30
    }]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 61 /*display*/, 0);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n2, 21 /*flexDirection*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n3, 65 /*opacity*/, 0.6);
            t.n4 = direct_1.createDiv();
            direct_1.setStyle(t.n4, 6 /*position*/, 1);
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n4, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n4, 58 /*transform*/, staticObj[5]);
            direct_1.setStyle(t.n4, 59 /*transformOrigin*/, staticObj[6]);
            t.n5 = direct_1.createDiv();
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n5, 46 /*backgroundColor*/, staticObj[8]);
            direct_1.setStyle(t.n5, 58 /*transform*/, staticObj[9]);
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 61 /*display*/, 0);
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n6, 46 /*backgroundColor*/, staticObj[10]);
            direct_1.setStyle(t.n6, 58 /*transform*/, staticObj[9]);
            t.n7 = direct_1.createDiv();
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n7, 46 /*backgroundColor*/, staticObj[11]);
            direct_1.setStyle(t.n7, 65 /*opacity*/, 0.6);
            t.n8 = direct_1.createDiv();
            direct_1.setStyle(t.n8, 6 /*position*/, 1);
            direct_1.setStyle(t.n8, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n8, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n8, 46 /*backgroundColor*/, staticObj[11]);
            direct_1.setStyle(t.n8, 58 /*transform*/, staticObj[5]);
            direct_1.setStyle(t.n8, 59 /*transformOrigin*/, staticObj[12]);
            t.n9 = direct_1.createDiv();
            direct_1.setStyle(t.n9, 6 /*position*/, 1);
            direct_1.setStyle(t.n9, 10 /*left*/, staticObj[13]);
            direct_1.setStyle(t.n9, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n9, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n9, 46 /*backgroundColor*/, staticObj[10]);
            direct_1.setStyle(t.n9, 58 /*transform*/, staticObj[14]);
            t.n10 = direct_1.createDiv();
            direct_1.setStyle(t.n10, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n10, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n10, 46 /*backgroundColor*/, staticObj[15]);
            direct_1.setStyle(t.n10, 58 /*transform*/, staticObj[16]);
            t.n11 = direct_1.createDiv();
            direct_1.setStyle(t.n11, 61 /*display*/, 0);
            direct_1.setStyle(t.n11, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n11, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n11, 46 /*backgroundColor*/, staticObj[10]);
            direct_1.setStyle(t.n11, 58 /*transform*/, staticObj[17]);
            t.n12 = direct_1.createDiv();
            direct_1.setStyle(t.n12, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n12, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n12, 46 /*backgroundColor*/, staticObj[18]);
            direct_1.setStyle(t.n12, 58 /*transform*/, staticObj[19]);
            t.n13 = direct_1.createDiv();
            direct_1.setStyle(t.n13, 61 /*display*/, 0);
            direct_1.setStyle(t.n13, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n13, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n13, 46 /*backgroundColor*/, staticObj[10]);
            direct_1.setStyle(t.n13, 58 /*transform*/, staticObj[19]);
            t.n14 = direct_1.createDiv();
            direct_1.setStyle(t.n14, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n14, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n14, 46 /*backgroundColor*/, staticObj[18]);
            direct_1.setStyle(t.n14, 16 /*margin*/, staticObj[20]);
            t.n15 = direct_1.createDiv();
            direct_1.setStyle(t.n15, 61 /*display*/, 0);
            direct_1.setStyle(t.n15, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n15, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n15, 46 /*backgroundColor*/, staticObj[10]);
            direct_1.setStyle(t.n15, 58 /*transform*/, staticObj[21]);
            t.n16 = direct_1.createDiv();
            direct_1.setStyle(t.n16, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n16, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n16, 46 /*backgroundColor*/, staticObj[18]);
            direct_1.setStyle(t.n16, 16 /*margin*/, staticObj[20]);
            t.n17 = direct_1.createDiv();
            direct_1.setStyle(t.n17, 61 /*display*/, 0);
            direct_1.setStyle(t.n17, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n17, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n17, 46 /*backgroundColor*/, staticObj[10]);
            direct_1.setStyle(t.n17, 58 /*transform*/, staticObj[22]);
            t.n18 = direct_1.createDiv();
            direct_1.setStyle(t.n18, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n18, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n18, 46 /*backgroundColor*/, staticObj[18]);
            direct_1.setStyle(t.n18, 16 /*margin*/, staticObj[20]);
            t.n19 = direct_1.createDiv();
            direct_1.setStyle(t.n19, 61 /*display*/, 0);
            direct_1.setStyle(t.n19, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n19, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n19, 46 /*backgroundColor*/, staticObj[10]);
            direct_1.setStyle(t.n19, 58 /*transform*/, staticObj[23]);
            t.n20 = direct_1.createDiv();
            direct_1.setStyle(t.n20, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n20, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n20, 46 /*backgroundColor*/, staticObj[18]);
            direct_1.setStyle(t.n20, 16 /*margin*/, staticObj[20]);
            t.n21 = direct_1.createDiv();
            direct_1.setStyle(t.n21, 61 /*display*/, 0);
            direct_1.setStyle(t.n21, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n21, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n21, 46 /*backgroundColor*/, staticObj[10]);
            direct_1.setStyle(t.n21, 58 /*transform*/, staticObj[24]);
            t.n22 = direct_1.createDiv();
            direct_1.setStyle(t.n22, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n22, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n22, 46 /*backgroundColor*/, staticObj[18]);
            direct_1.setStyle(t.n22, 16 /*margin*/, staticObj[20]);
            t.n23 = direct_1.createDiv();
            direct_1.setStyle(t.n23, 61 /*display*/, 0);
            direct_1.setStyle(t.n23, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n23, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n23, 46 /*backgroundColor*/, staticObj[10]);
            direct_1.setStyle(t.n23, 58 /*transform*/, staticObj[25]);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n6, t.n5);
            direct_1.append(t.n5, t.n2);
            direct_1.append(t.n9, t.n8);
            direct_1.append(t.n8, t.n7);
            direct_1.append(t.n7, t.n2);
            direct_1.append(t.n11, t.n10);
            direct_1.append(t.n10, t.n2);
            direct_1.append(t.n13, t.n12);
            direct_1.append(t.n12, t.n2);
            direct_1.append(t.n15, t.n14);
            direct_1.append(t.n14, t.n2);
            direct_1.append(t.n17, t.n16);
            direct_1.append(t.n16, t.n2);
            direct_1.append(t.n19, t.n18);
            direct_1.append(t.n18, t.n2);
            direct_1.append(t.n21, t.n20);
            direct_1.append(t.n20, t.n2);
            direct_1.append(t.n23, t.n22);
            direct_1.append(t.n22, t.n2);
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