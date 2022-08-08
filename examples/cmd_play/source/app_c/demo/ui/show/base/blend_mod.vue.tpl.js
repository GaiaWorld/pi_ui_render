_$pi.define("app_c/demo/ui/show/base/blend_mod.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 1024], [0, 600], [0, 1920], [0, -700], [0, -200], [0, 510], [0, 480], [0, 450]];
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
            direct_1.setStyle(t.n3, 6 /*position*/, 1);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n3, 66 /*overflow*/, true);
            t.n4 = direct_1.createDiv();
            direct_1.setStyle(t.n4, 6 /*position*/, 1);
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n4, 48 /*backgroundImage*/, "app_c/demo/ui/images/chouka_bg.png");
            direct_1.setStyle(t.n4, 29 /*justifyContent*/, 2);
            direct_1.setStyle(t.n4, 28 /*alignItems*/, 2);
            direct_1.setStyle(t.n4, 7 /*top*/, staticObj[3]);
            direct_1.setStyle(t.n4, 10 /*left*/, staticObj[4]);
            t.n5 = direct_1.createDiv();
            direct_1.setStyle(t.n5, 6 /*position*/, 1);
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[5]);
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[6]);
            direct_1.setStyle(t.n5, 48 /*backgroundImage*/, "app_c/demo/ui/images/chouka_shitou_1.png");
            direct_1.setStyle(t.n5, 26 /*alignSelf*/, 3);
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 6 /*position*/, 1);
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n6, 48 /*backgroundImage*/, "app_c/demo/ui/images/6.png");
            direct_1.setStyle(t.n6, 79 /*blendMode*/, 1);
            direct_1.setStyle(t.n6, 26 /*alignSelf*/, 3);
            t.n7 = direct_1.createDiv();
            direct_1.setStyle(t.n7, 6 /*position*/, 1);
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n7, 66 /*overflow*/, true);
            direct_1.setStyle(t.n7, 7 /*top*/, staticObj[1]);
            t.n8 = direct_1.createDiv();
            direct_1.setStyle(t.n8, 6 /*position*/, 1);
            direct_1.setStyle(t.n8, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n8, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n8, 48 /*backgroundImage*/, "app_c/demo/ui/images/chouka_bg.png");
            direct_1.setStyle(t.n8, 29 /*justifyContent*/, 2);
            direct_1.setStyle(t.n8, 28 /*alignItems*/, 2);
            direct_1.setStyle(t.n8, 7 /*top*/, staticObj[3]);
            direct_1.setStyle(t.n8, 10 /*left*/, staticObj[4]);
            t.n9 = direct_1.createDiv();
            direct_1.setStyle(t.n9, 6 /*position*/, 1);
            direct_1.setStyle(t.n9, 0 /*width*/, staticObj[5]);
            direct_1.setStyle(t.n9, 1 /*height*/, staticObj[6]);
            direct_1.setStyle(t.n9, 48 /*backgroundImage*/, "app_c/demo/ui/images/chouka_shitou_1.png");
            direct_1.setStyle(t.n9, 26 /*alignSelf*/, 3);
            t.n10 = direct_1.createDiv();
            direct_1.setStyle(t.n10, 6 /*position*/, 1);
            direct_1.setStyle(t.n10, 0 /*width*/, staticObj[7]);
            direct_1.setStyle(t.n10, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n10, 48 /*backgroundImage*/, "app_c/demo/ui/images/6.png");
            direct_1.setStyle(t.n10, 79 /*blendMode*/, 0);
            direct_1.setStyle(t.n10, 26 /*alignSelf*/, 3);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n5, t.n4);
            direct_1.append(t.n6, t.n4);
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n9, t.n8);
            direct_1.append(t.n10, t.n8);
            direct_1.append(t.n8, t.n7);
            direct_1.append(t.n7, t.n2);
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