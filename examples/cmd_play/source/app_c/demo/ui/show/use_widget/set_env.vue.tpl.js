_$pi.define("app_c/demo/ui/show/use_widget/set_env.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 0, 0, 1], [1, 1], [1, 0, 0, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s3() {
            let w = this.w;
            return w.env === 1 ? B3 : null;
        }

        s4() {
            let w = this.w;
            return w.env === -1 ? B4 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 46 /*backgroundColor*/, staticObj[0]);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[1]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.i3 = t.s3();
            t.n3 = direct_1.createIf(w, t.i3);
            t.i4 = t.s4();
            t.n4 = direct_1.createIf(w, t.i4);
            t.n5 = direct_1.createSpan();
            direct_1.setStyle(t.n5, 46 /*backgroundColor*/, staticObj[2]);
            direct_1.setEvent(t.n5, "pointerclick", $event => {
                let r = w.change($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n5, 452038505);
            direct_1.setText(t.n5, "切换全局环境");
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
            direct_1.destroyContext(t.n4);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n3.m(t.n2);
            t.n4.m(t.n2);
            direct_1.append(t.n5, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            t.n3 = direct_1.patchIf(w, t.n3, t.i3, t.i3 = t.s3(), t.n2);
            t.n4 = direct_1.patchIf(w, t.n4, t.i4, t.i4 = t.s4(), t.n2);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B4 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n4 = direct_1.createSpan();
            direct_1.setClass(t.n4, 452038505);
            direct_1.setText(t.n4, "全局环境为-1");
            return this.n4;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n4, target, anchor);
        }
        p() {
            return this.n4;
        }
    }
    class B3 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n3 = direct_1.createSpan();
            direct_1.setClass(t.n3, 452038505);
            direct_1.setText(t.n3, "全局环境为1");
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