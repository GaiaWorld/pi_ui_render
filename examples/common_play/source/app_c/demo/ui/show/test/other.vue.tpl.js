_$pi.define("app_c/demo/ui/show/test/other.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "./other.vue"], function (require, exports, module, direct_1, other_vue_1) {
    "use strict";

    exports.BW6 = exports.BW2 = void 0;
    const staticObj = [[0, 0, 0, 1], [1, 1], [0, 100], [0, 40], [null, [0, 0], [0, 10], [0, 0], [0, 10]], [1, 1, 1, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 46 /*backgroundColor*/, staticObj[0]);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[1]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[1]);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createFor(w, w.count, B3);
            t.n4 = direct_1.createSpan();
            direct_1.setEvent(t.n4, "pointerclick", $event => {
                let r = w.change($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setClass(t.n4, 4288962427);
            direct_1.setText(t.n4, "更换内容");
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
            direct_1.append(t.n4, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            direct_1.patchFor(w, t.n3, w.count, B3);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B3 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [i] = t._$ctx;
            t.n3 = direct_1.createWidget(other_vue_1.Text, w, {
                attrs: {
                    text: i,
                    key: i
                },
                style: {
                    fontSize /*fontSize*/: "50px",
                    color /*color*/: "#fff"
                }
            });
            return this.n3;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [i] = t._$ctx;
            let { dirty0 } = w._$info;
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.text = i;
                _$attrs.key = i;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n3, _$wAttrs);
            return this.n3;
        }
        s(i) {
            let w = this.w;
            this._$ctx[0] = i;
        }
        getKey(i) {
            return i;
        }
    }
    class BW6 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n6, 16 /*margin*/, staticObj[4]);
            direct_1.extendAttr(t.n6, w, null, true);
            t.n7 = direct_1.createSpan();
            direct_1.setStyle(t.n7, 42 /*fontSize*/, 50);
            direct_1.setStyle(t.n7, 30 /*color*/, staticObj[5]);
            direct_1.setText(t.n7, w.text);
            return this.n6;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n7, t.n6);
            direct_1.insertBefore(t.n6, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n6, w);
            if (dirty0 & 1) direct_1.setText(t.n7, w.text);
            return this.n6;
        }
    }
    exports.BW6 = BW6;
});