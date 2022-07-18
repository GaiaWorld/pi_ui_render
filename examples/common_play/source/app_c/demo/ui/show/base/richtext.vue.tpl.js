_$pi.define("app_c/demo/ui/show/base/richtext.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [0.07, 0.96, 0.89, 1], [0, 80], [0.97, 0.48, 0.48, 1], [0, 48], [0, 24], [0.88, 0.2, 0.2, 1], [0, 1], [0.91, 0.97, 0.96, 1], [0, 120], [0, 20], [0.39, 0.88, 0.2, 1], [0, 160], [0, 30]];
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
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 46 /*backgroundColor*/, staticObj[1]);
            direct_1.setStyle(t.n2, 21 /*flexDirection*/, 0);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n3, 28 /*alignItems*/, 2);
            direct_1.setStyle(t.n3, 29 /*justifyContent*/, 2);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[3]);
            t.n4 = direct_1.createImg();
            direct_1.setAttr(t.n4, "src", "app_c/demo/ui/images/01.png");
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[4]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[4]);
            t.n5 = direct_1.createSpan();
            direct_1.setStyle(t.n5, 36 /*textShadow*/, "rgb(255,0,0) 0px 0px 1px");
            direct_1.setText(t.n5, "测试一下测试一下测试一下测试一下测试一下测试一下测试一下测试一下测试一下测试一下测试一下");
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[5]);
            direct_1.setStyle(t.n6, 66 /*overflow*/, true);
            direct_1.setStyle(t.n6, 46 /*backgroundColor*/, staticObj[6]);
            t.n7 = direct_1.createSpan();
            direct_1.setStyle(t.n7, 42 /*fontSize*/, 30);
            direct_1.setStyle(t.n7, 32 /*lineHeight*/, staticObj[5]);
            direct_1.setStyle(t.n7, 43 /*fontStyle*/, 1);
            direct_1.setStyle(t.n7, 44 /*fontWeight*/, 500);
            direct_1.setStyle(t.n7, 41 /*fontFamily*/, "HYXingKaiJ");
            direct_1.setStyle(t.n7, 31 /*letterSpacing*/, staticObj[7]);
            direct_1.setStyle(t.n7, 30 /*color*/, staticObj[8]);
            direct_1.setText(t.n7, "测试一下测试一下测试一下测试一下测试一");
            t.n8 = direct_1.createDiv();
            direct_1.setStyle(t.n8, 0 /*width*/, staticObj[9]);
            direct_1.setStyle(t.n8, 1 /*height*/, staticObj[10]);
            direct_1.setStyle(t.n8, 66 /*overflow*/, true);
            direct_1.setStyle(t.n8, 46 /*backgroundColor*/, staticObj[11]);
            t.n9 = direct_1.createSpan();
            direct_1.setStyle(t.n9, 42 /*fontSize*/, 24);
            direct_1.setStyle(t.n9, 32 /*lineHeight*/, staticObj[5]);
            direct_1.setStyle(t.n9, 44 /*fontWeight*/, 100);
            direct_1.setStyle(t.n9, 41 /*fontFamily*/, "HYXingKaiJ");
            direct_1.setStyle(t.n9, 39 /*whiteSpace*/, 1);
            direct_1.setStyle(t.n9, 31 /*letterSpacing*/, staticObj[7]);
            direct_1.setStyle(t.n9, 30 /*color*/, staticObj[8]);
            direct_1.setText(t.n9, "测试一下测试一下测试一下测试一下测试一下测试一");
            t.n10 = direct_1.createDiv();
            direct_1.setStyle(t.n10, 61 /*display*/, 0);
            direct_1.setStyle(t.n10, 0 /*width*/, staticObj[12]);
            direct_1.setStyle(t.n10, 1 /*height*/, staticObj[13]);
            direct_1.setStyle(t.n10, 66 /*overflow*/, true);
            direct_1.setStyle(t.n10, 46 /*backgroundColor*/, staticObj[11]);
            direct_1.setStyle(t.n10, 28 /*alignItems*/, 1);
            direct_1.setStyle(t.n10, 29 /*justifyContent*/, 1);
            direct_1.setStyle(t.n10, 21 /*flexDirection*/, 0);
            t.n11 = direct_1.createSpan();
            direct_1.setStyle(t.n11, 42 /*fontSize*/, 24);
            direct_1.setStyle(t.n11, 32 /*lineHeight*/, staticObj[5]);
            direct_1.setStyle(t.n11, 44 /*fontWeight*/, 900);
            direct_1.setStyle(t.n11, 41 /*fontFamily*/, "HYXingKaiJ");
            direct_1.setStyle(t.n11, 31 /*letterSpacing*/, staticObj[10]);
            direct_1.setText(t.n11, "测试一下");
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n5, t.n2);
            direct_1.append(t.n7, t.n6);
            direct_1.append(t.n6, t.n2);
            direct_1.append(t.n9, t.n8);
            direct_1.append(t.n8, t.n2);
            direct_1.append(t.n11, t.n10);
            direct_1.append(t.n10, t.n2);
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