_$pi.define("app_b/widget/open_treasure/open_treasure.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s3() {
            let w = this.w;
            return w.showTip ? B3 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setEvent(t.n2, "pointerclick", $event => {
                let r = w.boxOpenAnime($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.extendAttr(t.n2, w, 2848432957, true);
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
            direct_1.extendAttr(t.n2, w, 2848432957);
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
            direct_1.setAttr(t.n3, "e-show", "tip");
            direct_1.setClass(t.n3, 3569424899);
            t.n4 = direct_1.createSpan();
            direct_1.setClass(t.n4, 1621317832);
            direct_1.setText(t.n4, "点击屏幕任意位置退出");
            return this.n3;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.insertBefore(t.n3, target, anchor);
        }
        p() {
            return this.n3;
        }
    }
});