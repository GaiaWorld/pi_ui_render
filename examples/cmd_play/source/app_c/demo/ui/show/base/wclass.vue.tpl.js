_$pi.define("app_c/demo/ui/show/base/wclass.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 700], [0, 400], [0, 50], [0, 100]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 61 /*display*/, 0);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 61 /*display*/, 0);
            direct_1.setStyle(t.n3, 6 /*position*/, 1);
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n3, 21 /*flexDirection*/, 1);
            direct_1.setClass(t.n3, 236057231);
            t.n4 = direct_1.createFor(w, w.list, B4);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n4);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n4.m(t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            direct_1.patchFor(w, t.n4, w.list, B4);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B4 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            t.n4 = direct_1.createSpan();
            direct_1.setAttr(t.n4, "key", i);
            direct_1.setEvent(t.n4, "pointerclick", $event => {
                let [v, i] = t._$eventsCtx;
                let r = w.tap($event, i);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n4, [112024820, 1066944960, 2007788919]);
            direct_1.setText(t.n4, v);
            return this.n4;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n4, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            let { dirty0 } = w._$info;
            direct_1.setAttr(t.n4, "key", i);
            direct_1.setText(t.n4, v);
            return this.n4;
        }
        s(v, i) {
            let w = this.w;
            this._$ctx[0] = v;
            this._$ctx[1] = i;
            if (!this._$eventsCtx) {
                this._$eventsCtx = [];
            }
            this._$eventsCtx[1] = this._$ctx[1];
        }
        getKey(v, i) {
            return i;
        }
    }
});