_$pi.define("app_c/demo/ui/menu/treemenu.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "./treemenu.vue"], function (require, exports, module, direct_1, treemenu_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 10]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s3() {
            let w = this.w;
            return w.tree.show.cfg ? B3 : null;
        }

        s4() {
            let w = this.w;
            return w.tree.arr && w.tree.show.select ? B4 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 21 /*flexDirection*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.i3 = t.s3();
            t.n3 = direct_1.createIf(w, t.i3);
            t.i4 = t.s4();
            t.n4 = direct_1.createIf(w, t.i4);
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
            t.n4 = direct_1.createDiv();
            direct_1.setStyle(t.n4, 15 /*paddingLeft*/, staticObj[0]);
            direct_1.setStyle(t.n4, 21 /*flexDirection*/, 1);
            direct_1.setClass(t.n4, 485797935);
            t.n5 = direct_1.createFor(w, w.tree.arr, B5);
            return this.n4;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n5);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n5.m(t.n4);
            direct_1.insertBefore(t.n4, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.patchFor(w, t.n5, w.tree.arr, B5);
            return this.n4;
        }
    }
    class B5 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            t.n5 = direct_1.createWidget(treemenu_vue_1.default, w, {
                attrs: {
                    key: i,
                    tree: w.tree.arr[i],
                    btnWidget: w.btnWidget
                }
            });
            return this.n5;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n5);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n5, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            let { dirty0 } = w._$info;
            var _$wAttrs = {};
            {
                const _$attrs = {};
                _$attrs.key = i;
                _$attrs.tree = w.tree.arr[i];
                if (dirty0 & 2) _$attrs.btnWidget = w.btnWidget;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n5, _$wAttrs);
            return this.n5;
        }
        s(v, i) {
            let w = this.w;
            this._$ctx[0] = v;
            this._$ctx[1] = i;
        }
        getKey(v, i) {
            return i;
        }
    }
    class B3 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n3 = direct_1.createComponent(w, {
                attrs: {
                    is: w.btnWidget,
                    text: w.tree.show.cfg.text,
                    sid: w.tree.show.sid,
                    leaf: w.tree.show.leaf,
                    select: w.tree.show.select
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.click();
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
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
            direct_1.mountComponent(t.n3, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            var _$wAttrs = {};
            if (dirty0 & 3) {
                const _$attrs = {};
                if (dirty0 & 2) _$attrs.is = w.btnWidget;
                if (dirty0 & 1) _$attrs.text = w.tree.show.cfg.text;
                if (dirty0 & 1) _$attrs.sid = w.tree.show.sid;
                if (dirty0 & 1) _$attrs.leaf = w.tree.show.leaf;
                if (dirty0 & 1) _$attrs.select = w.tree.show.select;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            t.n3 = direct_1.patchAttrsForComponent(t.n3, _$wAttrs);
            return this.n3;
        }
    }
});