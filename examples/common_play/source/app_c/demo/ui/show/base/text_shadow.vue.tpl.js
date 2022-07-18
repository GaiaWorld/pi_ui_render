_$pi.define("app_c/demo/ui/show/base/text_shadow.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [1, 1, 1, 1], [0, 0, 0, 1], [0, 0, 0, 0], [0, 0, 0, 0.3]];
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
            t.n3 = direct_1.createSpan();
            direct_1.setStyle(t.n3, 42 /*fontSize*/, 100);
            direct_1.setStyle(t.n3, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n3, 30 /*color*/, staticObj[2]);
            direct_1.setStyle(t.n3, 36 /*textShadow*/, "rgb(255,0,0) 0px 0px 5px");
            direct_1.setText(t.n3, "测试一下");
            t.n4 = direct_1.createSpan();
            direct_1.setStyle(t.n4, 42 /*fontSize*/, 100);
            direct_1.setStyle(t.n4, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n4, 30 /*color*/, staticObj[2]);
            direct_1.setStyle(t.n4, 36 /*textShadow*/, "rgb(255,0,0) 0px 0px 5px,rgb(255,0,0) 0px 0px 3px, rgb(255,255,255) 0px 0px 1px");
            direct_1.setText(t.n4, "测试一下");
            t.n5 = direct_1.createSpan();
            direct_1.setStyle(t.n5, 42 /*fontSize*/, 100);
            direct_1.setStyle(t.n5, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n5, 30 /*color*/, staticObj[2]);
            direct_1.setClass(t.n5, 2677724671);
            direct_1.setText(t.n5, "测试一下");
            t.n6 = direct_1.createSpan();
            direct_1.setStyle(t.n6, 42 /*fontSize*/, 100);
            direct_1.setStyle(t.n6, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n6, 36 /*textShadow*/, "rgb(190,136,83) 0px 0px 5px");
            direct_1.setStyle(t.n6, 30 /*color*/, staticObj[3]);
            direct_1.setClass(t.n6, 2677724671);
            direct_1.setText(t.n6, "测试一下");
            t.n7 = direct_1.createDiv();
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n7, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n7, 6 /*position*/, 1);
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n4, t.n2);
            direct_1.append(t.n5, t.n2);
            direct_1.append(t.n6, t.n2);
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