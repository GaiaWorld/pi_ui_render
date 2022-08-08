_$pi.define("app_c/demo/ui/show/base/mask_img.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 200], [[0, 10], null, null, null, null], [1, 0, 0, 1], [0, 100], [0, 300], [0, 1, 0, 1], [0, 30], [0, -30]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n3, 47 /*background*/, "linear-gradient(0deg,#ff0000,#00ff00)");
            direct_1.setStyle(t.n3, 77 /*maskImageSource*/, "app_c/demo/ui/images/mask.a.jpg");
            t.n4 = direct_1.createDiv();
            direct_1.setStyle(t.n4, 16 /*margin*/, staticObj[1]);
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n4, 47 /*background*/, "linear-gradient(0deg,#ff0000,#00ff00)");
            direct_1.setStyle(t.n4, 77 /*maskImageSource*/, "linear-gradient(90deg, #000000, #777777 50%, #ffffff)");
            t.n5 = direct_1.createDiv();
            direct_1.setStyle(t.n5, 16 /*margin*/, staticObj[1]);
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n5, 47 /*background*/, "linear-gradient(0deg,#ff0000,#00ff00)");
            direct_1.setStyle(t.n5, 77 /*maskImageSource*/, "linear-gradient(90deg, #ffffff, #777777 50%, #000000)");
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n6, 46 /*backgroundColor*/, staticObj[2]);
            direct_1.setStyle(t.n6, 10 /*left*/, staticObj[3]);
            direct_1.setStyle(t.n6, 65 /*opacity*/, 0.5);
            t.n7 = direct_1.createDiv();
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[4]);
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n7, 46 /*backgroundColor*/, staticObj[5]);
            direct_1.setStyle(t.n7, 6 /*position*/, 1);
            direct_1.setStyle(t.n7, 7 /*top*/, staticObj[6]);
            direct_1.setStyle(t.n7, 10 /*left*/, staticObj[7]);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n4, t.n2);
            direct_1.append(t.n5, t.n2);
            direct_1.append(t.n7, t.n6);
            direct_1.append(t.n6, t.n2);
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