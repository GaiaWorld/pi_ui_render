_$pi.define("app_c/demo/ui/menu/treebtn.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 24], [0.85, 0.85, 0.85, 1], [0, 16], [0, 34], [1, 1, 1, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s3() {
            let w = this.w;
            return w.selectSid === w.sid ? B3 : null;
        }

        s5() {
            let w = this.w;
            return w.leaf ? B5 : w.select ? B6 : B7;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 28 /*alignItems*/, 2);
            direct_1.extendAttr(t.n2, w, null, true);
            t.i3 = t.s3();
            t.n3 = direct_1.createIf(w, t.i3);
            t.n4 = direct_1.createDiv();
            t.i5 = t.s5();
            t.n5 = direct_1.createIf(w, t.i5);
            t.n8 = direct_1.createDiv();
            direct_1.setStyle(t.n8, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n8, 23 /*flexGrow*/, 1);
            t.n9 = direct_1.createSpan();
            direct_1.setStyle(t.n9, 30 /*color*/, staticObj[4]);
            direct_1.setStyle(t.n9, 42 /*fontSize*/, 24);
            direct_1.setStyle(t.n9, 32 /*lineHeight*/, staticObj[3]);
            direct_1.setText(t.n9, w.text);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
            direct_1.destroyContext(t.n5);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n3.m(t.n2);
            t.n5.m(t.n4);
            direct_1.append(t.n9, t.n8);
            direct_1.append(t.n8, t.n4);
            direct_1.append(t.n4, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            t.n3 = direct_1.patchIf(w, t.n3, t.i3, t.i3 = t.s3(), t.n2);
            t.n5 = direct_1.patchIf(w, t.n5, t.i5, t.i5 = t.s5(), t.n4);
            if (dirty0 & 16) direct_1.setText(t.n9, w.text);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B7 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n7 = direct_1.createSpan();
            direct_1.setStyle(t.n7, 32 /*lineHeight*/, staticObj[0]);
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n7, 29 /*justifyContent*/, 2);
            direct_1.setText(t.n7, "+");
            return this.n7;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n7, target, anchor);
        }
        p() {
            return this.n7;
        }
    }
    class B6 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n6 = direct_1.createSpan();
            direct_1.setStyle(t.n6, 32 /*lineHeight*/, staticObj[0]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n6, 29 /*justifyContent*/, 2);
            direct_1.setText(t.n6, "-");
            return this.n6;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n6, target, anchor);
        }
        p() {
            return this.n6;
        }
    }
    class B5 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n5 = direct_1.createDiv();
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[0]);
            return this.n5;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n5, target, anchor);
        }
        p() {
            return this.n5;
        }
    }
    class B3 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[1]);
            direct_1.setStyle(t.n3, 6 /*position*/, 1);
            return this.n3;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n3, target, anchor);
        }
        p() {
            return this.n3;
        }
    }
});