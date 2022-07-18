_$pi.define("app_c/demo/ui/show/animation/tween.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0.25, 0.87, 0.67, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.extendAttr(t.n2, w, 566043874, true);
            t.n3 = direct_1.createSpan();
            direct_1.setEvent(t.n3, "pointerclick", $event => {
                let r = w.click(1);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n3, 2961905013);
            direct_1.setText(t.n3, "开始播放js");
            t.n4 = direct_1.createDiv();
            direct_1.setAttr(t.n4, "id", "box");
            direct_1.setStyle(t.n4, 46 /*backgroundColor*/, staticObj[0]);
            direct_1.setClass(t.n4, 1511651737);
            t.n5 = direct_1.createSpan();
            direct_1.setEvent(t.n5, "pointerclick", $event => {
                let r = w.click(2);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n5, 2961905013);
            direct_1.setText(t.n5, "开始播放css");
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 70 /*animation*/, direct_1.createRunTimeAnimation(w.playFg ? 'bigger 1s quadInTween 0 1 normal forwards' : '', w));
            direct_1.setClass(t.n6, 1511651737);
            t.n7 = direct_1.createDiv();
            direct_1.setStyle(t.n7, 70 /*animation*/, direct_1.createRunTimeAnimation(w.playFg ? 'bigger 1s quadOutTween 0 1 normal forwards' : '', w));
            direct_1.setClass(t.n7, 1511651737);
            t.n8 = direct_1.createDiv();
            direct_1.setStyle(t.n8, 70 /*animation*/, direct_1.createRunTimeAnimation(w.playFg ? 'bigger 1s cubicInTween 0 1 normal forwards' : '', w));
            direct_1.setClass(t.n8, 1511651737);
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
            direct_1.append(t.n8, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w, 566043874);
            if (dirty0 & 1) direct_1.setStyle(t.n6, 70 /*animation*/, direct_1.createRunTimeAnimation(w.playFg ? 'bigger 1s quadInTween 0 1 normal forwards' : '', w));
            if (dirty0 & 1) direct_1.setStyle(t.n7, 70 /*animation*/, direct_1.createRunTimeAnimation(w.playFg ? 'bigger 1s quadOutTween 0 1 normal forwards' : '', w));
            if (dirty0 & 1) direct_1.setStyle(t.n8, 70 /*animation*/, direct_1.createRunTimeAnimation(w.playFg ? 'bigger 1s cubicInTween 0 1 normal forwards' : '', w));
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});