_$pi.define("app_c/demo/ui/show/component/list_align.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_common/ui/list.vue"], function (require, exports, module, direct_1, list_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [0.78, 0.78, 0.78, 1], [0, 30], [0, 10], [0, 0, 0, 1], [null, [0, 0], [0, 10], [0, 0], [0, 10]], [0, 250], [1, 0, 0, 1], [0, 0, 1, 1], [0, 1, 0, 1], [0, 200], [0, 40]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n2, 46 /*backgroundColor*/, staticObj[1]);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createSpan();
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n3, 17 /*marginTop*/, staticObj[3]);
            direct_1.setStyle(t.n3, 30 /*color*/, staticObj[4]);
            direct_1.setText(t.n3, "scrollSnapAlign: center");
            t.ctx5 = [undefined];
            t.n4 = direct_1.createWidget(list_vue_1.default, w, {
                attrs: {
                    scrollSnapAlign: "center",
                    ref: "list",
                    data: w.arr,
                    direction: 1
                },
                style: {
                    backgroundColor /*backgroundColor*/: "rgb(0,255,0)",
                    width /*width*/: "100%",
                    height /*height*/: "200px",
                    marginBottom /*marginBottom*/: "40px"
                },
                slot: {
                    default: CreateSlot5
                },
                sCtx: {
                    default: t.ctx5
                },
                scope: w
            });
            t.n8 = direct_1.createSpan();
            direct_1.setStyle(t.n8, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n8, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n8, 17 /*marginTop*/, staticObj[3]);
            direct_1.setStyle(t.n8, 30 /*color*/, staticObj[4]);
            direct_1.setText(t.n8, "scrollSnapAlign: start");
            t.ctx10 = [undefined];
            t.n9 = direct_1.createWidget(list_vue_1.default, w, {
                attrs: {
                    scrollSnapAlign: "start",
                    ref: "list",
                    data: w.arr,
                    direction: 1
                },
                style: {
                    backgroundColor /*backgroundColor*/: "rgb(0,255,0)",
                    width /*width*/: "100%",
                    height /*height*/: "200px",
                    marginBottom /*marginBottom*/: "40px"
                },
                slot: {
                    default: CreateSlot10
                },
                sCtx: {
                    default: t.ctx10
                },
                scope: w
            });
            t.n13 = direct_1.createSpan();
            direct_1.setStyle(t.n13, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n13, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n13, 17 /*marginTop*/, staticObj[3]);
            direct_1.setStyle(t.n13, 30 /*color*/, staticObj[4]);
            direct_1.setText(t.n13, "scrollSnapAlign: end");
            t.ctx15 = [undefined];
            t.n14 = direct_1.createWidget(list_vue_1.default, w, {
                attrs: {
                    scrollSnapAlign: "end",
                    ref: "list",
                    data: w.arr,
                    direction: 1
                },
                style: {
                    backgroundColor /*backgroundColor*/: "rgb(0,255,0)",
                    width /*width*/: "100%",
                    height /*height*/: "200px",
                    marginBottom /*marginBottom*/: "40px"
                },
                slot: {
                    default: CreateSlot15
                },
                sCtx: {
                    default: t.ctx15
                },
                scope: w
            });
            t.n18 = direct_1.createSpan();
            direct_1.setStyle(t.n18, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n18, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n18, 17 /*marginTop*/, staticObj[3]);
            direct_1.setStyle(t.n18, 30 /*color*/, staticObj[4]);
            direct_1.setText(t.n18, "mask");
            t.n19 = direct_1.createDiv();
            direct_1.setStyle(t.n19, 46 /*backgroundColor*/, staticObj[9]);
            direct_1.setStyle(t.n19, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n19, 1 /*height*/, staticObj[10]);
            direct_1.setStyle(t.n19, 19 /*marginBottom*/, staticObj[11]);
            direct_1.setStyle(t.n19, 77 /*maskImageSource*/, "linear-gradient(90deg, #000000, #ffffff 10%, #ffffff 90%, #000000)");
            direct_1.setStyle(t.n19, 29 /*justifyContent*/, 2);
            direct_1.setStyle(t.n19, 28 /*alignItems*/, 2);
            direct_1.setStyle(t.n19, 66 /*overflow*/, true);
            t.ctx21 = [undefined];
            t.n20 = direct_1.createWidget(list_vue_1.default, w, {
                attrs: {
                    scrollSnapAlign: "center",
                    ref: "list",
                    data: w.arr,
                    direction: 1,
                    showDistance: 0.2
                },
                style: {
                    width /*width*/: "80%",
                    height /*height*/: "200px",
                    overflow /*overflow*/: "visible"
                },
                slot: {
                    default: CreateSlot21
                },
                sCtx: {
                    default: t.ctx21
                },
                scope: w
            });
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n4);
            direct_1.destroyContext(t.n9);
            direct_1.destroyContext(t.n14);
            direct_1.destroyContext(t.n20);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n3, t.n2);
            direct_1.mountChildWidget(t.n4, t.n2);
            direct_1.append(t.n8, t.n2);
            direct_1.mountChildWidget(t.n9, t.n2);
            direct_1.append(t.n13, t.n2);
            direct_1.mountChildWidget(t.n14, t.n2);
            direct_1.append(t.n18, t.n2);
            direct_1.mountChildWidget(t.n20, t.n19);
            direct_1.append(t.n19, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            var _$wAttrs = {};
            if (dirty0 & 1) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.data = w.arr;
                _$wAttrs.attrs = _$attrs;
            }
            if (dirty0 & 3) _$wAttrs.scope = w;
            ;
            direct_1.patchAttrsForWidget(t.n4, _$wAttrs);
            var _$wAttrs = {};
            if (dirty0 & 1) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.data = w.arr;
                _$wAttrs.attrs = _$attrs;
            }
            if (dirty0 & 5) _$wAttrs.scope = w;
            ;
            direct_1.patchAttrsForWidget(t.n9, _$wAttrs);
            var _$wAttrs = {};
            if (dirty0 & 1) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.data = w.arr;
                _$wAttrs.attrs = _$attrs;
            }
            if (dirty0 & 9) _$wAttrs.scope = w;
            ;
            direct_1.patchAttrsForWidget(t.n14, _$wAttrs);
            var _$wAttrs = {};
            if (dirty0 & 1) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.data = w.arr;
                _$wAttrs.attrs = _$attrs;
            }
            if (dirty0 & 17) _$wAttrs.scope = w;
            ;
            direct_1.patchAttrsForWidget(t.n20, _$wAttrs);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class CreateSlot21 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        s22() {
            let w = this.w;
            let [v] = this._$ctx;
            return v % 2 > 0 ? B22 : B23;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            t.i22 = t.s22();
            t.n22 = direct_1.createIf(w, t.i22);
            return this.n22;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n22);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            this.parent = target;
            t.n22.m(target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            let { dirty0 } = w._$info;
            t.n22 = direct_1.patchIf(w, t.n22, t.i22, t.i22 = t.s22(), this.parent);
            return this.n22;
        }
        static getDirty({ v }) {
            return {
                v: v ? 4 : 0
            };
        }
        s({ v }) {
            let w = this.w;
            this._$ctx[0] = v;
        }
    }
    class B23 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            t.n23 = direct_1.createDiv();
            direct_1.setStyle(t.n23, 16 /*margin*/, staticObj[5]);
            direct_1.setStyle(t.n23, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n23, 0 /*width*/, staticObj[6]);
            direct_1.setStyle(t.n23, 46 /*backgroundColor*/, staticObj[8]);
            return this.n23;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n23, target, anchor);
        }
        p() {
            return this.n23;
        }
    }
    class B22 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            t.n22 = direct_1.createDiv();
            direct_1.setStyle(t.n22, 16 /*margin*/, staticObj[5]);
            direct_1.setStyle(t.n22, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n22, 0 /*width*/, staticObj[6]);
            direct_1.setStyle(t.n22, 46 /*backgroundColor*/, staticObj[7]);
            return this.n22;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n22, target, anchor);
        }
        p() {
            return this.n22;
        }
    }
    class CreateSlot15 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        s16() {
            let w = this.w;
            let [v] = this._$ctx;
            return v % 2 > 0 ? B16 : B17;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            t.i16 = t.s16();
            t.n16 = direct_1.createIf(w, t.i16);
            return this.n16;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n16);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            this.parent = target;
            t.n16.m(target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            let { dirty0 } = w._$info;
            t.n16 = direct_1.patchIf(w, t.n16, t.i16, t.i16 = t.s16(), this.parent);
            return this.n16;
        }
        static getDirty({ v }) {
            return {
                v: v ? 3 : 0
            };
        }
        s({ v }) {
            let w = this.w;
            this._$ctx[0] = v;
        }
    }
    class B17 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            t.n17 = direct_1.createDiv();
            direct_1.setStyle(t.n17, 16 /*margin*/, staticObj[5]);
            direct_1.setStyle(t.n17, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n17, 0 /*width*/, staticObj[6]);
            direct_1.setStyle(t.n17, 46 /*backgroundColor*/, staticObj[8]);
            return this.n17;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n17, target, anchor);
        }
        p() {
            return this.n17;
        }
    }
    class B16 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            t.n16 = direct_1.createDiv();
            direct_1.setStyle(t.n16, 16 /*margin*/, staticObj[5]);
            direct_1.setStyle(t.n16, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n16, 0 /*width*/, staticObj[6]);
            direct_1.setStyle(t.n16, 46 /*backgroundColor*/, staticObj[7]);
            return this.n16;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n16, target, anchor);
        }
        p() {
            return this.n16;
        }
    }
    class CreateSlot10 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        s11() {
            let w = this.w;
            let [v] = this._$ctx;
            return v % 2 > 0 ? B11 : B12;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            t.i11 = t.s11();
            t.n11 = direct_1.createIf(w, t.i11);
            return this.n11;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n11);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            this.parent = target;
            t.n11.m(target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            let { dirty0 } = w._$info;
            t.n11 = direct_1.patchIf(w, t.n11, t.i11, t.i11 = t.s11(), this.parent);
            return this.n11;
        }
        static getDirty({ v }) {
            return {
                v: v ? 2 : 0
            };
        }
        s({ v }) {
            let w = this.w;
            this._$ctx[0] = v;
        }
    }
    class B12 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            t.n12 = direct_1.createDiv();
            direct_1.setStyle(t.n12, 16 /*margin*/, staticObj[5]);
            direct_1.setStyle(t.n12, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n12, 0 /*width*/, staticObj[6]);
            direct_1.setStyle(t.n12, 46 /*backgroundColor*/, staticObj[8]);
            return this.n12;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n12, target, anchor);
        }
        p() {
            return this.n12;
        }
    }
    class B11 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            t.n11 = direct_1.createDiv();
            direct_1.setStyle(t.n11, 16 /*margin*/, staticObj[5]);
            direct_1.setStyle(t.n11, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n11, 0 /*width*/, staticObj[6]);
            direct_1.setStyle(t.n11, 46 /*backgroundColor*/, staticObj[7]);
            return this.n11;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.insertBefore(t.n11, target, anchor);
        }
        p() {
            return this.n11;
        }
    }
    class CreateSlot5 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        s6() {
            let w = this.w;
            let [v] = this._$ctx;
            return v % 2 > 0 ? B6 : B7;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            t.i6 = t.s6();
            t.n6 = direct_1.createIf(w, t.i6);
            return this.n6;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n6);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            this.parent = target;
            t.n6.m(target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            let { dirty0 } = w._$info;
            t.n6 = direct_1.patchIf(w, t.n6, t.i6, t.i6 = t.s6(), this.parent);
            return this.n6;
        }
        static getDirty({ v }) {
            return {
                v: v ? 1 : 0
            };
        }
        s({ v }) {
            let w = this.w;
            this._$ctx[0] = v;
        }
    }
    class B7 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            t.n7 = direct_1.createDiv();
            direct_1.setStyle(t.n7, 16 /*margin*/, staticObj[5]);
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[6]);
            direct_1.setStyle(t.n7, 46 /*backgroundColor*/, staticObj[8]);
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
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v] = t._$ctx;
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 16 /*margin*/, staticObj[5]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[6]);
            direct_1.setStyle(t.n6, 46 /*backgroundColor*/, staticObj[7]);
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
});