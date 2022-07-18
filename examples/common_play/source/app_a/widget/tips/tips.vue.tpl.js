_$pi.define("app_a/widget/tips/tips.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[{
        duration: 2000,
        timingFunction: "linear",
        delayTime: 0,
        iteration: 1,
        direction: "normal",
        fillMode: "none",
        name: "popAnim"
    }]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s3() {
            let w = this.w;
            return w.text ? B3 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 70 /*animation*/, direct_1.createRunTimeAnimation(staticObj[0], w));
            direct_1.extendAttr(t.n2, w, 1239800721, true);
            t.i3 = t.s3();
            t.n3 = direct_1.createIf(w, t.i3);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n3.m(t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 1239800721);
            t.n3 = direct_1.patchIf(w, t.n3, t.i3, t.i3 = t.s3(), t.n2);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B3 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n3 = direct_1.createDiv();
            direct_1.setClass(t.n3, 3788844983);
            t.n4 = direct_1.createDiv();
            direct_1.setClass(t.n4, 3352417400);
            t.n5 = direct_1.createSpan();
            direct_1.setClass(t.n5, 2857884504);
            direct_1.setText(t.n5, w.text);
            return this.n3;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n5, t.n4);
            direct_1.append(t.n4, t.n3);
            direct_1.insertBefore(t.n3, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            if (dirty0 & 1) direct_1.setText(t.n5, w.text);
            return this.n3;
        }
    }
});