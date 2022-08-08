_$pi.define("app_c/demo/ui/show/base/texteffect.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 700], [0, 400], [0, 1, 0.92, 1], [1, 1], [0, 20], [1, 1, 1, 1], [0, 0], [0, 24], [0, 1], [2, 1, 1, 1, 1]];
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
            direct_1.setStyle(t.n2, 46 /*backgroundColor*/, staticObj[2]);
            direct_1.setStyle(t.n2, 21 /*flexDirection*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 61 /*display*/, 0);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[3]);
            direct_1.setStyle(t.n3, 17 /*marginTop*/, staticObj[4]);
            t.n4 = direct_1.createSpan();
            direct_1.setStyle(t.n4, 43 /*fontStyle*/, 0);
            direct_1.setStyle(t.n4, 44 /*fontWeight*/, 200);
            direct_1.setStyle(t.n4, 30 /*color*/, staticObj[5]);
            direct_1.setStyle(t.n4, 42 /*fontSize*/, 32);
            direct_1.setStyle(t.n4, 31 /*letterSpacing*/, staticObj[6]);
            direct_1.setStyle(t.n4, 41 /*fontFamily*/, "kaijian");
            direct_1.setStyle(t.n4, 36 /*textShadow*/, "#19120d 1px 1px 2px");
            direct_1.setStyle(t.n4, 38 /*textGradient*/, "linear-gradient(90deg, #fffde7, #ffeeaa 45%, #cab79d)");
            direct_1.setText(t.n4, "我是渐变颜色+阴影");
            t.n5 = direct_1.createDiv();
            direct_1.setStyle(t.n5, 61 /*display*/, 0);
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[3]);
            direct_1.setStyle(t.n5, 17 /*marginTop*/, staticObj[4]);
            t.n6 = direct_1.createSpan();
            direct_1.setStyle(t.n6, 42 /*fontSize*/, 24);
            direct_1.setStyle(t.n6, 32 /*lineHeight*/, staticObj[7]);
            direct_1.setStyle(t.n6, 30 /*color*/, staticObj[5]);
            direct_1.setStyle(t.n6, 44 /*fontWeight*/, 400);
            direct_1.setStyle(t.n6, 31 /*letterSpacing*/, staticObj[8]);
            direct_1.setStyle(t.n6, 36 /*textShadow*/, "#19120d 0px 1px 2px");
            direct_1.setStyle(t.n6, 37 /*textStroke*/, staticObj[9]);
            direct_1.setStyle(t.n6, 38 /*textGradient*/, "linear-gradient(90deg, #fffde7, #ffeeaa 45%, #cab79d)");
            direct_1.setText(t.n6, "我是渐变颜色+描边");
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n6, t.n5);
            direct_1.append(t.n5, t.n2);
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